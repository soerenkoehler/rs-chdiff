#!/bin/bash

create() {
    create_dir generated/$1/data
    $1
    popd
}

create_dir() {
    mkdir -p $1
    pushd $1
}

create_file() {
    dd if=/dev/urandom of=$1 bs=1024 count=$2 status=none
}

filelist_test() {
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
    -not -path "./file3.dat"  \
    | sed 's/^\.\///' \
    | sort >../specific_one_pattern.txt

    find . -type f \
    -not -path "./dir0/file2.dat"  \
    -not -path "./dir1/file4.dat"  \
    | sed 's/^\.\///' \
    | sort >../specific_two_patterns.txt

    find . -type f \
    -not -path "**/file3.dat"  \
    | sed 's/^\.\///' \
    | sort >../wildcard_one_pattern.txt

    find . -type f \
    -not -path "**/dir0/file2.dat"  \
    -not -path "**/dir1/file4.dat"  \
    | sed 's/^\.\///' \
    | sort >../wildcard_two_patterns.txt
}

if [[ ! -e Cargo.toml && -e .git ]]; then
    printf "not in project root\n"
    exit -1
fi

create filelist_test
create digest_test
