use std::{path::PathBuf, str::FromStr};

use super::Config;

#[test]
fn valid_config() {
    let file = PathBuf::from_str("tests/config_data/valid.json").unwrap();

    // catch missing test file because otherwise from_file() would silently
    // create a valid file
    assert!(file.exists(), "test file missing");

    let cfg = Config::from_file(&file);
    assert_eq!(cfg.exclude_absolute,Vec::<String>::new());
    assert_eq!(cfg.exclude_relative,vec![".chdiff.txt"]);
}
