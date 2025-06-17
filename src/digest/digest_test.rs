use std::{path::PathBuf, str::FromStr};

use super::Digest;

#[test]
fn load_file() {
    println!("{:?}", Digest::from_file(&PathBuf::from_str("generated/digest_test/sha256.txt").unwrap()));
}