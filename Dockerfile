FROM rust:latest AS build

ARG GITHUB_API_KEY

WORKDIR /usr/src

RUN git config --global --add url."https://${GITHUB_API_KEY}@github.com/".insteadOf "git@github.com:" && git config --global --add url."https://${GITHUB_API_KEY}@github.com/".insteadOf "https://github.com/"

COPY circuit_segment_manager ./circuit_segment_manager
COPY diagrams ./diagrams
COPY deps ./deps
COPY microgrid-protobuf ./microgrid-protobuf
COPY Server ./Server
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /usr/src/target/release/hmi_server /usr/local/bin/

WORKDIR /hmi_server

EXPOSE 32771 42771

ENTRYPOINT ["hmi_server"]