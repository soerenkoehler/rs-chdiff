mod common;

use std::path::Path;

use common::{run_binary, run_in_dir};
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

// FIXME test stderr with integration test

// assert_config_error!(
//     missing_config_file,
//     std::io::Error,
//     "No such file or directory",
//     "tests/config_data/missing-file"
// );

// assert_config_error!(
//     invalid_1_empty_file,
//     serde_json::Error,
//     "EOF while parsing a value",
//     "tests/config_data/invalid-1.json"
// );

// assert_config_error!(
//     invalid_2_missing_excludes,
//     serde_json::Error,
//     "missing field `excludes`",
//     "tests/config_data/invalid-2.json"
// );

// assert_config_error!(
//     invalid_3_unexpected_attribute,
//     serde_json::Error,
//     "unknown field `other-attribute`",
//     "tests/config_data/invalid-3.json"
// );
