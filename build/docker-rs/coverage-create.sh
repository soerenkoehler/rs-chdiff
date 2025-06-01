#!/bin/bash

SCRIPTNAME=$(readlink -f $0)

./coverage-init.sh

pushd /app/work

COVERAGE_DIR=$(readlink -f "coverage")
PROFRAW_DIR="$COVERAGE_DIR/profraw"
PROFDATA_FILE="$COVERAGE_DIR/coverage.profdata"
HTML_REPORT_DIR="$COVERAGE_DIR/html"

CRATE_NAME="rs-chdiff"
CRATE_NAME_FS_SAFE=$(echo "$CRATE_NAME" | tr '-' '_')

export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="$PROFRAW_DIR/$CRATE_NAME_FS_SAFE-%p-%m.profraw"

cargo clean
rm -rf "$COVERAGE_DIR"
mkdir -p "$PROFRAW_DIR"
mkdir -p "$HTML_REPORT_DIR"

TEST_OUTPUT=$(cargo t --jobs 1 --message-format=json)

if [[ $? != 0 ]]; then
    printf "%s: tests have failed\n" $SCRIPTNAME
    exit -1
fi

OBJECTS=$( \
    jq -r -R "fromjson? | select(.profile.test == true) | .filenames[]" <<< $TEST_OUTPUT \
    | xargs -I {} printf "%s %s " "-object" {}; \
    find target/debug -type f \( -name "$CRATE_NAME*" -or -name "$CRATE_NAME_FS_SAFE*" \) -not -name "*.d" \
    | xargs -I {} printf "%s %s " "-object" {} \
)

llvm-profdata-20 merge \
    -sparse "$PROFRAW_DIR"/* \
    -o "$PROFDATA_FILE"

llvm-cov-20 show \
    --format=html \
    --output-dir="$HTML_REPORT_DIR" \
    -Xdemangler=rustfilt \
    --show-instantiations=true \
    --show-mcdc=true \
    --show-regions=true \
    --show-line-counts=false \
    --show-line-counts-or-regions=false \
    --instr-profile="$PROFDATA_FILE" \
    --ignore-filename-regex='/.cargo' \
    --ignore-filename-regex='/.rustup/' \
    --ignore-filename-regex='/rustc/' \
    --ignore-filename-regex='/tests/' \
    --ignore-filename-regex='_test.rs$' \
    $OBJECTS

cp -r "$HTML_REPORT_DIR"/* /app/output

popd
