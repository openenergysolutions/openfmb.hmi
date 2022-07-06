FROM rust:alpine3.16 as build

RUN apk update && apk add --no-cache \ 
    build-base \
    linux-headers \
    libressl-dev \
    protobuf-dev

COPY Server /openfmb.hmi/Server
COPY Cargo.toml /openfmb.hmi/Cargo.toml
COPY Cargo.lock /openfmb.hmi/Cargo.lock

WORKDIR /openfmb.hmi

RUN cargo build --release

FROM node:alpine3.16 AS build2

COPY Client ./Client

WORKDIR /Client
RUN yarn --version
RUN npx browserslist --update-db
RUN yarn install
RUN yarn run build

FROM alpine:3.16

COPY --from=build2 /Client/dist/openfmb-hmi /hmi_server/Client/dist/openfmb-hmi

COPY --from=build /openfmb.hmi/target/release/hmi_server /usr/local/bin/

WORKDIR /hmi_server

EXPOSE 80 8080 443 32000 32771 42771

ENTRYPOINT ["hmi_server"]