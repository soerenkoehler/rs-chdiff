use std::{
    collections::HashMap,
    fs::read_dir,
    io::Error,
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

        let _ = Digest::collect_hashes(tx, path);

        for (file, hash) in rx {
            result.entries.insert(file, hash);
        }

        result
    }

    fn collect_hashes(tx: Sender<(String, String)>, path: &Path) -> Result<(), Error> {
        read_dir(path)?.into_iter().for_each(|entry| match entry {
            Ok(entry) if entry.path().is_dir() => {
                // TODO process subdir
            }
            Ok(entry) if entry.path().is_file() => {
                // TODO calculate hash
                let _ = tx.send((
                    "".to_string(),
                    match entry.path().strip_prefix(path) {
                        Ok(path) => path.to_str().unwrap_or_default().to_string(),
                        Err(_) => "".to_string(),
                    },
                ));
            }
            _ => (),
        });
        Ok(())
    }
}
