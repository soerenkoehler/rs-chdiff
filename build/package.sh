#!/bin/bash

DISTDIR=./dist
BINARIES=$(find target \
    -type f \
    -path "*/release/*" \
    \( -name "rs-chdiff" -or -name "rs-chdiff.exe" \) )

for FILE in $BINARIES; do
    ARCH=$(basename $(dirname $(dirname $FILE)))
    ZIP="$DISTDIR/chdiff-$(date -I)-$ARCH.zip"
    zip -9j "$ZIP" "$FILE"
done
