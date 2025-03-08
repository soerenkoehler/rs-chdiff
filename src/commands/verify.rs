use super::CommandExecutor;
use crate::{cli::ArgsVerify, digest::FileList};

pub(crate) struct Verify {}

impl CommandExecutor<ArgsVerify> for Verify {
    fn execute(&self, args: ArgsVerify) {
        println!("verify (wip) {:?}", args);
        FileList::from_dir(args.path)
            .entries
            .into_iter()
            .for_each(|x| println!("{:?}", x));
    }
}
