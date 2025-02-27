use assert_cmd::{cargo::CargoError, prelude::*}; // Add methods on commands
use predicates::prelude::{predicate::str::contains, *}; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn create_defaultpath() -> Result<(), CargoError> {
    let mut cmd = Command::cargo_bin("rs-chdiff")?;

    cmd.arg("c") //
        .assert()
        .success()
        .stdout(
            contains("create (wip)") //
                .and(contains("path: \"..\"")),
        );

    Ok(())
}

#[test]
fn verify_defaultpath() -> Result<(), CargoError> {
    let mut cmd = Command::cargo_bin("rs-chdiff")?;

    cmd.arg("v") //
        .assert()
        .success()
        .stdout(
            contains("verify (wip)") //
                .and(contains("path: \".\"")),
        );

    Ok(())
}

#[test]
fn backup_defaultpath() -> Result<(), CargoError> {
    let mut cmd = Command::cargo_bin("rs-chdiff")?;

    cmd.arg("b") //
        .assert()
        .success()
        .stdout(
            contains("backup (wip)") //
                .and(contains("path: \".\"")),
        );

    Ok(())
}
