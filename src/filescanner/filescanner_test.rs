use glob::Pattern;
use std::{
    fs::{DirEntry, OpenOptions, read_dir},
    io::{BufRead, BufReader, Error, ErrorKind},
    path::PathBuf,
    str::FromStr,
};

use super::{FileList, PatternList};

macro_rules! to_absolute_pattern {
    ($p:expr) => {
        std::env::current_dir()
            .unwrap()
            .canonicalize()
            .unwrap()
            .join("generated/filelist_test/data/")
            .join($p)
            .to_str()
            .unwrap()
    };
}

#[test]
fn non_existant_root_path() {
    match FileList::from_path(
        &PathBuf::from_str("generated/filelist_test/non-existant").unwrap(),
        &PatternList::new(),
        &PatternList::new(),
    ) {
        Err(err) => assert_eq!(err.kind(), ErrorKind::NotFound),
        _ => panic!("should report non-rexistant root path"),
    };
}

#[test]
fn result_to_option_ok() {
    let expected = read_dir(".").unwrap().next().unwrap().unwrap();
    let expected_path=expected.path();
    match FileList::result_to_option::<DirEntry>(Ok(expected)) {
        Some(entry) => assert_eq!(entry.path(), expected_path),
        None => panic!("should produce Some() value"),
    }
}

#[test]
fn result_to_option_err() {
    let expected = Error::new(ErrorKind::Other, "x");
    match FileList::result_to_option::<DirEntry>(Err(expected)) {
        Some(_) => panic!("should produce Err value"),
        None => (),
    };
}

#[test]
fn no_excludes() {
    assert_filelist("all_files.txt", &[], &[]);
}

#[test]
fn relative_specific_one_pattern() {
    assert_filelist("specific_one_pattern.txt", &[], &["file3.dat"]);
}

#[test]
fn relative_specific_two_patterns() {
    assert_filelist(
        "specific_two_patterns.txt",
        &[],
        &["dir0/file2.dat", "dir1/file4.dat"],
    );
}

#[test]
fn relative_wildcard_one_pattern() {
    assert_filelist("wildcard_one_pattern.txt", &[], &["**/file3.dat"]);
}

#[test]
fn relative_wildcard_two_patterns() {
    assert_filelist(
        "wildcard_two_patterns.txt",
        &[],
        &["**/dir0/file2.dat", "**/dir1/file4.dat"],
    );
}

#[test]
fn absolute_specific_one_pattern() {
    assert_filelist(
        "specific_one_pattern.txt",
        &[to_absolute_pattern!("file3.dat")],
        &[],
    );
}

#[test]
fn absolute_specific_two_pattern() {
    assert_filelist(
        "specific_two_patterns.txt",
        &[
            to_absolute_pattern!("dir0/file2.dat"),
            to_absolute_pattern!("dir1/file4.dat"),
        ],
        &[],
    );
}

#[test]
fn absolute_wildcard_one_pattern() {
    assert_filelist(
        "wildcard_one_pattern.txt",
        &[to_absolute_pattern!("**/file3.dat")],
        &[],
    );
}

#[test]
fn absolute_wildcard_two_pattern() {
    assert_filelist(
        "wildcard_two_patterns.txt",
        &[
            to_absolute_pattern!("**/dir0/file2.dat"),
            to_absolute_pattern!("**/dir1/file4.dat"),
        ],
        &[],
    );
}

fn assert_filelist(expect_file: &str, exclude_absolute: &[&str], exclude_relative: &[&str]) {
    let mut actual = FileList::from_path(
        &PathBuf::from_str("generated/filelist_test/data").unwrap(),
        &to_patternlist(exclude_absolute),
        &to_patternlist(exclude_relative),
    )
    .unwrap();
    actual.entries.sort();
    let mut actual = actual.entries.into_iter();

    let expect_file = OpenOptions::new()
        .read(true)
        .open(
            PathBuf::from_str("generated/filelist_test/")
                .unwrap()
                .join(expect_file),
        )
        .unwrap();
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
