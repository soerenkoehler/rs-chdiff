use super::CommandExecutor;
use crate::{Dependencies, cli::ArgsVerify, filelist::FileList};

pub(crate) struct Verify {}

impl CommandExecutor<ArgsVerify> for Verify {
    fn execute(&self, deps: &Dependencies, args: ArgsVerify) {
        println!("verify (wip) {:?}", args);
        let mut files = FileList::from_path(
            args.path,
            &deps.config.exclude_absolute,
            &deps.config.exclude_relative,
        )
        .entries;
        files.sort();
        files.iter().for_each(|x| println!("{}", x.display()));
    }
}
