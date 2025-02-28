use clap::{Args, Parser, Subcommand, crate_name, crate_version};

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
pub(crate) struct ArgsCreate {
    #[arg(default_value = ".")]
    path: String,
}

#[derive(Args, Debug)]
pub(crate) struct ArgsVerify {
    #[arg(default_value = ".")]
    path: String,
}

#[derive(Args, Debug)]
pub(crate) struct ArgsBackup {
    #[arg(default_value = ".")]
    path: String,
}

pub(crate) fn parse() {
    let cli = Cli::parse();

    if cli.cmd.is_none() && cli.version {
        println!("{} {}", crate_name!(), crate_version!())
    } else {
        execute(cli.cmd.unwrap_or(Command::Verify(ArgsVerify {
            path: ".".to_string(),
        })));
    }
}
