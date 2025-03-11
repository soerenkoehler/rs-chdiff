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
                Err(err) => Err(Box::new(err)),
            },
            Err(err) => {
                match err.downcast::<std::io::Error>() {
                    Ok( e) => match e.kind() {
                        ErrorKind::NotFound => println!("file not found: {}", e.to_string()),
                        _ => println!("{:?}", e),
                    },
                    Err(e) => println!("{:?}", e),
                }
                Err(Box::new(err))
            }
        }
        // Ok(Config { excludes: vec![] })
        //         Ok()
        // if let Err(e) = Config::from_file(Config::get_config_path()) {
        // match e.downcast::<Error>() {
        //     Ok(ref e) => match e.kind() {
        //         ErrorKind::NotFound => println!("file not found: {}", e.to_string()),
        //         _ => println!("{:?}", e),
        //     },
        //     Err(e) => println!("{:?}", e),
        // }
        // };
    }

    pub fn get_config_path() -> PathBuf {
        Path::new(&env::var(ENV_HOME).unwrap()).join(CONFIG_FILE)
    }
}

// TODO load from user home (win & linux)
// TODO create default file
// TODO add built-in excludes (".chdiff.txt")

// TODO detect missing config file
