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

assert_config_error!(
    missing_config,
    std::io::Error,
    "No such file or directory",
    "tests/config_data/missing-file"
);

assert_config_error!(
    invalid_config_1,
    serde_json::Error,
    "EOF while parsing a value",
    "tests/config_data/config-invalid-1.json"
);

assert_config_error!(
    invalid_config_2,
    serde_json::Error,
    "missing field `excludes`",
    "tests/config_data/config-invalid-2.json"
);
