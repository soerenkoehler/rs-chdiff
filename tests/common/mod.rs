use assert_cmd::{Command, assert::Assert, crate_name};
use tempfile::tempdir;
use std::path::PathBuf;

#[cfg(unix)]
const ENV_HOME: &str = "HOME";

#[cfg(windows)]
const ENV_HOME: &str = "USERPROFILE";

pub fn run_in_dir(cwd: &PathBuf, args: &[&str]) -> Assert {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.args(args).current_dir(cwd).env(ENV_HOME, cwd).assert()
}

pub fn run_binary(args: &[&str]) -> Assert {
    run_in_dir(&tempdir().unwrap().into_path(), args)
}
