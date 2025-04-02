use std::{collections::HashMap, path::PathBuf};

pub struct Digest {
    pub entries: HashMap<PathBuf, String>,
}
