use std::{
    fs::read_dir,
    path::PathBuf,
    sync::mpsc::{channel, Sender}, thread,
};

use super::FileList;

impl FileList {
    pub fn from_dir(path: PathBuf) -> FileList {
        let (tx, rx) = channel();
        let mut result: FileList = FileList {
            entries: Vec::new(),
        };

        Self::process_path(tx, path);

        rx.iter().for_each(|file| result.entries.push(file));

        result
    }

    fn process_path(tx: Sender<PathBuf>, path: PathBuf) {
        if path.is_dir() {
            read_dir(path)
                .unwrap() // TODO Handle Errors
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
                .for_each(|thread| thread.join().unwrap_or_default());
        } else if path.is_file() {
            tx.send(path.to_owned()).unwrap();
        }
    }
}
