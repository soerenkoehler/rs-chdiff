use std::{collections::HashMap, path::PathBuf};

use super::def::Digest;

impl Digest {
    pub fn new() -> Self {
        Digest {
            entries: HashMap::from([]),
        }
    }

    pub fn from_file(_file: &PathBuf) -> Self {
        Self::new()
    }
}
