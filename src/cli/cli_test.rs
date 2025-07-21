use clap::{crate_name, crate_version, error::ErrorKind};
use predicates::{ord::eq, str::contains};
use std::{ffi::OsString, path::PathBuf};

use crate::{
    CliErrorText, Config, Dependencies,
    cli::{ArgsBackup, ArgsCreate, ArgsVerify, parse},
    commands::MockCommandExecutor,
    tests::{cli_data::*, runner::run_binary},
};

macro_rules! create_mock {
    ($type:tt,
     $name:expr,
     $active:expr,
     $expected_func:expr) => {{
        let mut result = MockCommandExecutor::<$type>::new();

        let expectation = result.expect_execute().return_once(|_, _| Ok(()));

        if $name == $active {
            expectation.once().withf($expected_func);
        } else {
            expectation.never();
        }

        result
    }};
}

macro_rules! create_mock_with_path {
    ($type:tt,
     $name:expr,
     $active:expr,
     $expected_path:expr) => {
        create_mock!($type, $name, $active, |_, args| {
            args.path == PathBuf::from($expected_path)
        })
    };
}

macro_rules! create_dependencies {
    ($expected_cmd:expr,
     $expected_path:expr) => {
        &Dependencies {
            backup: Box::new(create_mock_with_path!(
                ArgsBackup,
                "backup",
                $expected_cmd,
                $expected_path
            )),
            create: Box::new(create_mock_with_path!(
                ArgsCreate,
                "create",
                $expected_cmd,
                $expected_path
            )),
            verify: Box::new(create_mock_with_path!(
                ArgsVerify,
                "verify",
                $expected_cmd,
                $expected_path
            )),
            version: Box::new(create_mock!((), "version", $expected_cmd, |_, _| true)),
            config: Config::new(),
        }
    };
}

macro_rules! test_parse {
    ($testname:ident,
     $expected_cmd:expr,
     $expected_path:expr
     $(,$arg:expr)* ) => {
        #[test]
        fn $testname() {
            let _ = parse(create_dependencies!($expected_cmd, $expected_path),
                vec![OsString::from("") $(, OsString::from($arg))*],
            );
        }
    };
}

macro_rules! test_parse_fail {
    ($testname:ident,
     $expected_err_kind:expr,
     $expected_err_text:expr
     $(,$arg:expr)* ) => {
        #[test]
        fn $testname() {
            match parse(create_dependencies!("unknown", ""),
                vec![OsString::from("") $(, OsString::from($arg))*],
            ) {
                Err(err) => {
                    assert_eq!(err.kind(), $expected_err_kind);
                    assert!(err.to_string().starts_with(&$expected_err_text));
                }
                _ => panic!("should report error type Clap"),
            }
        }
    };
}

test_parse!(backup_default, "backup", ".", "b");
test_parse!(backup_path, "backup", "x", "b", "x");
test_parse!(create_default, "create", ".", "c");
test_parse!(create_path, "create", "y", "c", "y");
test_parse!(verify_default, "verify", ".", "v");
test_parse!(verify_path, "verify", "z", "v", "z");
test_parse!(version, "version", "", "--version");
test_parse_fail!(
    missing_command,
    ErrorKind::MissingSubcommand,
    CliErrorText!("error: command required")
);
test_parse_fail!(
    empty_command,
    ErrorKind::InvalidSubcommand,
    CliErrorText!("error: unrecognized subcommand ''"),
    ""
);
test_parse_fail!(
    invalid_command,
    ErrorKind::InvalidSubcommand,
    CliErrorText!("error: unrecognized subcommand 'xxx'"),
    "xxx"
);

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
fn version_output() {
    run_binary(&["--version"]).success().stdout(eq(format!(
        "{} {}\n",
        crate_name!(),
        crate_version!()
    )));
}
