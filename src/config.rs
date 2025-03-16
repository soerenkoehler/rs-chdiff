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
    #[serde(rename = "exclude.absolute")]
    pub exclude_absolute: Vec<String>,
    #[serde(rename = "exclude.relative")]
    pub exclude_relative: Vec<String>,
}

#[cfg(unix)]
const ENV_HOME: &str = "HOME";

#[cfg(windows)]
const ENV_HOME: &str = "USERPROFILE";

const CONFIG_FILE: &str = ".chdiff-config.json";

impl Config {
    /// Return the path to the users config file.
    pub fn get_config_path() -> PathBuf {
        Path::new(&env::var(ENV_HOME).unwrap()).join(CONFIG_FILE)
    }

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
                ErrorKind::NotFound => Ok(Self::create_default_config(file)),
                _ => Err(eprintln!("{err}")),
            },
        }
        .unwrap_or(Self::new());

        // add built-in excludes
        config.exclude_relative.push(String::from(".chdiff.txt"));
        config
    }

    fn create_default_config(filepath: &PathBuf) -> Config {
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
