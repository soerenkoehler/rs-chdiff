use std::{
    collections::HashMap,
    fs::read_dir,
    path::Path,
    sync::mpsc::{Sender, channel},
};

use super::Digest;

impl Digest {
    pub fn from_dir(path: &Path) -> Digest {
        let (tx, rx) = channel();
        let mut result: Digest = Digest {
            entries: HashMap::new(),
        };

        let _ = Digest::process_path(tx, path);

        for (file, hash) in rx {
            result.entries.insert(file, hash);
        }

        result
    }

    fn process_path(tx: Sender<(String, String)>, path: &Path) {
        // TODO threads
        if path.is_dir() {
            Self::process_dir(tx, path)
        } else if path.is_file() {
            Self::process_file(tx, path)
        }
    }

    fn process_dir(tx: Sender<(String, String)>, path: &Path) {
        // TODO handle errors from read_dir
        read_dir(path).unwrap().into_iter().for_each(|entry| match entry {
            Ok(entry) => Self::process_path(tx.clone(), entry.path().as_path()),
            Err(_) => (),
        });
    }

    fn process_file(tx: Sender<(String, String)>, path: &Path) {
        // TODO calculate hash
        tx.send((
            "".to_string(),
            match path.strip_prefix(path) {
                Ok(path) => path.to_str().unwrap_or_default().to_string(),
                Err(_) => "".to_string(),
            },
        )).unwrap()
    }
}
