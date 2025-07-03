mod cli;
mod commands;
mod config;
mod digest;
mod filescanner;

use std::{env::args_os, ffi::OsString};

use crate::{
    cli::{ArgsBackup, ArgsCreate, ArgsVerify},
    commands::{CommandExecutor, backup::Backup, create::Create, verify::Verify, version::Version},
    config::Config,
};

struct Dependencies {
    backup: Box<dyn CommandExecutor<ArgsBackup>>,
    create: Box<dyn CommandExecutor<ArgsCreate>>,
    verify: Box<dyn CommandExecutor<ArgsVerify>>,
    version: Box<dyn CommandExecutor<()>>,
    config: Config,
}

pub fn main() {
    match Config::from_file(&Config::get_config_path()) {
        Ok(cfg) => {
            if let Err(err) = cli::parse(
                &Dependencies {
                    backup: Box::new(Backup {}),
                    create: Box::new(Create {}),
                    verify: Box::new(Verify {}),
                    version: Box::new(Version {}),
                    config: cfg,
                },
                args_os().collect::<Vec<OsString>>(),
            ) {
                err.exit();
            }
        }
        Err(err) => eprintln!("reading config file: {}", err),
    }
}
