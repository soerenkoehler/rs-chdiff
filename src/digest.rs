use std::path::PathBuf;

mod calculator;
mod file;

pub(crate) struct FileList {
    pub entries: Vec<PathBuf>,
}
