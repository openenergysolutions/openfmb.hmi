FROM rust:latest AS build

WORKDIR /usr/src

COPY deps ./deps
COPY microgrid-protobuf ./microgrid-protobuf
COPY Server ./Server
COPY Server/Cargo-Docker.toml ./Server/Cargo.toml
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM node:latest AS build2

COPY Client ./Client

WORKDIR /Client
RUN npm install
RUN npm run build

FROM debian:buster-slim
RUN apt-get -y update
RUN apt-get -y install nginx

COPY /Client/nginx-custom.conf /etc/nginx/conf.d/default.conf
COPY --from=build2 /Client/dist/openfmb-hmi /usr/share/nginx/html

COPY --from=build /usr/src/target/release/hmi_server /usr/local/bin/

WORKDIR /hmi_server

EXPOSE 80 8080 443 32771 42771

#ENTRYPOINT ["hmi_server"]
CMD ["/bin/sh",  "-c",  " exec nginx -g 'daemon off;' & hmi_server" ]