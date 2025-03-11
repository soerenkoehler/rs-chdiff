#[cfg(test)]
mod config_test;

use std::{
    env,
    error::Error,
    fs::File,
    io::{BufReader, ErrorKind},
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
    pub fn from_file(file: PathBuf) -> Result<Config, Box<dyn Error>> {
        match File::open(file) {
            Ok(file) => match serde_json::from_reader(BufReader::new(file)) {
                Ok(cfg) => Ok(cfg),
                Err(err) => {
                    println!("{:?}", err);
                    Err(Box::new(err))
                }
            },
            Err(err) => match err.downcast::<std::io::Error>() {
                Ok(err) => match err.kind() {
                    ErrorKind::NotFound => {
                        println!("file not found: {}", err.to_string());
                        Ok(Config { excludes: vec![] })
                    }
                    _ => {
                        println!("{:?}", err);
                        Err(Box::new(err))
                    }
                },
                Err(err) => {
                    println!("{:?}", err);
                    Err(Box::new(err))
                }
            },
        }
    }

    pub fn get_config_path() -> PathBuf {
        Path::new(&env::var(ENV_HOME).unwrap()).join(CONFIG_FILE)
    }
}

// TODO load from user home (win & linux)
// TODO create default file
// TODO add built-in excludes (".chdiff.txt")

// TODO detect missing config file
