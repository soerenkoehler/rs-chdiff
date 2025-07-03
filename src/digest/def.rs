use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Digest {
    pub entries: HashMap<PathBuf, String>,
}
