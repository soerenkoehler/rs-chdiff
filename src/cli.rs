#[cfg(test)]
mod cli_test;

use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum, crate_name, crate_version};
use std::{ffi::OsString, path::PathBuf};

use crate::Dependencies;

#[derive(Parser, Debug)]
#[command(about, version, long_version = "Y", disable_version_flag = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Command>,
    #[arg(long, help = "Print version")]
    pub version: bool,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    #[command(visible_alias = "c")]
    Create(ArgsCreate),
    #[command(visible_alias = "v")]
    Verify(ArgsVerify),
    #[command(visible_alias = "b")]
    Backup(ArgsBackup),
}

#[derive(ValueEnum, Clone, Debug)]
pub(crate) enum HashAlgorithm {
    Sha256,
    Sha512,
}

#[derive(Args, Debug)]
pub(crate) struct ArgsBackup {
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub(crate) struct ArgsCreate {
    #[arg(default_value = ".")]
    pub path: PathBuf,
    #[arg(short, long, value_enum, ignore_case=true, default_value="sha256")]
    pub algorithm: HashAlgorithm,
}

#[derive(Args, Debug)]
pub(crate) struct ArgsVerify {
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

pub(crate) fn parse<I>(deps: &Dependencies, args: I)
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
