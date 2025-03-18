use std::{path::PathBuf, str::FromStr};

use glob::Pattern;

use super::Config;
use crate::patternlist::PatternList;

#[test]
fn valid_config() {
    let file = PathBuf::from_str("tests/config_data/valid.json").unwrap();

    // catch missing test file because otherwise from_file() would silently
    // create a valid file
    assert!(file.exists(), "test file missing");

    let mut expect_relative = PatternList::new();
    expect_relative.push(Pattern::new(".chdiff.txt").unwrap());

    let cfg = Config::from_file(&file);
    assert_eq!(cfg.exclude_absolute, PatternList::new());
    assert_eq!(cfg.exclude_relative, expect_relative);
}
