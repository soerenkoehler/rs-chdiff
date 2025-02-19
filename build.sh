#!/bin/bash

cargo build \
    --release \
    --target x86_64-pc-windows-gnu \
    --target x86_64-unknown-linux-gnu

BINARIES=$(find target \
    -type f \
    -path "*/release/*" \
    \( -name "rs-chdiff" -or -name "rs-chdiff.exe" \) )

for FILE in $BINARIES; do
    ARCH=$(basename $(dirname $(dirname $FILE)))
    ZIP="chdiff-$(date -I)-$ARCH.zip"
    zip -9j "$ZIP" "$FILE"
done
