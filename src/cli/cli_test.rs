use clap::Parser;
use std::path::PathBuf;

use crate::{
    Config, Dependencies,
    cli::{
        ArgsBackup, ArgsCreate, ArgsVerify,
        def::{
            Cli,
            Command::{Backup, Create, Verify},
        },
    },
    commands::MockCommandExecutor,
};

macro_rules! command_args_test {
    ($testname:ident, $type:ident, $expected_cmd:expr, $expected_arg:expr, $cmd:expr $(,$arg:expr)* ) => {
        #[test]
        fn $testname() {
            let Some($type(args)) = Cli::parse_from(["", $cmd, $($arg,)*]).cmd else {
                panic!("expected command: {}", $expected_cmd);
            };
            assert_eq!(args.path, PathBuf::from($expected_arg))
        }
    };
}

command_args_test!(default_path_backup, Backup, "backup", ".", "b");
command_args_test!(arg_path_backup, Backup, "backup", "x", "b", "x");
command_args_test!(default_path_create, Create, "create", ".", "c");
command_args_test!(arg_path_create, Create, "create", "y", "c", "y");
command_args_test!(default_path_verify, Verify, "verify", ".", "v");
command_args_test!(arg_path_verify, Verify, "verify", "z", "v", "z");

macro_rules! command_mapping_test {
    ($testname:ident, $cmd:expr, $a:ident, $b:ident, $c:ident) => {
        #[test]
        fn $testname() {
            let mut mock_backup = MockCommandExecutor::<ArgsBackup>::new();
            let mut mock_create = MockCommandExecutor::<ArgsCreate>::new();
            let mut mock_verify = MockCommandExecutor::<ArgsVerify>::new();

            mock_backup.expect_execute().$a().return_const(());
            mock_create.expect_execute().$b().return_const(());
            mock_verify.expect_execute().$c().return_const(());

            crate::cli::parse(
                &Dependencies {
                    backup: Box::new(mock_backup),
                    create: Box::new(mock_create),
                    verify: Box::new(mock_verify),
                    config: Config::new(),
                },
                ["", $cmd],
            );
        }
    };
}

command_mapping_test!(command_mapping_backup, "b", once, never, never);
command_mapping_test!(command_mapping_create, "c", never, once, never);
command_mapping_test!(command_mapping_verify, "v", never, never, once);
