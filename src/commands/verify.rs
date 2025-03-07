use super::CommandExecutor;
use crate::cli::ArgsVerify;

pub(crate) struct Verify {}

impl CommandExecutor<ArgsVerify> for Verify {
    fn execute(&self, args: ArgsVerify) {
        println!("backup (wip) {:?}", args)
        // let _digest = Digest::from_dir(Path::new(&self.path));
    }
}
