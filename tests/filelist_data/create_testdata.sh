#!/bin/bash

create_dir() {
    mkdir -p $1
    pushd $1
}

create_file() {
    dd if=/dev/urandom of=$1 bs=1024 count=1 status=none
}

create() {
    create_dir $1/data
    $2
    popd
}

testcase_regular_operation() {
    for N in {0..15}; do
        mkdir -p dir$(($N/8%2))/dir$(($N/4%2))/dir$(($N/2%2))/dir$(($N%2))
    done

    for DIR in $(find d* -type d); do
        for N in {1..5}; do
            create_file $DIR/file$N.dat
        done
    done

    find . -type f | sort | xargs -I {} sha256sum {} >../sha256.txt
    find . -type f | sort | xargs -I {} sha512sum {} >../sha512.txt
}

testcase_skipped_chdiff_txt() {
    create_file .chdiff.txt
    for N in {1..5}; do
        create_file file$N.dat
    done

    create_dir subdir

    create_file .chdiff.txt
    for N in {1..5}; do
        create_file file$N.dat
    done

    popd

    find . -type f -not -path "./.chdiff*" | sort | xargs -I {} sha256sum {} >../sha256.txt
}

create regular-operation  testcase_regular_operation
create skipped-chdiff-txt testcase_skipped_chdiff_txt
