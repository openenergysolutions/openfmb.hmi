# SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
#
# SPDX-License-Identifier: Apache-2.0

FROM rust:1.86.0-alpine3.21 AS backend-build
WORKDIR /openfmb.hmi
COPY Cargo.toml ./
COPY Server/ ./Server
RUN apk update && apk add --no-cache \ 
    build-base \
    linux-headers \
    libressl-dev \
    protobuf-dev
RUN cargo build --release

FROM node:22.17.1-alpine3.21 AS frontend-build
WORKDIR /Client
COPY Client .
# RUN npx browserslist --update-db
RUN yarn --version
RUN yarn config set network-timeout 600000 -g
RUN yarn config set network-concurrency 2 -g
RUN yarn install
RUN yarn run build

FROM alpine:3.21 AS final
WORKDIR /hmi_server
COPY --from=frontend-build /Client/dist/openfmb-hmi /hmi_server/Client/dist/openfmb-hmi
COPY --from=backend-build /openfmb.hmi/target/release/hmi_server /usr/local/bin/

ENV APP_CONF=/config/app
ENV APP_DIR_NAME=/server
ENV CLIENT_DIST_DIR=/hmi_server/Client/dist/openfmb-hmi

ENTRYPOINT ["hmi_server"]