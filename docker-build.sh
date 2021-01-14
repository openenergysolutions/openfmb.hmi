#!/bin/sh
# build image
docker build -t openfmb.hmi.server:latest --build-arg GITHUB_API_KEY="${GITHUB_API_KEY}" .
