use super::CommandExecutor;
use std::fmt::Debug;

pub(crate) struct Create {}

impl CommandExecutor for Create {
    fn execute<ArgsCreate: Debug>(&self, args: ArgsCreate) {
        println!("backup (wip) {:?}", args)
    }
}
