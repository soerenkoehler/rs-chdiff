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
    pub errors: Vec<Error>,
}

impl FileList {
    pub fn from_path(
        root_path: &PathBuf,
        exclude_absolute: &PatternList,
        exclude_relative: &PatternList,
    ) -> Result<Self> {

        // scan file system in terms of absolute paths
        let root_path = match canonicalize(root_path) {
            Ok(path) => path,
            Err(err) => return Err(Error::other(format!("{} {}", err, root_path.display()))),
        };

        let (tx, rx) = channel();

        Self::process_path(tx, &root_path)?;

        let mut results = Self {
            entries: vec![],
            errors: vec![],
        };

        rx.iter().for_each(|result: Result<PathBuf>| match result {
            Ok(path) => {
                let path_rel = path.strip_prefix(&root_path).unwrap().to_path_buf();
                if !exclude_absolute.matches(&path) && !exclude_relative.matches(&path_rel) {
                    // return file list with relative paths
                    results.entries.push(path_rel);
                }
            }
            Err(err) => results.errors.push(err),
        });

        Ok(results)
    }

    fn process_path(tx: Sender<Result<PathBuf>>, path: &PathBuf) -> Result<()> {
        match path {
            file if file.is_file() => {
                let _ = tx.send(Ok(path.to_path_buf()));
                Ok(())
            }

            dir if dir.is_dir() => match read_dir(dir)?
                .filter_map(Result::ok)
                .map(|entry| {
                    let tx_clone = tx.clone();
                    thread::spawn(move || Self::process_path(tx_clone, &entry.path()))
                })
                .collect::<Vec<_>>()
                .into_iter()
                .filter_map(|thread| thread.join().err())
                .count()
            { // TODO probably no coverage possible (can't reliably provoke join errors)
                0 => Ok(()),
                thread_error_count => Err(Error::other(format!(
                    "could not join {} threads",
                    thread_error_count
                ))),
            },

            other => Err(Error::other(format!(
                "neither file nor directory: {}",
                other.display()
            ))),
        }
    }
}
