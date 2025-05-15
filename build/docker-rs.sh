#!/bin/bash

if [[ ! -e Cargo.toml || ! -e .git ]]; then
    printf "not in project root\n"
    exit -1
fi

docker run -it \
  --mount type=bind,src=.,dst=/app/input,ro \
  --mount type=bind,src=./coverage,dst=/app/output \
  --rm rs-chdiff:latest bash
