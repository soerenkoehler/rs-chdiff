#!/bin/bash

DISTDIR=./dist
BINARIES=$(find target \
    -type f \
    -path "*/release/*" \
    \( -name "rs-chdiff" -or -name "rs-chdiff.exe" \) )

mkdir -p $DISTDIR
for SRC in $BINARIES; do
    ARCH=$(basename $(dirname $(dirname $SRC)))
    DST="$DISTDIR/chdiff-$(date -I)-$ARCH"
    case $ARCH in
    *windows*)
        DST -9j "$DST.zip" "$SRC"
        ;;
    *)
        tar -c "$SRC" | gzip -9 >"$DST.tar.gz"
        ;;
    esac
done
