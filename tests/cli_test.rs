mod cli_data;
mod common;

use clap::{crate_name, crate_version};
use cli_data::*;
use predicates::{ord::eq, str::contains};

assert_stdout!(help, contains(HELP_TEXT), "help");

assert_stdout!(help_c, contains(HELP_TEXT_CREATE), "help", "c");

assert_stdout!(help_create, contains(HELP_TEXT_CREATE), "help", "create");

assert_stdout!(help_create_flag, contains(HELP_TEXT_CREATE), "c", "--help");

assert_stdout!(help_v, contains(HELP_TEXT_VERIFY), "help", "v");

assert_stdout!(help_verify, contains(HELP_TEXT_VERIFY), "help", "verify");

assert_stdout!(help_verify_flag, contains(HELP_TEXT_VERIFY), "v", "--help");

assert_stdout!(help_b, contains(HELP_TEXT_BACKUP), "help", "b");

assert_stdout!(help_backup, contains(HELP_TEXT_BACKUP), "help", "backup");

assert_stdout!(help_backup_flag, contains(HELP_TEXT_BACKUP), "b", "--help");

assert_stdout!(
    print_version,
    eq(format!("{} {}\n", crate_name!(), crate_version!())),
    "--version"
);

assert_stderr!(missing_cmd, eq(ERROR_MISSING_CMD));

assert_stderr!(empty_cmd, eq(ERROR_EMPTY_CMD), "");

assert_stderr!(wrong_cmd, eq(ERROR_WRONG_CMD), "unknown");
