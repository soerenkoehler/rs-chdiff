#!/bin/bash

LLVM_VERSION="-20"
export RUSTFLAGS="-C instrument-coverage"

mkdir -p work

rm -rf work/* output/*

find /app/input -mindepth 1 -maxdepth 1 \
    -not -name ".*" \
    -not -name "coverage" \
    -not -name "generated" \
    -not -name "rust-toolchain.toml" \
    -not -name "target" \
| xargs -I {SRC} cp -r {SRC} work/

pushd work

./build/generate-testdata.sh

mkdir -p coverage

# OBJECTS=$( \
#     cargo t --jobs 1 --message-format=json error_output_on_bad_symlink \
#     | jq -r -R "fromjson? | select(.profile.test == true) | .filenames[]" \
#     | xargs -I {} printf "%s %s " "--object" {} \
# )

# printf "\n%s\n" "$OBJECTS"

# "llvm-profdata$LLVM_VERSION" merge \
#     --sparse \
#     *.profraw \
#     -o rs-chdiff.profdata

# "llvm-cov$LLVM_VERSION" show \
#     -Xdemangler=rustfilt \
#     --instr-profile=rs-chdiff.profdata \
#     --ignore-filename-regex=/.cargo/ \
#     --ignore-filename-regex=/.rustup/ \
#     --ignore-filename-regex=/rustc/ \
#     --ignore-filename-regex=/tests/ \
#     --ignore-filename-regex=_test.rs$ \
#     --format=html \
#     --output-dir=coverage \
#     $OBJECTS

# cp -r coverage/* /app/output

popd