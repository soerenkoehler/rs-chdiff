use std::{
    collections::HashMap,
    fs::read_dir,
    path::{Path, PathBuf},
    sync::mpsc::{Sender, channel},
    thread::{self},
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
            println!("{hash}  {}", file.strip_prefix(path).unwrap().to_str().unwrap());
            result.entries.insert(file.to_str().unwrap().to_string(), hash);
        }

        result
    }

    fn process_path(tx: Sender<(PathBuf, String)>, path: &Path) {
        // TODO threads
        let wait = &mut vec![];
        if path.is_dir() {
            wait.append(&mut Self::process_dir(tx, path))
        } else if path.is_file() {
            let tx_clone = tx.clone();
            let path_clone = path.to_owned();
            wait.push(thread::spawn(move || {
                Self::process_file(tx_clone, path_clone.as_path())
            }))
        }
    }

    fn process_dir(tx: Sender<(PathBuf, String)>, path: &Path) -> Vec<thread::JoinHandle<()>> {
        let mut wait = vec![];
        // TODO handle errors from read_dir
        read_dir(path)
            .unwrap()
            .into_iter()
            .for_each(|entry| match entry {
                Ok(entry) => {
                    let tx_clone = tx.clone();
                    let path_clone = entry.path();
                    wait.push(thread::spawn(move || {
                        Self::process_path(tx_clone, path_clone.as_path())
                    }))
                }
                Err(_) => (),
            });
        wait
    }

    fn process_file(tx: Sender<(PathBuf, String)>, path: &Path) {
        // TODO calculate hash
        tx.send((path.to_owned(), "dummy".to_string())).unwrap()
    }
}
