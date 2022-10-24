#![allow(dead_code)]
use async_std::channel::{bounded, Receiver, Sender};
use async_std::io::prelude::*;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::sync::{Arc, Mutex};
use async_std::task;
use futures::future::FutureExt;
use futures::select;
use rmp_rpc::message::{Message, Notification, Request, Response};
use std::collections::HashMap;
use std::io::Cursor;

use crate::error::NetworkResult;
use crate::NetworkError;

/// msgpack client used to interface with the airsim msgpack server
#[derive(Clone, Debug)]
pub struct MsgPackClient {
    request_sender: Sender<Request>,
    notification_sender: Sender<Notification>,
    pub notification_receiver: Receiver<Notification>,
    pub request_receiver: Receiver<Request>,
    response_channels: Arc<Mutex<HashMap<u32, Sender<Response>>>>,
}

enum Rpc {
    Send(Message),
    Receive(usize),
}

impl MsgPackClient {
    /// Establish a TCP socket connection to the `MessagePack-RPC` server
    /// running in a background thread
    pub async fn connect(addrs: impl ToSocketAddrs) -> NetworkResult<Self> {
        let mut stream = TcpStream::connect(addrs).await?;
        let response_channels = Arc::new(Mutex::new(HashMap::new()));

        let (request_sender, request_receiver) = bounded::<Request>(1);
        let (inner_request_sender, inner_request_receiver) = bounded::<Request>(1);
        let (notification_sender, notification_receiver) = bounded::<Notification>(1);
        let (inner_notification_sender, inner_notification_receiver) = bounded::<Notification>(1);
        let res_channels = Arc::clone(&response_channels);

        task::spawn(async move {
            let mut current_message: Vec<u8> = vec![];

            // 1,024 bytes = 1 kB
            // 1kB x 1000 = 1mB
            let buf_size: usize = 1024 * 100; // 0.1mB
            let mut buf = vec![0_u8; buf_size];
            loop {
                let to_process = select! {
                    maybe_request = request_receiver.recv().fuse() => {
                        if let Ok(request) = maybe_request {
                            Some(Rpc::Send(Message::Request(request)))
                        } else {
                            None
                        }
                    },
                    maybe_notification = notification_receiver.recv().fuse() => {
                        if let Ok(notification) = maybe_notification {
                            Some(Rpc::Send(Message::Notification(notification)))
                        } else {
                            None
                        }
                    },
                    maybe_bytes_read = stream.read(&mut buf).fuse() => {
                        if let Ok(bytes_read) = maybe_bytes_read {
                            Some(Rpc::Receive(bytes_read))
                        } else {
                            None
                        }
                    }
                };
                match to_process {
                    Some(Rpc::Send(m)) => {
                        let message = m.pack().expect("Couldn't serialize message");
                        stream.write_all(&message).await.expect("Couldn't send message");
                    }
                    Some(Rpc::Receive(n)) => {
                        current_message.extend(&buf[..n]);
                        let mut frame = Cursor::new(current_message.clone());
                        let recv_res = match Message::decode(&mut frame) {
                            Ok(Message::Notification(n)) => inner_notification_sender
                                .send(n)
                                .await
                                .map_err(|e| NetworkError::Send { message: e.to_string() }),
                            Ok(Message::Request(r)) => inner_request_sender
                                .send(r)
                                .await
                                .map_err(|e| NetworkError::Send { message: e.to_string() }),
                            Ok(Message::Response(r)) => {
                                let mut senders = res_channels.lock().await;
                                let sender: Sender<Response> =
                                    senders.remove(&r.id).expect("Got response but no request awaiting it");

                                // send response to the `request` function
                                sender
                                    .send(r)
                                    .await
                                    .map_err(|e| NetworkError::Send { message: e.to_string() })
                            }
                            Err(e) => {
                                // DecodeError
                                panic!("{}", e);
                            }
                        };

                        // if error, return it
                        if let Err(e) = recv_res {
                            return e;
                        }

                        #[allow(clippy::cast_possible_truncation)]
                        {
                            let (_, remaining) = current_message.split_at(frame.position() as usize);
                            current_message = remaining.to_vec();
                        }
                    }
                    None => {}
                }
            }
        });
        Ok(Self {
            request_sender,
            notification_sender,
            notification_receiver: inner_notification_receiver,
            request_receiver: inner_request_receiver,
            response_channels,
        })
    }

    pub async fn request(&self, request: Request) -> Result<Response, NetworkError> {
        let (response_sender, response_receiver) = bounded(1);

        // add the response sender (forwards the response from the server) by request id
        let _ = self.response_channels.lock().await.insert(request.id, response_sender);

        // forward request to the thread that then forwards it to the MessagePack-RPC server
        // the response is added to the response channel
        let send_res = self.request_sender.send(request).await;
        if send_res.is_err() {
            let e = format!("Failed to send request: {:?}", send_res);
            return Err(NetworkError::Send { message: e });
        }

        // return result from request which is forwarded from the background thread above
        response_receiver.recv().await.map_err(NetworkError::Recv)
    }

    pub async fn _notify(&self, notification: Notification) -> Result<(), NetworkError> {
        let res = self.notification_sender.send(notification.to_owned()).await;
        if res.is_err() {
            let e = format!("Failed to send notification: {:?}", notification);
            return Err(NetworkError::Send { message: e });
        }
        Ok(())
    }
}
