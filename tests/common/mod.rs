// #![allow(dead_code)]

use assert_cmd::{Command, assert::Assert, crate_name};
use std::{
    fs::copy,
    path::PathBuf,
};
use tempfile::tempdir;

#[cfg(unix)]
const ENV_HOME: &str = "HOME";

#[cfg(windows)]
const ENV_HOME: &str = "USERPROFILE";

pub fn run_in_dir(cwd: &PathBuf, args: &[&str]) -> Assert {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.args(args).current_dir(cwd).env(ENV_HOME, cwd).assert()
}

pub fn run_binary(args: &[&str]) -> Assert {
    run_in_dir(&TempDir::new().as_path(), args)
}

pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new() -> TempDir {
        TempDir {
            path: tempdir().unwrap().into_path(),
        }
    }

    fn with_file(&self, src: &PathBuf, dst: &PathBuf) -> &TempDir {
        copy(src, self.path.as_path().join(dst)).unwrap();
        self
    }

    pub fn as_path(&self) -> PathBuf {
        self.path.clone()
    }
}
