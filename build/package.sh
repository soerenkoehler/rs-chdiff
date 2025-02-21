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
        zip -9jv "$DST.zip" "$SRC"
        ;;
    *)
        tar -cvC $(dirname "$SRC") $(basename $SRC) | gzip -9v >"$DST.tar.gz"
        ;;
    esac
done
