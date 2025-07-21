use std::{collections::HashMap, io::ErrorKind, path::PathBuf, str::FromStr};

use super::Digest;

macro_rules! test_error {
    ($name:ident,$input:expr,$kind:expr,$msg:expr) => {
        #[test]
        fn $name() {
            let err = Digest::from_file(&get_path($input)).unwrap_err();
            assert_eq!(err.kind(), $kind);
            assert_eq!(err.to_string(), $msg);
        }
    };
}

test_error!(
    bad_utf8,
    "tests/digest_data/bad_utf8.txt",
    ErrorKind::InvalidData,
    "stream did not contain valid UTF-8"
);

test_error!(
    invalid_hash,
    "tests/digest_data/hash_invalid.txt",
    ErrorKind::Other,
    "invalid digest line: x data/file.dat"
);

test_error!(
    mixed_hash_size,
    "tests/digest_data/hash_size_mixed.txt",
    ErrorKind::Other,
    "mixed hash sizes are not supported"
);

test_error!(
    wrong_hash_size,
    "tests/digest_data/hash_size_wrong.txt",
    ErrorKind::Other,
    "unknown hash type with length 4"
);

test_error!(
    hash_double_entry,
    "tests/digest_data/hash_double_entry.txt",
    ErrorKind::Other,
    "hash already defined for path data/file1.dat"
);

test_error!(
    file_not_readable,
    "tests/digest_data/non-existing.txt",
    ErrorKind::NotFound,
    "No such file or directory (os error 2)"
);

fn get_path(file: &str) -> PathBuf {
    PathBuf::from_str(file).unwrap()
}

macro_rules! test_hash {
    ($name:ident,$input:expr $(,$path:expr,$hash:expr)*) => {
        #[test]
        fn $name() {
            let digest = Digest::from_file(&get_path($input)).unwrap();
            $(assert_eq!(digest.entries[&get_path($path)], $hash);)*
        }
    };
}

test_hash!(
    load_sha256,
    "tests/digest_data/sha256.txt",
    "data/empty.dat",
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "data/file1.dat",
    "859be9ae6815dc73ff27c456db6f1d0e419b61459ed08c45fd2e0eeeca2d1266",
    "data/file2.dat",
    "41f9becae6c947181a50dc32c2840db8a904c5f9002841a5d809dd55b7240ac0"
);

test_hash!(
    load_sha512,
    "tests/digest_data/sha512.txt",
    "data/empty.dat",
    "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
    "data/file1.dat",
    "48690b2231dd962a5927f1da09c36a4bd7bf2a79dc92dbb6303f9c75e7b194e9670f5ca2e2282076e76a04e407639dbbfe0c0c56f9ebb9bc8578eab447da9266",
    "data/file2.dat",
    "a56f23a0272fd63541b137fd13902f7503dd7638a5ce971af88234064b49c8c1cddc221461a5ad3126e41367914388cc6b468164715d6f236ca570272f392767"
);

#[test]
fn new_digest() {
    let digest = Digest::new();
    assert_eq!(digest.entries, HashMap::from([]));
}
