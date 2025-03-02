mod calculator;
mod file;

use std::collections::HashMap;

pub(crate) struct Digest {
    entries: HashMap<String, String>,
}
