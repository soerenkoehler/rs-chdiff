#!/bin/bash

LLVM_VERSION="-19"

if [[ ! -e Cargo.toml || ! -e .git ]]; then
    printf "not in project root\n"
    exit -1
fi

rm *.profdata *.profraw

cargo install rustfilt

RUSTFLAGS="-C instrument-coverage" cargo t

"llvm-profdata$LLVM_VERSION" merge \
    --sparse \
    *.profraw \
    -o rs-chdiff.profdata

"llvm-cov$LLVM_VERSION" show \
    -Xdemangler=rustfilt \
    --instr-profile=rs-chdiff.profdata \
    --ignore-filename-regex=/.cargo/registry \
    --ignore-filename-regex=/.rustup \
    target/debug/rs-chdiff
