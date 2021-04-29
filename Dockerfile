FROM rust:latest AS build

WORKDIR /usr/src

COPY deps ./deps
COPY microgrid-protobuf ./microgrid-protobuf
COPY Server ./Server
COPY Server/Cargo-Docker.toml ./Server/Cargo.toml
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM node:15.14.0 AS build2

COPY Client ./Client

WORKDIR /Client
RUN yarn install
RUN yarn run build

FROM debian:buster-slim

COPY --from=build2 /Client/dist/openfmb-hmi /hmi_server/Client/dist/openfmb-hmi

COPY --from=build /usr/src/target/release/hmi_server /usr/local/bin/

WORKDIR /hmi_server

EXPOSE 80 8080 443 32771 42771

ENTRYPOINT ["hmi_server"]
