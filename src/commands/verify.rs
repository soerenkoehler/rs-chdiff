use super::CommandExecutor;
use std::fmt::Debug;

pub(crate) struct Verify {}

impl CommandExecutor for Verify {
    fn execute<ArgsVerify: Debug>(&self, args: ArgsVerify) {
        println!("backup (wip) {:?}", args)
        // let _digest = Digest::from_dir(Path::new(&self.path));
    }
}
