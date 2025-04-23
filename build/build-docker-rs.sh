#!/bin/bash

if [[ ! -e Cargo.toml || ! -e .git ]]; then
    printf "not in project root\n"
    exit -1
fi

docker build ./build/docker-rs -t rs-chdiff
