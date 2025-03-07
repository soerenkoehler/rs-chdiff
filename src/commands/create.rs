use super::CommandExecutor;
use crate::cli::ArgsCreate;

pub(crate) struct Create {}

impl CommandExecutor<ArgsCreate> for Create {
    fn execute(&self, args: ArgsCreate) {
        println!("backup (wip) {:?}", args)
    }
}
