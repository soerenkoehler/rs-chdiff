#!/bin/bash

SCRIPTNAME=$(readlink -f $0)

./init.sh

pushd /app/work

cargo build \
    --release \
    --target x86_64-pc-windows-gnu
    --target x86_64-unknown-linux-gnu

cp -r target/* /app/target

popd
