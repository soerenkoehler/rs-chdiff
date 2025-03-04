use clap::Parser;

use super::{
    Cli,
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
