use crate::cli::ArgsBackup;

use super::ExecutableCommand;

impl ExecutableCommand for ArgsBackup {
    fn execute(&self) {
        println!("backup (wip) {:?}", self)
    }
}
