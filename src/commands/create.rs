use crate::cli::ArgsCreate;

use super::ExecutableCommand;

impl ExecutableCommand for ArgsCreate {
    fn execute(&self) {
        println!("create (wip) {:?}", self)
    }
}
