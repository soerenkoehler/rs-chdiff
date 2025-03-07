use super::CommandExecutor;
use crate::cli::ArgsBackup;

pub(crate) struct Backup {}

impl CommandExecutor<ArgsBackup> for Backup {
    fn execute(&self, args: ArgsBackup) {
        println!("backup (wip) {:?}", args)
    }
}
