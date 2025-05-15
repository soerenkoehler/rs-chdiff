#!/bin/bash

if [[ ! -e Cargo.toml ]]; then
    printf "not in project root\n"
    exit -1
fi

mkdir -p coverage

docker run \
  -p 8888:80 \
  --mount type=bind,src=.,dst=/app/input,ro \
  --mount type=bind,src=./coverage,dst=/app/output \
  --rm rs-chdiff:latest bash cover.sh
