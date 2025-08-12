use std::{
    fs::{canonicalize, read_dir},
    io::{Error, Result},
    path::PathBuf,
    sync::mpsc::{Sender, channel},
    thread::{self},
};

use super::PatternList;

#[derive(Debug)]
pub struct FileList {
    pub entries: Vec<PathBuf>,
    pub errors: Vec<Error>,
}

type FileListSender = Sender<Result<PathBuf>>;

fn send_pathbuf(tx: FileListSender, pathbuf: PathBuf) {
    let _ = tx.send(Ok(pathbuf));
}

fn send_error(tx: FileListSender, error: Error) {
    let _ = tx.send(Err(error));
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

        Self::process_path(tx, &root_path);

        let mut results = Self {
            entries: vec![],
            errors: vec![],
        };

        rx.iter().for_each(|result: Result<PathBuf>| {
            match result {
                Ok(path) => {
                    let path_rel = path.strip_prefix(&root_path).unwrap().to_path_buf();
                    if !exclude_absolute.matches(&path) && !exclude_relative.matches(&path_rel) {
                        // return file list with relative paths
                        results.entries.push(path_rel);
                    }
                }
                Err(err) => results.errors.push(err),
            }
        });

        Ok(results)
    }

    fn process_path(tx: FileListSender, path: &PathBuf) {
        if path.is_dir() {
            Self::process_dir(tx, path);
        } else if path.is_file() {
            send_pathbuf(tx, path.to_path_buf());
        } else {
            send_error(
                tx,
                Error::other(format!("neither file nor directory: {}", path.display())),
            );
        }
    }

    fn process_dir(tx: FileListSender, dir: &PathBuf) {
        match read_dir(dir) {
            Ok(entries) => {
                let thread_error_count = entries
                    .filter_map(Result::ok)
                    .map(|entry| {
                        let tx_clone=tx.clone();
                        thread::spawn(move || Self::process_path(tx_clone, &entry.path()))
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .filter_map(|thread| thread.join().err())
                    .count();
                if thread_error_count > 0 {
                    // TODO probably no coverage possible (can't reliably provoke join errors)
                    send_error(
                        tx,
                        Error::other(format!("could not join {} threads", thread_error_count)),
                    )
                }
            }
            Err(err) => send_error(tx, err),
        };
    }
}
