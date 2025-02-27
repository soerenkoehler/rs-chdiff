mod cli;

use clap::Parser;
use cli::{Cli, Command};

pub fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Create(args) => {
            println!("create (wip) {:?}", args)
        }
        Command::Verify(args) => {
            println!("verify (wip) {:?}", args)
        }
        Command::Backup(args) => {
            println!("backup (wip) {:?}", args)
        }
    }
}
