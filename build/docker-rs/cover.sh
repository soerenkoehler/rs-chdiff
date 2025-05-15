#!/bin/bash

LLVM_VERSION="-19"

mkdir -p work

rm -r work/* output/*

find /app/input -mindepth 1 -maxdepth 1 \
    -not -name ".*" \
    -not -name "coverage" \
    -not -name "generated" \
    -not -name "rust-toolchain.toml" \
    -not -name "target" \
| xargs -I {SRC} cp -r {SRC} work/

pushd work

./build/generate_testdata.sh

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
    --ignore-filename-regex=/.cargo/ \
    --ignore-filename-regex=/.rustup/ \
    --ignore-filename-regex=/rustc/ \
    --ignore-filename-regex=/tests/ \
    --ignore-filename-regex=_test.rs$ \
    --format=html \
    --output-dir=coverage \
    $OBJECTS

cp -r --no-preserve=mode coverage/* /app/output
cp -r --no-preserve=mode coverage/* /var/www/html

popd

nginx -g "daemon off;"
