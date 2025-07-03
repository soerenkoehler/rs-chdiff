mod cli_data;
mod common;

use clap::{crate_name, crate_version};
use predicates::{ord::eq, str::contains};

use cli_data::*;
use common::run_binary;

#[test]
fn help() {
    run_binary(&["help"]).success().stdout(contains(HELP_TEXT));
}

#[test]
fn help_backup() {
    run_binary(&["help", "backup"])
        .success()
        .stdout(contains(HELP_TEXT_BACKUP));
}

#[test]
fn help_b() {
    run_binary(&["help", "b"])
        .success()
        .stdout(contains(HELP_TEXT_BACKUP));
}

#[test]
fn help_b_flag() {
    run_binary(&["b", "--help"])
        .success()
        .stdout(contains(HELP_TEXT_BACKUP));
}

#[test]
fn help_create() {
    run_binary(&["help", "create"])
        .success()
        .stdout(contains(HELP_TEXT_CREATE));
}

#[test]
fn help_c() {
    run_binary(&["help", "c"])
        .success()
        .stdout(contains(HELP_TEXT_CREATE));
}

#[test]
fn help_c_flag() {
    run_binary(&["c", "--help"])
        .success()
        .stdout(contains(HELP_TEXT_CREATE));
}

#[test]
fn help_verify() {
    run_binary(&["help", "verify"])
        .success()
        .stdout(contains(HELP_TEXT_VERIFY));
}

#[test]
fn help_v() {
    run_binary(&["help", "v"])
        .success()
        .stdout(contains(HELP_TEXT_VERIFY));
}

#[test]
fn help_v_flag() {
    run_binary(&["v", "--help"])
        .success()
        .stdout(contains(HELP_TEXT_VERIFY));
}

#[test]
fn version() {
    run_binary(&["--version"]).success().stdout(eq(format!(
        "{} {}\n",
        crate_name!(),
        crate_version!()
    )));
}
