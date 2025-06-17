use regex::Regex;
use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

pub static REGEX_DIGEST_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([0-9abcdefABCDEF]+)(\s\s|\s\*)(.+)$").unwrap());

#[derive(Debug)]
pub struct Digest {
    pub entries: HashMap<PathBuf, String>,
}
