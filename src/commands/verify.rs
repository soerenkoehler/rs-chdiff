use super::CommandExecutor;
use crate::{cli::ArgsVerify, digest::filelist::FileList};

pub(crate) struct Verify {}

impl CommandExecutor<ArgsVerify> for Verify {
    fn execute(&self, args: ArgsVerify) {
        println!("verify (wip) {:?}", args);
        let mut files = FileList::from_dir(args.path).entries;
        files.sort();
        files.into_iter().for_each(|x| println!("{:?}", x));
    }
}
