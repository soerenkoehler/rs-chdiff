use std::{path::PathBuf, str::FromStr};

use super::Config;

#[test]
fn valid_config_1() {
    let file = PathBuf::from_str("tests/config_data/valid-1.json").unwrap();
    let cfg = Config::from_file(&file);
    assert_eq!(cfg.exclude_absolute,Vec::<String>::new());
    assert_eq!(cfg.exclude_relative,Vec::<String>::new());
}
