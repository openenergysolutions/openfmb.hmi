# SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
#
# SPDX-License-Identifier: Apache-2.0

FROM rust:1.88-alpine3.21 AS backend-build
WORKDIR /openfmb.hmi
COPY Cargo.toml ./
COPY Server/ ./Server
RUN apk update && apk add --no-cache \ 
    build-base \
    linux-headers \
    libressl-dev \
    protobuf-dev
RUN cargo build --release

FROM node:18.20.7-alpine3.20 AS frontend-build
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

COPY --from=backend-build /openfmb.hmi/scripts/health /usr/local/bin/health
RUN chmod +x /usr/local/bin/health

COPY --from=frontend-build /Client/dist/openfmb-hmi /hmi_server/Client/dist/openfmb-hmi
COPY --from=backend-build /openfmb.hmi/target/release/hmi_server /usr/local/bin/
ENTRYPOINT ["hmi_server"]