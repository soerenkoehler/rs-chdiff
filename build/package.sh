#!/bin/bash

DISTDIR=$(readlink -f ./dist)
NAME_REPLACEMENT='s/rs-chdiff/chdiff/'

BINARIES=$(find ./target \
    -type f \
    -path "*/release/*" \
    \( -name "rs-chdiff" -or -name "rs-chdiff.exe" \) )

mkdir -p $DISTDIR

debug_perm() {
    OBJ=$(readlink -e "$1")
    ls -ald "$OBJ"
    PARENT=$(dirname "$OBJ")
    if [[ "$OBJ" != "$PARENT" ]]; then
        debug_perm "$PARENT"
    fi
}

for BIN in $BINARIES; do
    ARTIFACT=$(dirname $BIN)/$(sed $NAME_REPLACEMENT <<< $(basename $BIN))

    debug_perm $BIN
    debug_perm $ARTIFACT

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
        zip -v9jo "$DISTNAME.zip" "$ARTIFACT"
        ;;
    *)
        tar -cf "$DISTNAME.tar" \
            -C $(dirname "$ARTIFACT") \
            $(basename $ARTIFACT)
        gzip -v9f "$DISTNAME.tar"
        ;;
    esac

    printf "\n"
done

pushd ./coverage
zip -v9r "$DISTDIR/chdiff-$(date -I)-coverage.zip" \
    ./* \
    -x *.lcov \
    -x nginx*
popd
