#[cfg(test)]
mod cli_test;

use std::env::ArgsOs;

use clap::{Args, CommandFactory, Parser, Subcommand, crate_name, crate_version};

use crate::commands::execute;

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

#[derive(Args, Debug)]
pub(crate) struct ArgsBackup {
    #[arg(default_value = ".")]
    pub path: String,
}

#[derive(Args, Debug)]
pub(crate) struct ArgsCreate {
    #[arg(default_value = ".")]
    pub path: String,
}

#[derive(Args, Debug)]
pub(crate) struct ArgsVerify {
    #[arg(default_value = ".")]
    pub path: String,
}

pub(crate) fn parse(args: ArgsOs) {
    let cli = Cli::parse_from(args);

    match cli.cmd {
        Some(cmd) => execute(cmd),
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
