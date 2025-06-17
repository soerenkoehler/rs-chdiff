#!/bin/bash

SCRIPTNAME=$(readlink -f $0)

./init.sh

pushd /app/work

./build/generate-testdata.sh

COVERAGE_DIR=$(readlink -f "coverage")
PROFRAW_DIR="$COVERAGE_DIR/profraw"
PROFDATA_FILE="$COVERAGE_DIR/coverage.profdata"
REPORT_TEMP_FILE="$COVERAGE_DIR/coverage.lcov"
HTML_TEMP_DIR="$COVERAGE_DIR/html"

OUTPUT_DIR=/app/coverage
REPORT_FILE="$OUTPUT_DIR/coverage.lcov"

CRATE_NAME="rs-chdiff"
CRATE_NAME_FS_SAFE=$(echo "$CRATE_NAME" | tr '-' '_')

export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="$PROFRAW_DIR/$CRATE_NAME-%p-%m.profraw"

cargo clean
rm -rf "$OUTPUT_DIR"/*
rm -rf "$COVERAGE_DIR"
mkdir -p "$PROFRAW_DIR"
mkdir -p "$HTML_TEMP_DIR"

TEST_OUTPUT=$(cargo t --jobs 1 --message-format=json)

if [[ $? != 0 ]]; then
    printf "%s: tests have failed\n" "$SCRIPTNAME"
    exit -1
fi

OBJECTS=$( \
    jq -r -R "fromjson? | select(.profile.test == true) | .filenames[]" <<< $TEST_OUTPUT \
    | xargs -I {} printf "%s %s " "-object" {}; \
    find target/debug -type f \( -name "$CRATE_NAME*" -or -name "$CRATE_NAME_FS_SAFE*" \) -not -name "*.d" \
    | xargs -I {} printf "%s %s " "-object" {} \
)

llvm-profdata merge \
    -sparse "$PROFRAW_DIR"/* \
    -o "$PROFDATA_FILE"

llvm-cov export \
    --format=lcov \
    -path-equivalence=/app/work,. \
    --instr-profile="$PROFDATA_FILE" \
    --ignore-filename-regex='/.cargo' \
    --ignore-filename-regex='/.rustup/' \
    --ignore-filename-regex='/rustc/' \
    --ignore-filename-regex='/tests/' \
    --ignore-filename-regex='_test.rs$' \
    -Xdemangler=rustfilt \
    $OBJECTS \
    >"$REPORT_TEMP_FILE"

# 12:55:51.276 DEBUG Parsing LCOV report: /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov
# 12:55:51.278 WARN  Found 12 problems in LCOV report: /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov. More details in verbose mode
# 12:55:51.278 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:1: Invalid SF. File not found: /app/work/src/cli/def.rs
# 12:55:51.278 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:54: Invalid SF. File not found: /app/work/src/cli/parse.rs
# 12:55:51.278 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:84: Invalid SF. File not found: /app/work/src/commands/backup.rs
# 12:55:51.279 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:99: Invalid SF. File not found: /app/work/src/commands/create.rs
# 12:55:51.279 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:114: Invalid SF. File not found: /app/work/src/commands/mod.rs
# 12:55:51.279 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:299: Invalid SF. File not found: /app/work/src/commands/verify.rs
# 12:55:51.279 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:329: Invalid SF. File not found: /app/work/src/config/loader.rs
# 12:55:51.279 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:396: Invalid SF. File not found: /app/work/src/digest/def.rs
# 12:55:51.279 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:409: Invalid SF. File not found: /app/work/src/digest/file.rs
# 12:55:51.279 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:459: Invalid SF. File not found: /app/work/src/filescanner/file.rs
# 12:55:51.279 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:545: Invalid SF. File not found: /app/work/src/filescanner/pattern.rs
# 12:55:51.279 DEBUG /home/runner/work/rs-chdiff/rs-chdiff/coverage/coverage.lcov:639: Invalid SF. File not found: /app/work/src/main.rs
# 12:55:51.279 DEBUG Processed LCOV coverage reports

llvm-cov show \
    --format=html \
    --output-dir="$HTML_TEMP_DIR" \
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
    -Xdemangler=rustfilt \
    $OBJECTS

# copy HTML report
cp -r "$HTML_TEMP_DIR" "$OUTPUT_DIR"
# copy LCOV report and translate source file pathes
cat "$REPORT_TEMP_FILE" | sed 's/^SF:\/app\/work/SF:./' >"$REPORT_FILE"
# cp "$REPORT_FILE" "$OUTPUT_DIR"

popd
