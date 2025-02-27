#!/bin/bash

DISTDIR=./dist
NAME_REPLACEMENT='s/rs-chdiff/chdiff/'

BINARIES=$(find target \
    -type f \
    -path "*/release/*" \
    \( -name "rs-chdiff" -or -name "rs-chdiff.exe" \) )

mkdir -p $DISTDIR

for BIN in $BINARIES; do
    ARCH=$(basename $(dirname $(dirname $BIN)))
    SRC=$(dirname $BIN)/$(sed $NAME_REPLACEMENT <<< $(basename $BIN))
    DST="$DISTDIR/chdiff-$(date -I)-$ARCH"

    mv -v $BIN $SRC

    case $ARCH in
    *windows*)
        zip -v9j "$DST.zip" "$SRC"
        ;;
    *)
        tar -cf "$DST.tar" \
            -C $(dirname "$SRC") $(basename $SRC)
        gzip -v9 "$DST.tar"
        ;;
    esac

    printf "\n"
done
