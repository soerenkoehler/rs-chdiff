use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader, Error},
    path::{Path, PathBuf},
};

use super::def::{Digest, REGEX_DIGEST_LINE};

impl Digest {
    pub fn new() -> Self {
        Digest {
            entries: HashMap::from([]),
        }
    }

    pub fn from_file(file: &PathBuf) -> Result<Self, Error> {
        let mut digest = Self::new();
        let mut last_error = None;

        BufReader::new(OpenOptions::new().read(true).open(file)?)
            .lines()
            .for_each(|line| match Self::entry_from_line(line.unwrap()) {
                Ok((path, hash)) => {
                    digest.entries.insert(path, hash);
                }
                Err(err) => last_error = Some(err),
            });

        match last_error {
            Some(err) => Err(err),
            _ => Ok(digest),
        }
    }

    fn entry_from_line(line: String) -> Result<(PathBuf, String), Error> {
        match REGEX_DIGEST_LINE.captures(&line) {
            Some(captured) => {
                let (_, [hash, _, path]) = captured.extract();
                Ok((Path::new(path).to_path_buf(), hash.to_string()))
            }
            _ => Err(Error::other(format!("invalid digest line: {}", line))),
        }
    }
}
