use crate::{Dependencies, cli::ArgsBackup};

use super::CommandExecutor;

pub(crate) struct Backup {}

impl CommandExecutor<ArgsBackup> for Backup {
    fn execute(&self, _deps: &Dependencies, args: ArgsBackup) {
        println!("backup (wip) {:?}", args)
    }
}
