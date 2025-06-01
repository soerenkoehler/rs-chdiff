#!/bin/bash

if [[ ! -e Cargo.toml ]]; then
    printf "not in project root\n"
    exit -1
fi

docker build \
       -t rs-chdiff \
       --build-arg USER_ID=$(id -u) \
       --build-arg GROUP_ID=$(id -g) \
       ./build/docker-rs

docker images -a

docker images -aqf "dangling=true" | xargs -I {} docker rmi {}

docker images -a
