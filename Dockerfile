FROM rust:1.70.0-alpine3.17 as backend-build
WORKDIR /openfmb.hmi
COPY Cargo.toml Cargo.lock ./
COPY Server/ ./Server
RUN apk update && apk add --no-cache \ 
    build-base \
    linux-headers \
    libressl-dev \
    protobuf-dev
RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo build --release

FROM node:18.16.1-alpine3.17 AS frontend-build
WORKDIR /Client
COPY Client .
RUN apk --no-cache add git
RUN npx browserslist --update-db
RUN yarn --version
RUN yarn install
RUN yarn run build

FROM alpine:3.17 AS final
RUN apk update && apk add --no-cache \ 
    linux-headers \
    libressl-dev \
    protobuf-dev
WORKDIR /hmi_server
COPY --from=frontend-build /Client/dist/openfmb-hmi /hmi_server/Client/dist/openfmb-hmi
COPY --from=backend-build /openfmb.hmi/target/release/hmi_server /usr/local/bin/
ENTRYPOINT ["hmi_server"]