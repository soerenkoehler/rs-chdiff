#[cfg(test)]
mod filelist_test;

use std::{
    fs::read_dir,
    path::{self, PathBuf},
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
    ) -> Self {
        match root_path.try_exists() {
            Ok(true) => {
                let (tx, rx) = channel();

                Self::process_path(tx, root_path);

                Self {
                    entries: rx
                        .into_iter()
                        .filter(|path| {
                            let Ok(path_abs) = path::absolute(path) else {
                                return true;
                            };
                            !exclude_absolute.matches(&path_abs) && !exclude_relative.matches(path)
                        })
                        .collect(),
                }
            }
            _ => {
                eprint!("path not found: {}", root_path.display());
                Self { entries: vec![] }
            }
        }
    }

    fn process_path(tx: Sender<PathBuf>, path: PathBuf) {
        if path.is_dir() {
            match read_dir(&path) {
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
            tx.send(path.to_owned()).unwrap();
        }
    }
}
