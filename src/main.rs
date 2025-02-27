mod cli;

use clap::{crate_name, crate_version, Parser};
use cli::{Cli, Command};

pub fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Some(Command::Create(args)) => {
            println!("create (wip) {:?}", args)
        }
        Some(Command::Verify(args)) => {
            println!("verify (wip) {:?}", args)
        }
        Some(Command::Backup(args)) => {
            println!("backup (wip) {:?}", args)
        }
        None => {
            if cli.version {
                println!("{} {}", crate_name!(), crate_version!())
            }
        }
    }
}
