use std::{
    fs::{canonicalize, read_dir},
    io::Result,
    path::PathBuf,
    sync::mpsc::{Sender, channel},
    thread::{self},
};

use super::PatternList;

pub struct FileList {
    pub entries: Vec<PathBuf>,
}

impl FileList {
    pub fn from_path(
        root_path: &PathBuf,
        exclude_absolute: &PatternList,
        exclude_relative: &PatternList,
    ) -> Result<Self> {
        // Create file list in terms of absolute paths.
        let root_path = canonicalize(root_path)?;

        let (tx, rx) = channel();

        Self::process_path(tx, &root_path);

        Ok(Self {
            entries: rx
                .iter()
                .filter_map(|path: PathBuf| {
                    let path_rel = path.strip_prefix(&root_path).unwrap().to_path_buf();
                    if !exclude_absolute.matches(&path) && !exclude_relative.matches(&path_rel) {
                        Some(path_rel.to_path_buf())
                    } else {
                        None
                    }
                })
                .collect(),
        })
    }

    fn process_path(tx: Sender<PathBuf>, path: &PathBuf) {
        if path.is_dir() {
            match read_dir(path) {
                Ok(dir_entries) => dir_entries
                    .filter_map(Result::ok)
                    .map(|entry| {
                        let tx_clone: Sender<PathBuf> = tx.clone();
                        thread::spawn(move || Self::process_path(tx_clone, &entry.path()))
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    // TODO replace unwrap() with error handling
                    .for_each(|thread| thread.join().unwrap_or_default()),
                _ => eprintln!("error accessing {}", path.display()),
            }
        } else if path.is_file() {
            let _ = tx.send(path.to_path_buf());
        } else {
            eprintln!("neither file nor directory: {}", path.display())
        }
    }
}
