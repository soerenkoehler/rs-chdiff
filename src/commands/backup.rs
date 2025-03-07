use super::CommandExecutor;
use std::fmt::Debug;

pub(crate) struct Backup {}

impl CommandExecutor for Backup {
    fn execute<ArgsBackup: Debug>(&self, args: ArgsBackup) {
        println!("backup (wip) {:?}", args)
    }
}
