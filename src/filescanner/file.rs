use std::{
    fs::{canonicalize, read_dir},
    io::{Error, Result},
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
        let root_path = match canonicalize(root_path) {
            Ok(path)=>path,
            Err(err)=>return Err(Error::other(format!("{} {}", err, root_path.display()))),
        };

        let (tx, rx) = channel();

        Self::process_path(tx, &root_path)?;

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

    fn process_path(tx: Sender<PathBuf>, path: &PathBuf) -> Result<()> {
        if path.is_dir() {
            match read_dir(path) {
                Err(err) => return Err(Error::other(format!("{} {}", err, path.display()))),
                Ok(entries) => {
                    if let Some(err) = entries
                        .filter_map(Result::ok)
                        .map(|entry| {
                            let tx_clone: Sender<PathBuf> = tx.clone();
                            thread::spawn(move || Self::process_path(tx_clone, &entry.path()))
                        })
                        .collect::<Vec<_>>()
                        .into_iter()
                        .filter_map(|thread| thread.join().unwrap_or(Ok(())).err())
                        .next()
                    {
                        return Err(err);
                    }
                }
            }
        } else if path.is_file() {
            let _ = tx.send(path.to_path_buf());
        } else {
            return Err(Error::other(format!(
                "neither file nor directory: {}",
                path.display()
            )));
        }
        Ok(())
    }
}
