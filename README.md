# OpenFMB (Open Field Message Bus) Human Machine Interface

Single line diagram and HMI for OpenFMB

## Project Setup

Make sure you have installed `node` and `npm` on your computer

## Check [node.js](https://nodejs.org/en/about/) version:

```bash
> node --version
```

## Check [npm](https://www.npmjs.com/) version:

```bash
> npm --version
```

## Install angular [cli tool](https://angular.io/cli):

```bash
> npm install -g @angular/cli
```

## Install [Rust and Cargo](https://www.rust-lang.org/learn/get-started)

```bash
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install [build toolchain](https://github.com/protocolbuffers/protobuf/blob/main/src/README.md)

[prost](https://github.com/tokio-rs/prost) now recommends `prost-build`  
[prost-build](https://github.com/tokio-rs/prost/tree/master/prost-build) needs `protoc` or a C++ toolchain.

For the C++ toolchain, the following tools are needed:

* autoconf
* automake
* libtool
* make
* g++
* unzip
* cmake (not mentioned on the above pages, but still required)

On Ubuntu/Debian, you can install them with:

```bash
> sudo apt-get install autoconf automake libtool curl make g++ unzip cmake
```

### Install [OpenSSL](https://www.openssl.org/)

On Ubuntu:

```bash
> apt-get install libssl-dev
```

On Fedora:

```bash
> apt-get install openssl-devel
```

## Build Client

From the project directory:

```bash
> cd Client
> yarn install
...
> yarn run build
...
> cd ..
```

### If running locally, edit the `config/app.toml` file

Set the IP address on the following two lines:

```toml
[openfmb_nats_subscriber]
dev_uri = "192.168.86.39:4222"

[hmi]
server_host = "192.168.86.39"
```

## Run application

From the project directory:

```bash
> cargo run
```
