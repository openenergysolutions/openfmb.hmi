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
