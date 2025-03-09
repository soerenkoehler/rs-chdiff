#[cfg(test)]
mod config_test;

use std::{
    error::Error, fs::File, io::BufReader, path::PathBuf
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    excludes: Vec<String>,
}

impl Config {
    pub fn from_file(file: PathBuf) -> Result<Config, Box<dyn Error>> {
        Ok(serde_json::from_reader(BufReader::new(File::open(file)?))?)
    }
}
