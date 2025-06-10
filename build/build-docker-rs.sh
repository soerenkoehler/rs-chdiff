#!/bin/bash

if [[ ! -e Cargo.toml ]]; then
    printf "not in project root\n"
    exit -1
fi

IMAGENAME=rs-chdiff
DOCKERDIR=./docker-rs
RETRY=3

# for local use: delete old image before rebuild
if [[ -n $(docker images -a | grep rs-chdiff) ]]; then
    docker rmi $IMAGENAME
fi

# build image; 3 retries for build in pipeline
while [[ $RETRY > 0 && -z $(docker images -a | grep $IMAGENAME) ]]; do
    tar -c rust-toolchain.toml -C $DOCKERDIR $(find $DOCKERDIR -type f -printf "%P ") \
    | docker build \
        --progress plain \
        -t $IMAGENAME \
        --build-arg USER_ID=$(id -u) \
        --build-arg GROUP_ID=$(id -g) \
        -
    RETRY=$(($RETRY-1))
done

# clean up images
docker images -aqf "dangling=true" | xargs -I {} docker rmi {}
