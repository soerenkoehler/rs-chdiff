mod backup;
mod create;
mod verify;

use crate::cli::Command;

pub(crate) fn execute(cmd: Command) {
    match cmd {
        Command::Create(args) => {
            create::run(args);
        }
        Command::Verify(args) => {
            verify::run(args);
        }
        Command::Backup(args) => {
            backup::run(args);
        }
    }
}
