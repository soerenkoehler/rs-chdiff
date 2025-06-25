use glob::Pattern;
use std::{
    env,
    fs::OpenOptions,
    io::{BufReader, BufWriter, Error, ErrorKind},
    path::{Path, PathBuf},
};

use crate::filescanner::PatternList;

use super::{
    Config,
    def::{CONFIG_FILE, ENV_HOME},
};

impl Config {
    /// Create empty Config instance.
    pub fn new() -> Self {
        Self {
            exclude_absolute: PatternList::new(),
            exclude_relative: PatternList::new(),
        }
    }

    /// Load the given config file. If missing, a default config file is
    /// created.
    ///
    /// In every case the built-in relative exclude ".chdiff.txt" is added.
    ///
    pub fn from_file(file: &PathBuf) -> Result<Self, Error> {
        let mut config = match OpenOptions::new().read(true).open(file) {
            Ok(file) => match serde_json::from_reader(BufReader::new(file)) {
                Ok(cfg) => cfg,
                Err(err) => return Err(Error::other(err)),
            },
            Err(err) => match err.kind() {
                ErrorKind::NotFound => Self::create_default_config_file(file)?,
                _ => return Err(err),
            },
        };

        // add built-in excludes
        //
        // TODO replace unwrap() with error handling
        //
        // TODO replace string literal with constant, possibly even list of
        // patterns
        config
            .exclude_relative
            .push(Pattern::new(".chdiff.txt").unwrap());

        Ok(config)
    }

    /// Return the path to the users config file.
    //
    // TODO replace unwrap() with error handling
    pub fn get_config_path() -> PathBuf {
        Path::new(&env::var(ENV_HOME).unwrap()).join(CONFIG_FILE)
    }

    fn create_default_config_file(filepath: &PathBuf) -> Result<Self, Error> {
        let default = Self::new();
        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(filepath)
        {
            Ok(file) => match serde_json::to_writer(BufWriter::new(file), &default) {
                Ok(_) => {
                    println!("created default config file: {}", filepath.display());
                    Ok(default)
                }
                Err(err) => Err(Error::other(err)),
            },
            Err(err) => Err(err),
        }
    }
}
