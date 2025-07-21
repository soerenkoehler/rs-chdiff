use clap::error::{Error, ErrorKind};
use clap::{CommandFactory, Parser};
use std::ffi::OsString;

use crate::Dependencies;

use crate::cli::def::{Cli, Command};

pub fn parse(deps: &Dependencies, args: Vec<OsString>) -> Result<(), Error> {
    match Cli::try_parse_from(args) {
        Err(err) => Err(err),
        Ok(cli) => match cli.cmd {
            Some(Command::Backup(args)) => deps.backup.execute(deps, args),
            Some(Command::Create(args)) => deps.create.execute(deps, args),
            Some(Command::Verify(args)) => deps.verify.execute(deps, args),
            None if cli.version => deps.version.execute(deps, ()),
            None => Err(Cli::command().error(
                ErrorKind::MissingSubcommand,
                String::from("command required"),
            )),
        },
    }
}
