use glob::Pattern;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, ErrorKind},
    path::PathBuf,
    str::FromStr,
};

use super::FileList;
use crate::patternlist::PatternList;

#[test]
fn bad_root_path() {
    match FileList::from_path(
        PathBuf::from_str("tests/filelist_data/non-existant").unwrap(),
        &PatternList::new(),
        &PatternList::new(),
    ) {
        Err(err) => assert_eq!(err.kind(), ErrorKind::NotFound),
        _ => panic!("should report non-rexistant root path"),
    };
}

#[test]
fn no_excludes() {
    assert_filelist(
        "tests/filelist_data/all_files.txt",
        "tests/filelist_data/data",
        &[],
        &[],
    );
}

#[test]
fn relative_specific_one_pattern() {
    assert_filelist(
        "tests/filelist_data/specific_one_pattern.txt",
        "tests/filelist_data/data",
        &[],
        &["file3.dat"],
    );
}

#[test]
fn relative_specific_two_patterns() {
    assert_filelist(
        "tests/filelist_data/specific_two_patterns.txt",
        "tests/filelist_data/data",
        &[],
        &["dir0/file2.dat", "dir1/file4.dat"],
    );
}

#[test]
fn relative_wildcard_one_pattern() {
    assert_filelist(
        "tests/filelist_data/wildcard_one_pattern.txt",
        "tests/filelist_data/data",
        &[],
        &["**/file3.dat"],
    );
}

#[test]
fn relative_wildcard_two_patterns() {
    assert_filelist(
        "tests/filelist_data/wildcard_two_patterns.txt",
        "tests/filelist_data/data",
        &[],
        &["**/dir0/file2.dat", "**/dir1/file4.dat"],
    );
}

#[test]
fn absolute_specific_one_pattern() {
    let pattern = std::env::current_dir()
        .unwrap()
        .join("tests/filelist_data/data/file3.dat");
    assert_filelist(
        "tests/filelist_data/specific_one_pattern.txt",
        "tests/filelist_data/data",
        &[pattern.to_str().unwrap()],
        &[],
    );
}

#[test]
fn absolute_specific_two_pattern() {
    let pattern1 = std::env::current_dir()
        .unwrap()
        .join("tests/filelist_data/data/dir0/file2.dat");
    let pattern2 = std::env::current_dir()
        .unwrap()
        .join("tests/filelist_data/data/dir1/file4.dat");
    assert_filelist(
        "tests/filelist_data/specific_two_patterns.txt",
        "tests/filelist_data/data",
        &[pattern1.to_str().unwrap(), pattern2.to_str().unwrap()],
        &[],
    );
}

#[test]
fn absolute_wildcard_one_pattern() {
    let pattern = std::env::current_dir()
        .unwrap()
        .join("tests/filelist_data/data/**/file3.dat");
    assert_filelist(
        "tests/filelist_data/wildcard_one_pattern.txt",
        "tests/filelist_data/data",
        &[pattern.to_str().unwrap()],
        &[],
    );
}

#[test]
fn absolute_wildcard_two_pattern() {
    let pattern1 = std::env::current_dir()
        .unwrap()
        .join("tests/filelist_data/data/**/dir0/file2.dat");
    let pattern2 = std::env::current_dir()
        .unwrap()
        .join("tests/filelist_data/data/**/dir1/file4.dat");
    assert_filelist(
        "tests/filelist_data/wildcard_two_patterns.txt",
        "tests/filelist_data/data",
        &[pattern1.to_str().unwrap(), pattern2.to_str().unwrap()],
        &[],
    );
}

fn assert_filelist(
    expect_file: &str,
    root_path: &str,
    exclude_absolute: &[&str],
    exclude_relative: &[&str],
) {
    let mut actual = FileList::from_path(
        PathBuf::from_str(root_path).unwrap(),
        &to_patternlist(exclude_absolute),
        &to_patternlist(exclude_relative),
    )
    .unwrap();
    actual.entries.sort();
    let mut actual = actual.entries.into_iter();

    let expect_file = OpenOptions::new().read(true).open(expect_file).unwrap();
    let mut expect = BufReader::new(expect_file).lines();

    while match (actual.next(), expect.next()) {
        (Some(a), Some(Ok(b))) => {
            assert_eq!(a.to_str().unwrap(), b.as_str());
            true
        }
        (Some(a), None) => panic!("unexpected path: {}", a.display()),
        (None, Some(Ok(b))) => panic!("failed expectation: {}", b),
        (_, Some(Err(e))) => panic!("can't read expectation: {:?}", e),
        _ => false,
    } {}
}

fn to_patternlist(patterns: &[&str]) -> PatternList {
    let mut result = PatternList::new();
    for pattern in patterns {
        result.push(Pattern::new(pattern).unwrap());
    }
    result
}
