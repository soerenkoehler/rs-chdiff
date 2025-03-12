use std::{path::PathBuf, str::FromStr};

use super::Config;

macro_rules! assert_config_error {
    ($n:ident,$t:ty,$m:expr,$f:expr) => {
        #[test]
        fn $n() {
            Config::from_file(PathBuf::from_str($f).unwrap())
                .unwrap_err()
                .downcast::<$t>()
                .unwrap()
                .to_string()
                .contains($m);
        }
    };
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

#[test]
fn valid_config_1() {
    let file = PathBuf::from_str("tests/config_data/valid-1.json").unwrap();
    let cfg = Config::from_file(&file);
    assert_eq!(cfg.excludes,Vec::<String>::new());
}
