mod common;

use std::path::Path;

use common::{run_in_dir, run_with_config};
use predicates::str::contains;

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
fn invalid_1_empty_file() {
    run_with_config("tests/config_data/invalid-1.json", &["v"])
        .success()
        .stderr(contains("EOF while parsing a value"));
}

#[test]
fn invalid_2_missing_excludes() {
    run_with_config("tests/config_data/invalid-2.json", &["v"])
        .success()
        .stderr(contains("missing field `excludes`"));
}

#[test]
fn invalid_3_unexpected_attribute() {
    run_with_config("tests/config_data/invalid-3.json", &["v"])
        .success()
        .stderr(contains("unknown field `other-attribute`"));
}
