FROM rust:latest AS build

WORKDIR /usr/src

COPY deps ./deps
COPY microgrid-protobuf ./microgrid-protobuf
COPY Server ./Server
COPY Server/Cargo-Docker.toml ./Server/Cargo.toml
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /usr/src/target/release/hmi_server /usr/local/bin/

WORKDIR /hmi_server

EXPOSE 80 8080 32771 42771

ENTRYPOINT ["hmi_server"]