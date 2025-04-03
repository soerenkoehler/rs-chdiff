use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use regex::Regex;

use super::def::{Digest, REGEX_DIGEST_LINE};

impl Digest {
    pub fn new() -> Self {
        Digest {
            entries: HashMap::from([]),
        }
    }

    pub fn from_file(file: &PathBuf) -> Self {
        let re_digest_line = Regex::new(REGEX_DIGEST_LINE).unwrap();

        Digest {
            entries: BufReader::new(match OpenOptions::new().read(true).open(file) {
                Ok(file) => file,
                Err(err) => panic!("can't open digest file: {}", err),
            })
            .lines()
            .into_iter()
            .filter_map(|line| match line {
                Ok(line) => Self::digest_entry(line, &re_digest_line),
                Err(err) => panic!("can't read digest file: {}", err),
            })
            .collect(),
        }
    }

    fn digest_entry(line: String, re: &Regex) -> Option<(PathBuf, String)> {
        let Some(captures) = re.captures(&line) else {
            eprintln!("invalid digest line: {}", line);
            return None;
        };
        let (_, [hash, _, path]) = captures.extract();
        Some((PathBuf::from_str(path).unwrap(), hash.to_string()))
    }
}
