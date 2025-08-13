use std::path::Path;

use clap::{CommandFactory, Error};

use crate::{
    Dependencies,
    cli::{ArgsVerify, Cli},
    commands::CommandExecutor,
    digest::Digest,
    filescanner::FileList,
};

pub struct Verify {}

impl CommandExecutor<ArgsVerify> for Verify {
    fn execute(&self, deps: &Dependencies, args: ArgsVerify) -> Result<(), Error> {
        let _digest = match Digest::from_file(&args.path.join(Path::new(".chdiff.txt"))) {
            Ok(ok) => ok,
            Err(err) => return to_clap_error(err),
        };

        let file_list = match FileList::from_path(
            &args.path,
            &deps.config.exclude_absolute,
            &deps.config.exclude_relative,
        ) {
            Ok(ok) => ok,
            Err(err) => return to_clap_error(err),
        };

        file_list.errors.iter().for_each(|x| eprintln!("{}", x));

        let mut files = file_list.entries;
        files.sort();
        files.iter().for_each(|x| println!("{}", x.display()));

        // TODO call comparison
        Ok(())
    }
}

fn to_clap_error(err: std::io::Error) -> Result<(), clap::error::Error> {
    Err(Cli::command().error(clap::error::ErrorKind::Io, err.to_string()))
}
