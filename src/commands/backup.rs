use crate::cli::ArgsBackup;

use super::ExecuteCommand;

impl ExecuteCommand for ArgsBackup {
    fn execute(&self) {
        println!("backup (wip) {:?}", self)
    }
}
