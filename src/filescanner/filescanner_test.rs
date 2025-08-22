use predicates::str::starts_with;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, ErrorKind},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    CliErrorText,
    filescanner::{FileList, PatternList, pattern_test::to_patternlist},
    tests::runner::run_binary,
};

macro_rules! abs_path {
    ($p:expr) => {
        PathBuf::from_str($p).unwrap().canonicalize().unwrap()
    };
}

macro_rules! abs_pattern {
    ($p:expr) => {
        abs_path!("generated/filelist_test/data/")
            .join($p)
            .to_str()
            .unwrap()
    };
}

#[test]
fn error_output_on_bad_root_dir() {
    let path = abs_path!("generated/filelist_test_baddir/data").join("non-existant");
    let path = path.to_str().unwrap();
    let expected = CliErrorText!("error: No such file or directory (os error 2) {}", path);
    run_binary(&["v", path])
        .failure()
        .stderr(starts_with(expected));
}

#[test]
fn error_output_on_bad_dir() {
    let path = abs_path!("generated/filelist_test_baddir/data");
    let path = path.to_str().unwrap();
    let expected = format!("Permission denied (os error 13) {}/dir-unreachable", path);
    run_binary(&["v", path])
        .success()
        .stderr(starts_with(expected));
}

#[test]
fn error_output_on_bad_symlink() {
    let path = abs_path!("generated/filelist_test_badsymlink/data");
    let path = path.to_str().unwrap();
    let expected = format!("neither file nor directory: {}/symlink-to-file1", path);
    run_binary(&["v", path])
        .success()
        .stderr(starts_with(expected));
}

#[test]
fn non_existant_root_path() {
    match FileList::from_path(
        &PathBuf::from_str("generated/filelist_test/non-existant").unwrap(),
        &PatternList::new(),
        &PatternList::new(),
    ) {
        Err(err) => assert_eq!(err.kind(), ErrorKind::Other),
        _ => panic!("should report non-rexistant root path"),
    };
}

#[test]
fn errors_on_bad_symlink() {
    let path = PathBuf::from_str("generated/filelist_test_badsymlink/data").unwrap();
    let file_list = FileList::from_path(&path, &to_patternlist(&[]), &to_patternlist(&[])).unwrap();

    let actual = file_list.errors.get(0).unwrap();
    let expected = format!(
        "neither file nor directory: {}/symlink-to-file1",
        path.to_str().unwrap()
    );

    assert_eq!(actual.kind(), ErrorKind::Other);
    assert_eq!(actual.to_string(), expected);
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
        &[abs_pattern!("file3.dat")],
        &[],
    );
}

#[test]
fn absolute_specific_two_pattern() {
    assert_filelist(
        "specific_two_patterns.txt",
        &[
            abs_pattern!("dir0/file2.dat"),
            abs_pattern!("dir1/file4.dat"),
        ],
        &[],
    );
}

#[test]
fn absolute_wildcard_one_pattern() {
    assert_filelist(
        "wildcard_one_pattern.txt",
        &[abs_pattern!("**/file3.dat")],
        &[],
    );
}

#[test]
fn absolute_wildcard_two_pattern() {
    assert_filelist(
        "wildcard_two_patterns.txt",
        &[
            abs_pattern!("**/dir0/file2.dat"),
            abs_pattern!("**/dir1/file4.dat"),
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

    let mut expect = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(
                PathBuf::from_str("generated/filelist_test/")
                    .unwrap()
                    .join(expect_file),
            )
            .unwrap(),
    )
    .lines();

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
