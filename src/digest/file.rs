use regex::Regex;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Error, ErrorKind},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::digest::def::Digest;

pub static REGEX_DIGEST_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([0-9a-fA-F]+)(\s\s|\s\*)(.+)$").unwrap());

impl Digest {
    pub fn from_file(file: &PathBuf) -> Result<Self, Error> {
        match OpenOptions::new().read(true).open(file) {
            Ok(input) => {
                let mut digest = Self::new();
                match BufReader::new(input)
                    .lines()
                    .filter_map(|raw_line| match Self::entry_from_line(raw_line) {
                        Ok((path, hash)) => digest.add(path, hash).err(),
                        Err(err) => Some(err),
                    })
                    .last()
                {
                    Some(err) => Err(err),
                    _ => Ok(digest),
                }
            }
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(Self::new()),
            Err(err) => Err(err),
        }
    }

    fn entry_from_line(raw_line: Result<String, Error>) -> Result<(PathBuf, String), Error> {
        let line = raw_line?;
        match REGEX_DIGEST_LINE.captures(&line) {
            Some(captured) => {
                let (_, [hash, _, path]) = captured.extract();
                Ok((Path::new(path).to_path_buf(), hash.to_string()))
            }
            _ => Err(Error::other(format!("invalid digest line: {}", line))),
        }
    }
}
