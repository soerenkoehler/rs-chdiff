use std::path::Path;

use crate::{
    Dependencies, cli::ArgsVerify, commands::CommandExecutor, digest::Digest, filescanner::FileList,
};

pub struct Verify {}

impl CommandExecutor<ArgsVerify> for Verify {
    fn execute(&self, deps: &Dependencies, args: ArgsVerify) {
        println!("verify (wip) {:?}", args);
        let _ = Digest::from_file(&args.path.join(Path::new(".chdiff.txt").to_path_buf()));
        let mut files = match FileList::from_path(
            &args.path,
            &deps.config.exclude_absolute,
            &deps.config.exclude_relative,
        ) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("error accessing {}: {} \n", args.path.display(), err);
                return;
            }
        }
        .entries;
        files.sort();
        files.iter().for_each(|x| println!("{}", x.display()));
    }
}
