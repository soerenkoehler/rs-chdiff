use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use super::def::Digest;

impl Digest {
    pub fn new() -> Self {
        Digest {
            entries: HashMap::from([]),
        }
    }

    pub fn from_file(file: &PathBuf) -> Self {
        let mut digest = Self::new();

        BufReader::new(match OpenOptions::new().read(true).open(file) {
            Ok(file) => file,
            Err(err) => panic!("Can't open digest file: {}", err),
        })
        .lines()
        .into_iter()
        .filter_map(|line| match line {
            Ok(line) => Some(line),
            Err(err) => panic!("Can't read digest file: {}", err),
        })
        .for_each(|line| {
            // TODO split line
            digest.entries.insert(PathBuf::new(), line).unwrap();
        });

        digest
    }
}
