#!/bin/bash

# Git submodule upset and init
# git submodule update --init --recursive

# Install mkcert
curl -s https://api.github.com/repos/FiloSottile/mkcert/releases/latest| grep browser_download_url  | grep linux-amd64 | cut -d '"' -f 4 | wget -qi -
mv mkcert-v*-linux-amd64 mkcert
chmod a+x mkcert
sudo mv mkcert /usr/local/bin/

# Init Certs
export LOCAL_IP=$(ip -o route get to 8.8.8.8 | sed -n 's/.*src \([0-9.]\+\).*/\1/p')
mkdir certs
cd certs
mkdir server
cd server
mkcert -install
mkcert -cert-file server-cert.pem -key-file server-key.pem localhost 127.0.0.1 $LOCAL_IP ::1
mkcert -CAROOT
cd ..
cp -r ~/.local/share/mkcert/. .
cd ..

# Apt Packages
# sudo apt-get install autoconf automake libtool curl make g++ unzip cmake

# Project Yarn Dependencies
npm install -g @angular/cli
# npm install --prod=false
cd Client
yarn install
yarn run build
cd ..

echo "post-create-setup.sh done!"