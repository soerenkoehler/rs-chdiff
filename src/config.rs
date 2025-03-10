#[cfg(test)]
mod config_test;

use std::{
    error::Error, fs::File, io::BufReader, path::PathBuf
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    excludes: Vec<String>,
}

impl Config {
    pub fn from_file(file: PathBuf) -> Result<Config, Box<dyn Error>> {
        Ok(serde_json::from_reader(BufReader::new(File::open(file)?))?)
    }
}

// TODO load from user home (win & linux)
// TODO create default file
// TODO add built-in excludes (".chdiff.txt")