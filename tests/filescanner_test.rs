mod common;

use predicates::{ord::eq, str::starts_with};
use std::fs;

use common::run_binary;

#[test]
fn error_output_on_bad_root_dir() {
    let path = fs::canonicalize("./generated/filelist_test_baddir/data")
        .unwrap()
        .join("non-existant");
    let path = path.to_str().unwrap();
    let expected = format!("error accessing {}: No such file or directory", path);
    run_binary(&["v", path])
        .success()
        .stderr(starts_with(expected));
}

#[test]
fn error_output_on_bad_dir() {
    let path = fs::canonicalize("./generated/filelist_test_baddir/data").unwrap();
    let path = path.to_str().unwrap();
    let expected = format!("error accessing {}/dir-unreachable\n", path);
    run_binary(&["v", path]).success().stderr(eq(expected));
}

#[test]
fn error_output_on_bad_symlink() {
    let path = fs::canonicalize("./generated/filelist_test_badsymlink/data").unwrap();
    let path = path.to_str().unwrap();
    let expected = format!("neither file nor directory: {}/symlink-to-file1\n", path);
    run_binary(&["v", path]).success().stderr(eq(expected));
}
