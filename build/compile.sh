#!/bin/bash

cargo build \
    --release \
    --target x86_64-pc-windows-gnu \
    --target x86_64-unknown-linux-gnu
