use std::{collections::HashMap, path::PathBuf, str::FromStr};

use super::Digest;

#[test]
fn new_digest() {
    let digest = Digest::new();
    assert_eq!(digest.entries, HashMap::from([]));
}

#[test]
fn load_file() {
    let digest = Digest::from_file(&get_path("generated/digest_test/sha256.txt"));
    assert_eq!(digest.entries[&get_path("data/empty.dat")], "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(digest.entries[&get_path("data/file1.dat")], "859be9ae6815dc73ff27c456db6f1d0e419b61459ed08c45fd2e0eeeca2d1266");
    assert_eq!(digest.entries[&get_path("data/file2.dat")], "41f9becae6c947181a50dc32c2840db8a904c5f9002841a5d809dd55b7240ac0");
}

fn get_path(file: &str) -> PathBuf {
    PathBuf::from_str(file).unwrap()
}
