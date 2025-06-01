#!/bin/bash

if [[ ! -e Cargo.toml ]]; then
    printf "not in project root\n"
    exit -1
fi

mkdir -p coverage

docker run \
  --mount type=bind,src=.,dst=/app/input,ro \
  --mount type=bind,src=./coverage,dst=/app/work/coverage \
  --rm rs-chdiff:latest bash test.sh
