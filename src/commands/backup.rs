use clap::Error;

use super::CommandExecutor;
use crate::{Dependencies, cli::ArgsBackup};

pub struct Backup {}

impl CommandExecutor<ArgsBackup> for Backup {
    fn execute(&self, _deps: &Dependencies, args: ArgsBackup) -> Result<(), Error> {
        println!("backup (wip) {:?}", args);
        Ok(())
    }
}
