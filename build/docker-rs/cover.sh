#!/bin/bash

LLVM_VERSION="-19"

# if [[ ! -e Cargo.toml || ! -e .git ]]; then
#     printf "not in project root\n"
#     exit -1
# fi

# rm *.profdata *.profraw

# cargo install rustfilt

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
    --ignore-filename-regex=/tests/ \
    --ignore-filename-regex=_test.rs$ \
    --format=html \
    -o coverage \
    $OBJECTS

# rm *.profdata *.profraw
