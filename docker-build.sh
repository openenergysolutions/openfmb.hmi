#!/bin/sh
# build image
mkdir deps
cd deps
git clone https://github.com/openenergysolutions/openfmb-rs
cd ..
docker build -t openfmb.hmi .
