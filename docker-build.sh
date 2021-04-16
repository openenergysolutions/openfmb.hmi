#!/bin/sh
# build image
mkdir deps
cd deps
git clone -b hmi-develop https://github.com/openenergysolutions/openfmb-rs
cd ..
docker build -t openfmb.hmi .
