#!/bin/bash

create_dir() {
    mkdir -p $1
    pushd $1
}

create_file() {
    dd if=/dev/urandom of=$1 bs=1024 count=$2 status=none
}

create() {
    create_dir $1/data
    $2
    popd
}

testcase_filetree() {
    for N in {0..15}; do
        mkdir -p dir$(($N/8%2))/dir$(($N/4%2))/dir$(($N/2%2))/dir$(($N%2))
    done

    for DIR in $(find . -type d); do
        for N in {1..5}; do
            create_file $DIR/file$N.dat 0
        done
    done

    find . -type f \
    | sed 's/^\.\///' \
    | sort >../all_files.txt

    find . -type f \
    -not -path "./dir0/file3.dat"  \
    | sed 's/^\.\///' \
    | sort >../exclude_specific_file.txt

    find . -type f \
    -not -path "**/file3.dat"  \
    | sed 's/^\.\///' \
    | sort >../exclude_many_files.txt
}

create . testcase_filetree
