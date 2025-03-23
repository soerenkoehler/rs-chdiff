#[cfg(test)]
mod filelist_test;

use std::{
    fs::read_dir,
    io::Result,
    path::{self, Path, PathBuf},
    sync::mpsc::{Sender, channel},
    thread,
};

use crate::patternlist::PatternList;

// TODO maybe non-pub in future
pub(crate) struct FileList {
    pub entries: Vec<PathBuf>,
}

impl FileList {
    pub fn from_path(
        root_path: PathBuf,
        exclude_absolute: &PatternList,
        exclude_relative: &PatternList,
    ) -> Result<Self> {
        // Create file list in terms of absolute paths.
        let root_path = match path::absolute(root_path) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        let (tx, rx) = channel();

        Self::process_path(tx, &root_path);

        Ok(Self {
            entries: rx
                .iter()
                .filter_map(|path| {
                    if let Ok(path_rel) = path.strip_prefix(&root_path) {
                        if !exclude_absolute.matches(&path) && !exclude_relative.matches(path_rel)
                        {
                            return Some(path_rel.to_path_buf());
                        }
                    }
                    None
                })
                .collect(),
        })
    }

    fn process_path<P>(tx: Sender<PathBuf>, path: P)
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        if path.is_dir() {
            match read_dir(path) {
                Ok(dir_entries) => dir_entries
                    .filter_map(|entry| match entry {
                        Ok(entry) => Some({
                            let tx_clone: Sender<PathBuf> = tx.clone();
                            thread::spawn(move || Self::process_path(tx_clone, entry.path()))
                        }),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .for_each(|thread| thread.join().unwrap_or_default()),
                _ => eprintln!("error accessing {}", path.display()),
            }
        } else if path.is_file() {
            tx.send(path.to_path_buf()).unwrap();
        }
    }
}
