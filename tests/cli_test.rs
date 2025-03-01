mod common;
mod data;

use clap::{crate_name, crate_version};
use predicates::{ord::eq, prelude::PredicateBooleanExt, str::contains};

assert_stdout!(long_help, contains(data::HELP_TEXT), "help");
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
