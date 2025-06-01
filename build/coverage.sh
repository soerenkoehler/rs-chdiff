#!/bin/bash

if [[ ! -e Cargo.toml ]]; then
    printf "not in project root\n"
    exit -1
fi

mkdir -p coverage

if [[ -n $1 ]]; then
    CMD="coverage-$1.sh"
else
    CMD=""
fi

docker run \
  -p 8888:80 \
  --mount type=bind,src=.,dst=/app/input,ro \
  --mount type=bind,src=./coverage,dst=/app/output \
  --rm -it rs-chdiff:latest bash $CMD
