use assert_cmd::{
    assert::OutputAssertExt,
    cargo::{CargoError, CommandCargoExt},
};
use clap::{crate_name, crate_version};
use predicates::prelude::{
    predicate::{eq, str::contains}, PredicateBooleanExt
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

#[test]
fn long_help() -> Result<(), CargoError> {
    run_binary!("help").stdout(contains(
        "\
Create, verify and compare hash sums on whole directory trees.


Usage: rs-chdiff [COMMAND]

Commands:
  create  [aliases: c]
  verify  [aliases: v]
  backup  [aliases: b]
  help    Print this message or the help of the given subcommand(s)

Options:
      --version  Print version
  -h, --help     Print help",
    ));

    Ok(())
}

#[test]
fn verify_is_default_cmd()->Result<(), CargoError> {
    run_binary!().stdout(contains("verify (wip)").and(contains("path: \".\"")));

    Ok(())
}

#[test]
fn create_defaultpath() -> Result<(), CargoError> {
    run_binary!("c").stdout(contains("create (wip)").and(contains("path: \".\"")));

    Ok(())
}

#[test]
fn verify_defaultpath() -> Result<(), CargoError> {
    run_binary!("v").stdout(contains("verify (wip)").and(contains("path: \".\"")));

    Ok(())
}

#[test]
fn backup_defaultpath() -> Result<(), CargoError> {
    run_binary!("b").stdout(contains("backup (wip)").and(contains("path: \".\"")));

    Ok(())
}

#[test]
fn print_version() -> Result<(), CargoError> {
    run_binary!("--version").stdout(eq(format!("{} {}\n", crate_name!(), crate_version!())));

    Ok(())
}
