#!/bin/bash

if [[ ! -e Cargo.toml ]]; then
    printf "not in project root\n"
    exit -1
fi

RETRY=3
while [[ $RETRY > 0 && -z $(docker images -a | grep rs-chdiff) ]]; do
    docker build \
        -t rs-chdiff \
        --build-arg USER_ID=$(id -u) \
        --build-arg GROUP_ID=$(id -g) \
        ./build/docker-rs
    RETRY=$(($RETRY-1))
done

docker images -aqf "dangling=true" | xargs -I {} docker rmi {}
