#![allow(dead_code)]

use assert_cmd::{Command, assert::Assert, crate_name};
use std::{fs::copy, path::PathBuf};
use tempfile::tempdir;

#[cfg(unix)]
const ENV_HOME: &str = "HOME";

#[cfg(windows)]
const ENV_HOME: &str = "USERPROFILE";

pub fn run_binary(args: &[&str]) -> Assert {
    run_with_config("tests/config_data/valid.json", args)
}

pub fn run_with_config(cfg: &str, args: &[&str]) -> Assert {
    run_in_dir(
        &TempDir::new()
            .with_file(cfg, ".chdiff-config.json")
            .as_path(),
        args,
    )
}

pub fn run_in_dir(cwd: &PathBuf, args: &[&str]) -> Assert {
    let path = assert_cmd::cargo::cargo_bin(crate_name!());
    println!("DEBUG: binary={}", path.display());
    Command::new(path)
        .args(args)
        .current_dir(cwd)
        .env(ENV_HOME, cwd)
        .assert()
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

    pub fn with_file(&self, src: &str, dst: &str) -> &TempDir {
        copy(src, self.path.as_path().join(dst)).unwrap();
        self
    }

    pub fn as_path(&self) -> PathBuf {
        self.path.clone()
    }
}
