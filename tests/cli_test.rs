mod cli_data;
mod common;

use clap::{crate_name, crate_version};
use cli_data::*;
use common::run_binary;
use predicates::{ord::eq, str::contains};

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

// assert_stdout!(help, contains(HELP_TEXT), "help");

// assert_stdout!(help_c, contains(HELP_TEXT_CREATE), "help", "c");

// assert_stdout!(help_create, contains(HELP_TEXT_CREATE), "help", "create");

// assert_stdout!(help_create_flag, contains(HELP_TEXT_CREATE), "c", "--help");

// assert_stdout!(help_v, contains(HELP_TEXT_VERIFY), "help", "v");

// assert_stdout!(help_verify, contains(HELP_TEXT_VERIFY), "help", "verify");

// assert_stdout!(help_verify_flag, contains(HELP_TEXT_VERIFY), "v", "--help");

// assert_stdout!(help_b, contains(HELP_TEXT_BACKUP), "help", "b");

// assert_stdout!(help_backup, contains(HELP_TEXT_BACKUP), "help", "backup");

// assert_stdout!(help_backup_flag, contains(HELP_TEXT_BACKUP), "b", "--help");

#[test]
fn version() {
    run_binary(&["--version"]).success().stdout(eq(format!(
        "{} {}\n",
        crate_name!(),
        crate_version!()
    )));
}

#[test]
fn missing_cmd() {
    run_binary(&[]).failure().stderr(eq(ERROR_MISSING_CMD));
}

#[test]
fn empty_cmd() {
    run_binary(&[""]).failure().stderr(eq(ERROR_EMPTY_CMD));
}

#[test]
fn wrong_cmd() {
    run_binary(&["unknown"]).failure().stderr(eq(ERROR_WRONG_CMD));
}

// assert_stdout!(
//     print_version,
//     eq(format!("{} {}\n", crate_name!(), crate_version!())),
//     "--version"
// );

// assert_stderr!(missing_cmd, eq(ERROR_MISSING_CMD));

// assert_stderr!(empty_cmd, eq(ERROR_EMPTY_CMD), "");

// assert_stderr!(wrong_cmd, eq(ERROR_WRONG_CMD), "unknown");
