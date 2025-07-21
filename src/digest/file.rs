use regex::Regex;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Error},
    path::{Path, PathBuf},
    sync::LazyLock,
};

use crate::digest::def::Digest;

pub static REGEX_DIGEST_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([0-9abcdefABCDEF]+)(\s\s|\s\*)(.+)$").unwrap());

impl Digest {
    pub fn from_file(file: &PathBuf) -> Result<Self, Error> {
        let mut digest = Self::new();
        let mut error = None;

        BufReader::new(OpenOptions::new().read(true).open(file)?)
            .lines()
            .for_each(|line| {
                if let Some(err) = match line {
                    Ok(line) => match Self::entry_from_line(line) {
                        Ok((path, hash)) => digest.add(path, hash).err(),
                        Err(err) => Some(err),
                    },
                    Err(err) => Some(err),
                } {
                    error.get_or_insert(err);
                };
            });

        match error {
            // TODO Is it possible to wrap errors?
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
