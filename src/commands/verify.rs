use super::CommandExecutor;
use crate::{Dependencies, cli::ArgsVerify, filescanner::FileList};

pub struct Verify {}

impl CommandExecutor<ArgsVerify> for Verify {
    fn execute(&self, deps: &Dependencies, args: ArgsVerify) {
        println!("verify (wip) {:?}", args);
        let mut files = FileList::from_path(
            args.path,
            &deps.config.exclude_absolute,
            &deps.config.exclude_relative,
        )
        .unwrap()
        .entries;
        files.sort();
        files.iter().for_each(|x| println!("{}", x.display()));
    }
}
