mod cli_data;
mod common;

use clap::{crate_name, crate_version};
use cli_data::{HELP_TEXT, HELP_TEXT_BACKUP, HELP_TEXT_CREATE, HELP_TEXT_VERIFY};
use predicates::{ord::eq, prelude::PredicateBooleanExt, str::contains};

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
    verify_is_default_cmd,
    contains("verify (wip)").and(contains("path: \".\"")),
);
assert_stdout!(
    create_defaultpath,
    contains("create (wip)").and(contains("path: \".\"")),
    "c"
);
assert_stdout!(
    verify_defaultpath,
    contains("verify (wip)").and(contains("path: \".\"")),
    "v"
);
assert_stdout!(
    backup_defaultpath,
    contains("backup (wip)").and(contains("path: \".\"")),
    "b"
);
assert_stdout!(
    print_version,
    eq(format!("{} {}\n", crate_name!(), crate_version!())),
    "--version"
);
