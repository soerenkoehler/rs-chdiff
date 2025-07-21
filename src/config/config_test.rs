use glob::Pattern;
use predicates::str::{contains, starts_with};
use std::{
    env,
    fs::{self, Permissions},
    io::ErrorKind,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{
    CliErrorText,
    config::{
        Config,
        def::{CONFIG_FILE, ENV_HOME},
    },
    filescanner::pattern_test::to_patternlist,
    tests::runner::run_in_dir,
};

#[test]
fn cant_create_default_config_file() {
    fn set_readonly(path: &PathBuf, readonly: bool) {
        let mut permissions: Permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_readonly(readonly);
        fs::set_permissions(&path, permissions).unwrap();
    }

    let cwd = tempfile::tempdir().unwrap().into_path();

    // provoke error by making cwd readonly
    set_readonly(&cwd, true);

    run_in_dir(&cwd, &["v"])
        .failure()
        .stderr(starts_with(CliErrorText!(
            "error: Permission denied (os error 13) {}/.chdiff-config.json",
            cwd.display()
        )));

    // reset readonly flag on cwd
    set_readonly(&cwd, false);
}

#[test]
fn missing_config_file() {
    let cwd = tempfile::tempdir().unwrap().into_path();
    let expect = format!(
        "created default config file: {}",
        Path::join(&cwd, ".chdiff-config.json").to_str().unwrap()
    );
    run_in_dir(&cwd, &["v"]).success().stdout(contains(expect));
}

#[test]
fn get_config_path() {
    assert_eq!(
        Config::get_config_path(),
        Path::new(&env::var(ENV_HOME).unwrap())
            .to_path_buf()
            .join(CONFIG_FILE)
    );
}

#[test]
fn valid_config() {
    assert_valid_config("tests/config_data/valid.json", &[], &[]);
}

#[test]
fn valid_with_list_abs_config() {
    assert_valid_config(
        "tests/config_data/valid_with_list_abs.json",
        &["**/*.txt"],
        &[],
    );
}

#[test]
fn valid_with_list_rel_config() {
    assert_valid_config(
        "tests/config_data/valid_with_list_rel.json",
        &[],
        &["**/*.txt"],
    );
}

#[test]
fn file_exists_but_unreadable() {
    assert_fs_error(
        "generated/config_test_unreadable_file/data/unreadable.json",
        "Permission denied",
    );
}

#[test]
fn file_unwritable() {
    assert_fs_error(
        "generated/config_test_unwritable_file/data/unwritable.json",
        "Permission denied",
    );
}

#[test]
fn empty_file() {
    assert_error(
        "tests/config_data/empty_file.json",
        "EOF while parsing a value at line 1 column 0",
    );
}

#[test]
fn unexpected_attribute() {
    assert_error(
        "tests/config_data/unexpected_attribute.json",
        "unknown field `other.attribute`, expected `exclude.absolute` or `exclude.relative` at line 4 column 22",
    );
}

#[test]
fn missing_exclude_abs() {
    assert_error(
        "tests/config_data/missing_exclude_abs.json",
        "missing field `exclude.absolute` at line 3 column 1",
    );
}

#[test]
fn missing_exclude_rel() {
    assert_error(
        "tests/config_data/missing_exclude_rel.json",
        "missing field `exclude.relative` at line 3 column 1",
    );
}

#[test]
fn invalid_type_int_abs() {
    assert_error(
        "tests/config_data/invalid_type_int_abs.json",
        "invalid type: integer `0`, expected a sequence of valid glob patterns at line 2 column 26",
    );
}

#[test]
// for some reason serde reports the error on line 4
fn invalid_type_int_rel() {
    assert_error(
        "tests/config_data/invalid_type_int_rel.json",
        "invalid type: integer `0`, expected a sequence of valid glob patterns at line 4 column 0",
    );
}

#[test]
fn invalid_type_map_abs() {
    assert_error(
        "tests/config_data/invalid_type_map_abs.json",
        "invalid type: map, expected a sequence of valid glob patterns at line 2 column 25",
    );
}

#[test]
fn invalid_type_map_rel() {
    assert_error(
        "tests/config_data/invalid_type_map_rel.json",
        "invalid type: map, expected a sequence of valid glob patterns at line 3 column 25",
    );
}

#[test]
fn invalid_type_str_abs() {
    assert_error(
        "tests/config_data/invalid_type_str_abs.json",
        "invalid type: string \"string\", expected a sequence of valid glob patterns at line 2 column 32",
    );
}

#[test]
fn invalid_type_str_rel() {
    assert_error(
        "tests/config_data/invalid_type_str_rel.json",
        "invalid type: string \"string\", expected a sequence of valid glob patterns at line 3 column 32",
    );
}

#[test]
fn invalid_list_abs() {
    assert_error(
        "tests/config_data/invalid_list_abs.json",
        "expected value at line 2 column 26",
    );
}

#[test]
fn invalid_list_rel() {
    assert_error(
        "tests/config_data/invalid_list_rel.json",
        "expected value at line 4 column 1",
    );
}

#[test]
fn invalid_listentry_abs() {
    assert_error(
        "tests/config_data/invalid_listentry_abs.json",
        "Pattern syntax error near position 2: recursive wildcards must form a single path component at line 2 column 39",
    );
}

#[test]
fn invalid_listentry_rel() {
    assert_error(
        "tests/config_data/invalid_listentry_rel.json",
        "Pattern syntax error near position 2: recursive wildcards must form a single path component at line 3 column 39",
    );
}

fn assert_valid_config(file: &str, absolute: &[&str], relative: &[&str]) {
    let file = PathBuf::from_str(file).unwrap();

    // catch missing test file because otherwise from_file() would silently
    // create a valid file
    assert!(file.exists(), "test file missing");

    let expect_absolute = to_patternlist(absolute);
    let mut expect_relative = to_patternlist(relative);
    expect_relative.push(Pattern::new(".chdiff.txt").unwrap());

    let cfg = Config::from_file(&file).unwrap();
    assert_eq!(cfg.exclude_absolute, expect_absolute);
    assert_eq!(cfg.exclude_relative, expect_relative);
}

fn assert_fs_error(file: &str, expected: &str) {
    let err = Config::from_file(&PathBuf::from_str(file).unwrap()).unwrap_err();
    assert_ne!(err.kind(), ErrorKind::NotFound);
    assert!(err.to_string().contains(expected));
}

fn assert_error(file: &str, expected: &str) {
    let actual = Config::from_file(&PathBuf::from_str(file).unwrap())
        .unwrap_err()
        .to_string();
    assert_eq!(actual, expected);
}
