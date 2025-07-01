mod common;

use predicates::str::contains;
use std::{
    fs::{self, Permissions},
    path::{Path, PathBuf},
};

use common::{run_in_dir, run_with_config};

#[test]
fn missing_config_file() {
    let cwd = tempfile::tempdir().unwrap().into_path();
    let expect = format!(
        "created default config file: {}",
        Path::join(&cwd, ".chdiff-config.json").to_str().unwrap()
    );
    run_in_dir(&cwd, &["v"]).success().stdout(contains(expect));
}

#[test]
fn cant_create_default_config_file() {
    fn set_readonly(path: &PathBuf, readonly: bool) {
        let mut permissions: Permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_readonly(readonly);
        fs::set_permissions(&path, permissions).unwrap();
    }

    let cwd = tempfile::tempdir().unwrap().into_path();

    // provoke error by making cwd readonly
    set_readonly(&cwd, true);

    run_in_dir(&cwd, &["v"])
        .success()
        .stderr(contains("Reading config file: Permission denied"));

    // reset readonly flag on cwd
    set_readonly(&cwd, false);
}
