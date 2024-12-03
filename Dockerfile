# SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
#
# SPDX-License-Identifier: Apache-2.0

FROM rust:1.78-alpine3.18 AS backend-build
WORKDIR /openfmb.hmi
COPY Cargo.toml Cargo.lock ./
COPY Server/ ./Server
RUN apk update && apk add --no-cache \ 
    build-base \
    linux-headers \
    libressl-dev \
    protobuf-dev
RUN cargo build --release

FROM node:18.16.1-alpine3.18 AS frontend-build
WORKDIR /Client
COPY Client .
RUN npx browserslist --update-db
RUN yarn --version
RUN yarn install
RUN yarn run build

FROM alpine:3.18 AS final
WORKDIR /hmi_server
COPY --from=frontend-build /Client/dist/openfmb-hmi /hmi_server/Client/dist/openfmb-hmi
COPY --from=backend-build /openfmb.hmi/target/release/hmi_server /usr/local/bin/
ENTRYPOINT ["hmi_server"]