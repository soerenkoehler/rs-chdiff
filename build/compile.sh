#!/bin/bash

if [[ ! -e Cargo.toml ]]; then
    printf "not in project root\n"
    exit -1
fi

mkdir -p target

docker run \
  --mount type=bind,src=.,dst=/app/input,ro \
  --mount type=bind,src=./target,dst=/app/target \
  --rm rs-chdiff:latest bash compile.sh
