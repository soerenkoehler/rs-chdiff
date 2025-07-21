use std::path::Path;

use clap::{CommandFactory, Error};

use crate::{
    cli::{ArgsVerify, Cli}, commands::CommandExecutor, digest::Digest, filescanner::FileList, Dependencies
};

pub struct Verify {}

impl CommandExecutor<ArgsVerify> for Verify {
    fn execute(&self, deps: &Dependencies, args: ArgsVerify) -> Result<(), Error> {
        let _ = Digest::from_file(&args.path.join(Path::new(".chdiff.txt")));

        let mut files = match FileList::from_path(
            &args.path,
            &deps.config.exclude_absolute,
            &deps.config.exclude_relative,
        ) {
            Ok(value) => value,
            Err(err) => {
                return Err(Cli::command().error(clap::error::ErrorKind::Io, err.to_string()));
            }
        }
        .entries;
        files.sort();
        files.iter().for_each(|x| println!("{}", x.display()));

        // TODO call comparison
        Ok(())
    }
}
