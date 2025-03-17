use std::{
    fs::read_dir,
    path::PathBuf,
    sync::mpsc::{Sender, channel},
    thread,
};

use glob::Pattern;

// TODO maybe non-pub in future
pub(crate) struct FileList {
    pub entries: Vec<PathBuf>,
}

pub type PatternList = Vec<Pattern>;

impl FileList {
    pub fn from_dir(
        path: PathBuf,
        exclude_absolute: &PatternList,
        exclude_relative: &PatternList,
    ) -> FileList {
        let (tx, rx) = channel();

        Self::process_path(tx, path);

        FileList {
            entries: rx.iter().filter(|_entry| {
                // TODO apply filter
                true
            }).collect(),
        }
    }

    fn process_path(tx: Sender<PathBuf>, path: PathBuf) {
        if path.is_dir() {
            match read_dir(&path) {
                Ok(dir_entries) => dir_entries
                    .into_iter()
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

    // TODO fn filter_path(path:&Pathbuf,Vec)
}
