#[cfg(test)]
mod config_test;

use glob::Pattern;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::OpenOptions,
    io::{BufReader, BufWriter, ErrorKind},
    path::{Path, PathBuf},
};

use crate::filelist::PatternList;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(rename = "exclude.absolute")]
    pub exclude_absolute: PatternList,
    #[serde(rename = "exclude.relative")]
    pub exclude_relative: PatternList,
}

#[cfg(unix)]
const ENV_HOME: &str = "HOME";

#[cfg(windows)]
const ENV_HOME: &str = "USERPROFILE";

const CONFIG_FILE: &str = ".chdiff-config.json";

impl Config {
    /// Create empty Config instance.
    pub fn new() -> Self {
        Self {
            exclude_absolute: PatternList::new(),
            exclude_relative: PatternList::new(),
        }
    }

    /// Load the given config file.
    ///
    /// Errors are printed to stderr and then the default config is returned.
    ///
    /// In every case the built-in relative exclude ".chdiff.txt" is added.
    ///
    pub fn from_file(file: &PathBuf) -> Self {
        let mut config = match OpenOptions::new().read(true).open(file) {
            Ok(file) => match serde_json::from_reader(BufReader::new(file)) {
                Ok(cfg) => Ok(cfg),
                Err(err) => Err(eprintln!("Reading config file: {err}")),
            },
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(Self::create_default_config_file(file)),
                _ => Err(eprintln!("Reading config file: {err}")),
            },
        }
        .unwrap_or(Self::new());

        // add built-in excludes
        config
            .exclude_relative
            .push(Pattern::new(".chdiff.txt").unwrap());

        config
    }

    /// Return the path to the users config file.
    pub fn get_config_path() -> PathBuf {
        Path::new(&env::var(ENV_HOME).unwrap()).join(CONFIG_FILE)
    }

    fn create_default_config_file(filepath: &PathBuf) -> Self {
        let default = Self::new();
        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(filepath)
        {
            Ok(file) => match serde_json::to_writer(BufWriter::new(file), &default) {
                Ok(_) => println!("created default config file: {}", filepath.display()),
                Err(err) => eprintln!("{err}"),
            },
            Err(err) => eprintln!("{err}"),
        }
        default
    }
}
