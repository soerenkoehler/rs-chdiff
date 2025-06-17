mod common;

use predicates::str::contains;
use std::path::Path;

use common::{run_in_dir, run_with_config};

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
fn empty_file() {
    run_with_config("tests/config_data/empty_file.json", &["v"])
        .success()
        .stderr(contains("EOF while parsing a value"));
}

#[test]
fn unexpected_attribute() {
    run_with_config("tests/config_data/unexpected_attribute.json", &["v"])
        .success()
        .stderr(contains("unknown field `other.attribute`"));
}

#[test]
fn missing_exclude_abs() {
    run_with_config("tests/config_data/missing_exclude_abs.json", &["v"])
        .success()
        .stderr(contains("missing field `exclude.absolute`"));
}

#[test]
fn missing_exclude_rel() {
    run_with_config("tests/config_data/missing_exclude_rel.json", &["v"])
        .success()
        .stderr(contains("missing field `exclude.relative`"));
}

#[test]
fn invalid_type_int_abs() {
    run_with_config("tests/config_data/invalid_type_int_abs.json", &["v"])
        .success()
        .stderr(contains(
            "invalid type: integer `0`, expected a sequence of valid glob patterns at line 2",
        ));
}

#[test]
// for some reason serde reports the error on line 4
fn invalid_type_int_rel() {
    run_with_config("tests/config_data/invalid_type_int_rel.json", &["v"])
        .success()
        .stderr(contains(
            "invalid type: integer `0`, expected a sequence of valid glob patterns at line 4",
        ));
}

#[test]
fn invalid_type_map_abs() {
    run_with_config("tests/config_data/invalid_type_map_abs.json", &["v"])
        .success()
        .stderr(contains(
            "invalid type: map, expected a sequence of valid glob patterns at line 2",
        ));
}

#[test]
fn invalid_type_map_rel() {
    run_with_config("tests/config_data/invalid_type_map_rel.json", &["v"])
        .success()
        .stderr(contains(
            "invalid type: map, expected a sequence of valid glob patterns at line 3",
        ));
}

#[test]
fn invalid_type_str_abs() {
    run_with_config("tests/config_data/invalid_type_str_abs.json", &["v"])
        .success()
        .stderr(contains(
            "invalid type: string \"string\", expected a sequence of valid glob patterns at line 2",
        ));
}

#[test]
fn invalid_type_str_rel() {
    run_with_config("tests/config_data/invalid_type_str_rel.json", &["v"])
        .success()
        .stderr(contains(
            "invalid type: string \"string\", expected a sequence of valid glob patterns at line 3",
        ));
}

#[test]
fn invalid_list_abs() {
    run_with_config("tests/config_data/invalid_list_abs.json", &["v"])
        .success()
        .stderr(contains(
            "Reading config file: expected value at line 2 column 26",
        ));
}

#[test]
fn invalid_list_rel() {
    run_with_config("tests/config_data/invalid_list_rel.json", &["v"])
        .success()
        .stderr(contains(
            "Reading config file: expected value at line 4 column 1",
        ));
}

#[test]
fn invalid_listentry_abs() {
    run_with_config("tests/config_data/invalid_listentry_abs.json", &["v"])
        .success()
        .stderr(contains(
            "Reading config file: Pattern syntax error near position 2: recursive wildcards must form a single path component at line 2 column 39",
        ));
}

#[test]
fn invalid_listentry_rel() {
    run_with_config("tests/config_data/invalid_listentry_rel.json", &["v"])
        .success()
        .stderr(contains(
            "Reading config file: Pattern syntax error near position 2: recursive wildcards must form a single path component at line 3 column 39",
        ));
}
