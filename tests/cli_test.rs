mod cli_test_data;

use assert_cmd::{
    assert::OutputAssertExt,
    cargo::{CargoError, CommandCargoExt},
};
use clap::{crate_name, crate_version};
use predicates::prelude::{
    PredicateBooleanExt,
    predicate::{eq, str::contains},
};
use std::process::Command;

macro_rules! run_binary {
    ( $( $a:expr ),* ) => {{
        Command::cargo_bin("rs-chdiff")?
        $(
            .arg($a)
        )*
        .assert()
        .success()
    }}
}

macro_rules! assert_stdout {
    ($n:ident,$p:expr,$($a:expr),*) => {
        #[test]
        fn $n() -> Result<(), CargoError> {
            run_binary!($($a),*).stdout($p);
            Ok(())
        }
    };
}

assert_stdout!(long_help, contains(cli_test_data::HELP_TEXT), "help");
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
