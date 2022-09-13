# airsim-client
A Rust client library for Airsim.

## Pre-requisites to build project

```sh
$ sudo apt install build-essential
```

## Running with Docker

Install `docker-nvidia` first

then

```sh
$ make airsim-up
```

## WSL2 (client) and Windows 11 Server

https://docs.microsoft.com/en-us/windows/wsl/networking

Open Windows PowerShell in directory `Blocks\Blocks\WindowsNoEditor` and type:

```PowerShell
./Blocks.exe -ResX=640 -ResY=480 -windowed
```
