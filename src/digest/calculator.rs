use std::{collections::HashMap, path::Path};

use super::Digest;

impl Digest {
    pub fn from_dir(_path: &Path) -> Digest {
        Digest {
            entries: HashMap::new(),
        }
    }
}
