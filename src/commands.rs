mod backup;
mod create;
mod verify;

use crate::cli::Command;

trait ExecuteCommand {
    fn execute(&self);
}

pub(crate) fn execute(cmd: Command) {
    handleCommand!(cmd, Backup, Create, Verify);
    match cmd {
    Command::Create(args) => {
        args.execute();
    }
    Command::Verify(args) => {
        args.execute();
    }
    Command::Backup(args) => {
        args.execute();
    }
    }
}
