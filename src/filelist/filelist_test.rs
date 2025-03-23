use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use crate::patternlist::PatternList;

use super::FileList;

#[test]
fn relative_excludes() {
    // FIXME invalid root path is reported directly in stderr => write
    // integration test
    assert_filelist(
        FileList::from_path(
            PathBuf::from_str("tests/filelist_data/data").unwrap(),
            &PatternList::new(),
            &PatternList::new(),
        )
        .unwrap(),
        "tests/filelist_data/all_files.txt",
    );
}

fn assert_filelist(mut actual: FileList, expect_file: &str) {
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
