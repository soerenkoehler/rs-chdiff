use clap::{CommandFactory, Parser, error::Result};
use std::ffi::OsString;

use crate::Dependencies;

use crate::cli::def::{Cli, Command};

pub fn parse(deps: &Dependencies, args: Vec<OsString>) -> Result<()> {
    let cli = Cli::try_parse_from(args)?;

    Ok(match cli.cmd {
        Some(Command::Backup(args)) => deps.backup.execute(deps, args),
        Some(Command::Create(args)) => deps.create.execute(deps, args),
        Some(Command::Verify(args)) => deps.verify.execute(deps, args),
        None if cli.version => deps.version.execute(deps, ()),
        None => {
            return Err(Cli::command().error(
                clap::error::ErrorKind::MissingSubcommand,
                "command required",
            ));
        }
    })
}
