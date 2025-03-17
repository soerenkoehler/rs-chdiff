mod pattern_serializer;

#[cfg(test)]
mod config_test;

use std::{
    env,
    fs::OpenOptions,
    io::{BufReader, BufWriter, ErrorKind},
    path::{Path, PathBuf},
};

use glob::Pattern;
use serde::{Deserialize, Serialize};

use crate::digest::filelist::PatternList;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(rename = "exclude.absolute", with = "pattern_serializer")]
    pub exclude_absolute: PatternList,
    #[serde(rename = "exclude.relative", with = "pattern_serializer")]
    pub exclude_relative: PatternList,
}

#[cfg(unix)]
const ENV_HOME: &str = "HOME";

#[cfg(windows)]
const ENV_HOME: &str = "USERPROFILE";

const CONFIG_FILE: &str = ".chdiff-config.json";

impl Config {
    /// Create empty Config instance.
    pub fn new() -> Config {
        Config {
            exclude_absolute: vec![],
            exclude_relative: vec![],
        }
    }

    /// Load the given config file.
    ///
    /// Errors are printed to stderr and then the default config is returned.
    ///
    /// In every case the built-in relative exclude ".chdiff.txt" is added.
    ///
    pub fn from_file(file: &PathBuf) -> Config {
        let mut config = match OpenOptions::new().read(true).open(file) {
            Ok(file) => match serde_json::from_reader(BufReader::new(file)) {
                Ok(cfg) => Ok(cfg),
                Err(err) => Err(eprintln!("{err}")),
            },
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Ok(Self::create_default_config_file(file)),
                _ => Err(eprintln!("{err}")),
            },
        }
        .unwrap_or(Self::new());

        // add built-in excludes
        config.exclude_relative.push(Pattern::new(".chdiff.txt").unwrap());
        config
    }

    /// Return the path to the users config file.
    pub fn get_config_path() -> PathBuf {
        Path::new(&env::var(ENV_HOME).unwrap()).join(CONFIG_FILE)
    }

    fn create_default_config_file(filepath: &PathBuf) -> Config {
        let default = Self::new();
        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(filepath)
        {
            Ok(file) => {
                // if let Err(err) =
                match serde_json::to_writer(BufWriter::new(file), &default) {
                    Ok(_) => println!("created default config file: {}", filepath.display()),
                    Err(err) => eprintln!("{err}"),
                }
            }
            Err(err) => eprintln!("{err}"),
        }
        default
    }
}
