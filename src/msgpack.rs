use async_std::channel::{bounded, RecvError, Sender};
use async_std::io::prelude::*;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_std::sync::{Arc, Mutex};
use async_std::task;
use futures::future::FutureExt;
use futures::{pin_mut, select};
use rmp_rpc::message::{Message, Notification, Request, Response};
use std::collections::HashMap;
use std::io::Cursor;

use crate::error::NetworkResult;

/// msgpack client used to interface with the airsim
/// msgpack server
pub struct MsgPackClient {
    request_sender: Sender<Request>,
    //request_receiver: Receiver<Request>,
    notification_sender: Sender<Notification>,
    //notificaiton_receiver: Receiver<Notification>,
    response_channels: Arc<Mutex<HashMap<u32, Sender<Response>>>>,
}

/// todo
enum Rpc {
    Send(Message),
    Receive(usize),
}

impl MsgPackClient {
    /// Establish a TCP socket connection to the `MessagePack-RPC` server
    /// running in a background thread
    pub async fn connect(addrs: impl ToSocketAddrs) -> NetworkResult<Self> {
        // Opens a TCP connection to a remote host
        let mut stream = TcpStream::connect(addrs).await.unwrap();

        // all response channels
        let response_channels = Arc::new(Mutex::new(HashMap::new()));

        // create multiple bounded channels with only 1 size message queue
        let (request_sender, request_receiver) = bounded::<Request>(1);
        let (inner_request_sender, _inner_request_receiver) = bounded::<Request>(1);
        let (notification_sender, notification_receiver) = bounded::<Notification>(1);
        let (inner_notification_sender, _inner_notification_receiver) = bounded::<Notification>(1);
        let res_channels = Arc::clone(&response_channels);

        task::spawn(async move {
            let mut current_message: Vec<u8> = vec![];

            // pre-allocate a vector buffer that can fit 1024 bytes
            let mut buf = vec![0_u8; 1024];

            let request_to_server = request_receiver.recv().fuse();
            let notification_to_server = notification_receiver.recv().fuse();
            // let stream_from_server = stream.read(&mut buf).fuse();

            pin_mut!(request_to_server, notification_to_server);

            // start loop that listens to incomning messages on three channels
            // using the select! future. The `to_process` variable will be set
            // when either of the three callbacks are invoked
            loop {
                let to_process = select! {
                    next = request_to_server => {
                        if let Ok(request) = next {
                            Some(Rpc::Send(Message::Request(request)))
                        } else {
                            None
                        }
                    },
                    next = notification_to_server => {
                        if let Ok(notification) = next {
                            Some(Rpc::Send(Message::Notification(notification)))
                        } else {
                            None
                        }
                    },
                    next = stream.read(&mut buf).fuse() => {
                        if let Ok(bytes_read) = next {
                            Some(Rpc::Receive(bytes_read))
                        } else {
                            None
                        }
                    },
                };

                match to_process {
                    // handler for outbound messages
                    Some(Rpc::Send(msg)) => {
                        // serialize message to bytes using msgpack
                        let message = msg.pack().expect("Couldn't serialize message");

                        // writes entire buffer to byte stream
                        stream.write_all(&message).await.expect("couldn't send message");
                    }
                    // Handler for inbound
                    Some(Rpc::Receive(n)) => {
                        //
                        current_message.extend(&buf[..n]);
                        let mut frame = Cursor::new(current_message.clone());
                        match Message::decode(&mut frame) {
                            Ok(Message::Notification(n)) => {
                                let _ = inner_notification_sender.send(n).await;
                            }
                            Ok(Message::Request(r)) => {
                                let _ = inner_request_sender.send(r).await;
                            }
                            Ok(Message::Response(r)) => {
                                let mut senders = res_channels.lock().await;
                                let sender: Sender<Response> =
                                    senders.remove(&r.id).expect("Got response but no request awaiting it");

                                // send response to the `request` function
                                let _ = sender.send(r).await;
                            }
                            Err(decode_err) => {
                                // TODO: let's figure something out!
                                panic!("{}", decode_err);
                            }
                        };
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
            response_channels,
        })
    }

    /// todo
    pub async fn request(&self, request: Request) -> Result<Response, RecvError> {
        let (response_sender, response_receiver) = bounded(1);

        // add the response sender (forwards the response from the server) by request id
        let _ = self.response_channels.lock().await.insert(request.id, response_sender);

        // forward request to the thread that then forwards it to the MessagePack-RPC server
        // the response is added to the response channel
        let _ = self.request_sender.send(request).await;

        // return result from request which is forwarded from the background thread above
        response_receiver.recv().await
    }

    /// todo
    pub async fn notify(&self, notification: Notification) -> Result<(), RecvError> {
        let _ = self.notification_sender.send(notification).await;
        Ok(())
    }
}
