use std::{
    collections::HashMap,
    path::PathBuf,
};

use crate::cli::HashAlgorithm;

#[derive(Debug)]
pub struct Digest {
    pub hash_algorithm: Option<HashAlgorithm>,
    pub entries: HashMap<PathBuf, String>,
}
