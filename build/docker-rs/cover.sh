#!/bin/bash

LLVM_VERSION="-19"

mkdir -p work

rm work/* output/*

find . -maxdepth 1 -type d \
    -not -name ".*" \
    -not -name "coverage" \
    -not -name "generated" \
    -not -name "target" \
    -printf "%f\n" \
| xargs -I {DIR} cp -r input/{DIR} work

OBJECTS=$( \
    RUSTFLAGS="-C instrument-coverage" \
    cargo t --jobs 1 --message-format=json \
    | jq -r -R "fromjson? | select(.profile.test == true) | .filenames[]" \
    | xargs -I {} printf "%s %s " "--object" {} \
)

"llvm-profdata$LLVM_VERSION" merge \
    --sparse \
    *.profraw \
    -o rs-chdiff.profdata

"llvm-cov$LLVM_VERSION" show \
    -Xdemangler=rustfilt \
    --instr-profile=rs-chdiff.profdata \
    --ignore-filename-regex=/.cargo/registry \
    --ignore-filename-regex=/.rustup \
    --ignore-filename-regex=/rustc \
    --ignore-filename-regex=/tests/ \
    --ignore-filename-regex=_test.rs$ \
    --format=html \
    -o ../output \
    $OBJECTS
