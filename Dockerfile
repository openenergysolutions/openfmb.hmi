FROM rust:1.52 AS build

COPY microgrid-protobuf /openfmb.hmi/microgrid-protobuf
COPY Server /openfmb.hmi/Server
COPY Cargo.toml /openfmb.hmi/Cargo.toml
COPY Cargo.lock /openfmb.hmi/Cargo.lock

WORKDIR /openfmb.hmi

RUN cargo build --release

FROM node:14.17.6 AS build2

COPY Client ./Client

WORKDIR /Client
RUN yarn install
RUN yarn run build

FROM debian:buster-slim

COPY --from=build2 /Client/dist/openfmb-hmi /hmi_server/Client/dist/openfmb-hmi

COPY --from=build /openfmb.hmi/target/release/hmi_server /usr/local/bin/

WORKDIR /hmi_server

EXPOSE 80 8080 443 32000 32771 42771

ENTRYPOINT ["hmi_server"]
