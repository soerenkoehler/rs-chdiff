#[cfg(test)]
mod config_test;

use std::{
    env,
    fs::OpenOptions,
    io::{BufReader, BufWriter, ErrorKind},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    excludes: Vec<String>,
}

#[cfg(unix)]
const ENV_HOME: &str = "HOME";

#[cfg(windows)]
const ENV_HOME: &str = "USERPROFILE";

const CONFIG_FILE: &str = ".chdiff-config.json";

impl Config {
    pub fn from_file(file: &PathBuf) -> Config {
        match OpenOptions::new().read(true).open(file) {
            Ok(file) => match serde_json::from_reader(BufReader::new(file)) {
                Ok(cfg) => Ok(cfg),
                Err(err) => Err(eprintln!("{err}")),
            },
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(Self::create_default_config(file)),
                _ => Err(eprintln!("{err}")),
            },
        }
        .unwrap_or(Config { excludes: vec![] })
    }

    pub fn get_config_path() -> PathBuf {
        Path::new(&env::var(ENV_HOME).unwrap()).join(CONFIG_FILE)
    }

    fn create_default_config(file: &PathBuf) -> Config {
        let default = Config { excludes: vec![] };
        match OpenOptions::new().create_new(true).write(true).open(file) {
            Ok(file) => {
                if let Err(err) = serde_json::to_writer(BufWriter::new(file), &default) {
                    eprintln!("{err}")
                }
            }
            Err(err) => eprintln!("{err}"),
        }
        default
    }
}

// TODO load from user home (win & linux)
// TODO add built-in excludes (".chdiff.txt")
