use glob::Pattern;
use std::{path::PathBuf, str::FromStr};

use super::Config;
use crate::filescanner::PatternList;

#[test]
fn valid_config() {
    let file = PathBuf::from_str("tests/config_data/valid.json").unwrap();

    // catch missing test file because otherwise from_file() would silently
    // create a valid file
    assert!(file.exists(), "test file missing");

    let expect_absolute = PatternList::new();

    let mut expect_relative = PatternList::new();
    expect_relative.push(Pattern::new(".chdiff.txt").unwrap());

    let cfg = Config::from_file(&file).unwrap();
    assert_eq!(cfg.exclude_absolute, expect_absolute);
    assert_eq!(cfg.exclude_relative, expect_relative);
}

#[test]
fn valid_with_list_abs_config() {
    let file = PathBuf::from_str("tests/config_data/valid_with_list_abs.json").unwrap();

    // catch missing test file because otherwise from_file() would silently
    // create a valid file
    assert!(file.exists(), "test file missing");

    let mut expect_absolute = PatternList::new();
    expect_absolute.push(Pattern::new("**/*.txt").unwrap());

    let mut expect_relative = PatternList::new();
    expect_relative.push(Pattern::new(".chdiff.txt").unwrap());

    let cfg = Config::from_file(&file).unwrap();
    assert_eq!(cfg.exclude_absolute, expect_absolute);
    assert_eq!(cfg.exclude_relative, expect_relative);
}

#[test]
fn valid_with_list_rel_config() {
    let file = PathBuf::from_str("tests/config_data/valid_with_list_rel.json").unwrap();

    // catch missing test file because otherwise from_file() would silently
    // create a valid file
    assert!(file.exists(), "test file missing");

    let expect_absolute = PatternList::new();

    let mut expect_relative = PatternList::new();
    expect_relative.push(Pattern::new("**/*.txt").unwrap());
    expect_relative.push(Pattern::new(".chdiff.txt").unwrap());

    let cfg = Config::from_file(&file).unwrap();
    assert_eq!(cfg.exclude_absolute, expect_absolute);
    assert_eq!(cfg.exclude_relative, expect_relative);
}
