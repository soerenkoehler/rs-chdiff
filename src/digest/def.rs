use std::{collections::HashMap, path::PathBuf};

pub const REGEX_DIGEST_LINE: &str = r"^([0-9abcdefABCDEF]+)(\s\s|\s\*)(.+)$";

#[derive(Debug)]
pub struct Digest {
    pub entries: HashMap<PathBuf, String>,
}
