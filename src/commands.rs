mod backup;
mod create;
mod verify;

#[cfg(test)]
mod commands_test;

use crate::cli::Command;

trait ExecutableCommand {
    fn execute(&self);
}

pub(crate) fn execute(cmd: Command) {
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
