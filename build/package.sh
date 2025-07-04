#!/bin/bash

DISTDIR=./dist
NAME_REPLACEMENT='s/rs-chdiff/chdiff/'

BINARIES=$(find ./target \
    -type f \
    -path "*/release/*" \
    \( -name "rs-chdiff" -or -name "rs-chdiff.exe" \) )

mkdir -p $DISTDIR

for BIN in $BINARIES; do
    ARTIFACT=$(dirname $BIN)/$(sed $NAME_REPLACEMENT <<< $(basename $BIN))

    mv -v $BIN $ARTIFACT

    case $(basename $(dirname $(dirname $BIN))) in
    armv7*)
        ARCH=armV7
        ;;
    aarch64*)
        ARCH=arm64
        ;;
    x86_64-pc-windows-gnu)
        ARCH=win64
        ;;
    x86_64-unknown-linux-gnu)
        ARCH=linux
        ;;
    esac

    DISTNAME="$DISTDIR/chdiff-$(date -I)-$ARCH"

    case $ARCH in
    *windows*)
        zip -v9j "$DISTNAME.zip" "$ARTIFACT"
        ;;
    *)
        tar -cf "$DISTNAME.tar" \
            -C $(dirname "$ARTIFACT") $(basename $ARTIFACT)
        gzip -v9 "$DISTNAME.tar"
        ;;
    esac

    printf "\n"
done

zip -r9 "$DISTDIR/chdiff-$(date -I)-coverage.zip" ./coverage/*
