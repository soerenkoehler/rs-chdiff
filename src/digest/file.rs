use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use super::def::{Digest, REGEX_DIGEST_LINE};

impl Digest {
    pub fn new() -> Self {
        Digest {
            entries: HashMap::from([]),
        }
    }

    pub fn from_file(file: &PathBuf) -> Self {
        Digest {
            entries: BufReader::new(match OpenOptions::new().read(true).open(file) {
                Ok(file) => file,
                Err(err) => panic!("can't open digest file: {}", err),
            })
            .lines()
            .into_iter()
            .filter_map(|line| match line {
                Ok(line) => Self::entry_from_line(line),
                Err(err) => panic!("can't read digest file: {}", err),
            })
            .collect(),
        }
    }

    fn entry_from_line(line: String) -> Option<(PathBuf, String)> {
        let Some(captures) = REGEX_DIGEST_LINE.captures(&line) else {
            eprintln!("invalid digest line: {}", line);
            return None;
        };
        let (_, [hash, _, path]) = captures.extract();
        Some((PathBuf::from_str(path).unwrap(), hash.to_string()))
    }
}
