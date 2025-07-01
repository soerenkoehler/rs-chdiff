use serde::{Deserialize, Serialize};

use crate::filescanner::PatternList;

#[cfg(unix)]
pub const ENV_HOME: &str = "HOME";

#[cfg(windows)]
pub const ENV_HOME: &str = "USERPROFILE";

pub const CONFIG_FILE: &str = ".chdiff-config.json";
pub const DIGEST_FILE: &str = ".chdiff.txt";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename = "exclude.absolute")]
    pub exclude_absolute: PatternList,
    #[serde(rename = "exclude.relative")]
    pub exclude_relative: PatternList,
}
