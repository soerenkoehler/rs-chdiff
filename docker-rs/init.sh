#!/bin/bash

SCRIPTNAME=$(readlink -f $0)

rm -rf /app/work/* /app/output/*

pushd /app/work

find /app/input -mindepth 1 -maxdepth 1 \
    -not -name ".*" \
    -not -name "coverage" \
    -not -name "generated" \
    -not -name "rust-toolchain.toml" \
    -not -name "target" \
| xargs -I {SRC} cp -r {SRC} .

popd
