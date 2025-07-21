use std::{
    io::{Error, Result},
    path::PathBuf,
    collections::HashMap,
};

use crate::{cli::HashAlgorithm, digest::Digest};

impl Digest {
    pub fn new() -> Self {
        Digest {
            hash_algorithm: None,
            entries: HashMap::from([]),
        }
    }

    pub fn add(&mut self, path: PathBuf, hash: String) -> Result<()> {
        let new_algorithm = match hash.len() {
            64 => &HashAlgorithm::Sha256,
            128 => &HashAlgorithm::Sha512,
            _ => {
                return Err(Error::other(format!(
                    "unknown hash type with length {}",
                    hash.len()
                )));
            }
        };

        match &self.hash_algorithm {
            None => self.hash_algorithm = Some(new_algorithm.clone()),
            Some(old_algorithm) if old_algorithm != new_algorithm => {
                return Err(Error::other("mixed hash sizes are not supported"));
            }
            _ => (),
        }

        if self.entries.contains_key(&path) {
            return Err(Error::other(format!(
                "hash already defined for path {}",
                path.display()
            )));
        }

        self.entries.insert(path, hash);
        Ok(())
    }
}
