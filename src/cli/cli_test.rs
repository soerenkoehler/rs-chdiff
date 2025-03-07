use clap::Parser;

use crate::commands::MockCommandExecutor;

use super::{
    ArgsBackup, ArgsCreate, ArgsVerify, Cli,
    Command::{Backup, Create, Verify},
};

#[test]
fn default_path_backup() {
    let Some(Backup(args)) = Cli::parse_from(["", "b"]).cmd else {
        panic!("expected command: backup")
    };
    assert_eq!(args.path, ".")
}

#[test]
fn default_path_create() {
    let Some(Create(args)) = Cli::parse_from(["", "c"]).cmd else {
        panic!("expected command: create")
    };
    assert_eq!(args.path, ".")
}

#[test]
fn default_path_verify() {
    let Some(Verify(args)) = Cli::parse_from(["", "v"]).cmd else {
        panic!("expected command: verify")
    };
    assert_eq!(args.path, ".")
}

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

            crate::cli::parse(["", $cmd], &mock_backup, &mock_create, &mock_verify);
        }
    };
}

command_mapping_test!(command_mapping_backup, "b", once, never, never);
command_mapping_test!(command_mapping_create, "c", never, once, never);
command_mapping_test!(command_mapping_verify, "v", never, never, once);
