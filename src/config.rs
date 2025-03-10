#[cfg(test)]
mod config_test;

use std::{
    env,
    error::Error,
    fs::File,
    io::BufReader,
    path::{self, Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    excludes: Vec<String>,
}

#[cfg(target_os = "linux")]
const ENV_HOME: &str = "HOME";

#[cfg(target_os = "windows")]
const ENV_HOME: &str = "USERPROFILE";

const CONFIG_FILE: &str = ".chdiff-config.json";

impl Config {
    pub fn from_file(file: PathBuf) -> Result<Config, Box<dyn Error>> {
        Ok(serde_json::from_reader(BufReader::new(File::open(file)?))?)
    }

    pub fn get_config_path() -> PathBuf {
        Path::new(&env::var(ENV_HOME).unwrap()).join(CONFIG_FILE)
    }

    pub fn init_config_file() {}
}

// TODO load from user home (win & linux)
// TODO create default file
// TODO add built-in excludes (".chdiff.txt")
