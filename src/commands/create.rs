use crate::cli::ArgsCreate;

use super::ExecuteCommand;

impl ExecuteCommand for ArgsCreate {
    fn execute(&self) {
        println!("create (wip) {:?}", self)
    }
}
