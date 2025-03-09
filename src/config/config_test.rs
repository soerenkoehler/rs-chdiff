use std::{error::Error, path::PathBuf, str::FromStr};

use super::Config;

#[test]
fn missing_config() {
    assert!(
        get_reading_error("tests/config_data/missing-file")
            .downcast::<std::io::Error>()
            .unwrap()
            .to_string()
            .contains("No such file or directory")
    );
}

#[test]
fn invalid_config_1() {
    assert!(
        get_reading_error("tests/config_data/config-invalid-1.json")
        .downcast::<serde_json::Error>()
        .unwrap()
        .to_string()
        .contains("EOF while parsing a value")
    );
}

#[test]
fn invalid_config_2() {
    assert!(
        get_reading_error("tests/config_data/config-invalid-2.json")
        .downcast::<serde_json::Error>()
        .unwrap()
        .to_string()
        .contains("missing field `excludes`")
    );
}

fn get_reading_error(file: &str) -> Box<dyn Error> {
    Config::from_file(PathBuf::from_str(file).unwrap()).unwrap_err()
}
