use clap::error::ErrorKind;
use std::{ffi::OsString, path::PathBuf};

use crate::{
    Config, Dependencies,
    cli::{ArgsBackup, ArgsCreate, ArgsVerify, parse},
    commands::MockCommandExecutor,
};

macro_rules! create_mock {
    ($type:tt,
     $name:expr,
     $active:expr,
     $expected_func:expr) => {{
        let mut result = MockCommandExecutor::<$type>::new();

        let expectation = result.expect_execute().return_const(());

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
        // #[should_panic(expected=$expected_err_text)]
        fn $testname() {
            let err = parse(create_dependencies!("unknown", ""),
                vec![OsString::from("") $(, OsString::from($arg))*],
            ).unwrap_err();
            assert_eq!(err.kind(), $expected_err_kind);
            assert_eq!(err.to_string(), $expected_err_text);
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
    "error: command required\n\nUsage: chdiff [COMMAND]\n\nFor more information, try '--help'.\n"
);
test_parse_fail!(
    empty_command,
    ErrorKind::InvalidSubcommand,
    "error: unrecognized subcommand ''\n\nUsage: chdiff [COMMAND]\n\nFor more information, try '--help'.\n",
    ""
);
test_parse_fail!(
    invalid_command,
    ErrorKind::InvalidSubcommand,
    "error: unrecognized subcommand 'xxx'\n\nUsage: chdiff [COMMAND]\n\nFor more information, try '--help'.\n",
    "xxx"
);
