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

#[test]
fn valid_config_1() {
    let file = PathBuf::from_str("tests/config_data/valid-1.json").unwrap();
    let cfg = Config::from_file(&file);
    assert_eq!(cfg.excludes,Vec::<String>::new());
}
