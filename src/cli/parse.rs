use clap::{crate_name, crate_version, CommandFactory, Parser};
use std::ffi::OsString;

use crate::Dependencies;

use super::def::{Cli, Command};

pub fn parse<I>(deps: &Dependencies, args: I)
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    let cli = Cli::parse_from(args);

    match cli.cmd {
        Some(Command::Backup(args)) => deps.backup.execute(deps, args),
        Some(Command::Create(args)) => deps.create.execute(deps, args),
        Some(Command::Verify(args)) => deps.verify.execute(deps, args),
        None if cli.version => println!("{} {}", crate_name!(), crate_version!()),
        None => {
            Cli::command()
                .error(
                    clap::error::ErrorKind::MissingSubcommand,
                    "Command required",
                )
                .exit();
        }
    }
}
