mod cli;
mod commands;
mod config;
mod digest;
mod filescanner;

#[cfg(test)]
mod tests;

use clap::CommandFactory;
use std::{env::args_os, ffi::OsString, process::exit};

use crate::{
    cli::{ArgsBackup, ArgsCreate, ArgsVerify, Cli},
    commands::{CommandExecutor, backup::Backup, create::Create, verify::Verify, version::Version},
    config::Config,
};

pub struct Dependencies {
    backup: Box<dyn CommandExecutor<ArgsBackup>>,
    create: Box<dyn CommandExecutor<ArgsCreate>>,
    verify: Box<dyn CommandExecutor<ArgsVerify>>,
    version: Box<dyn CommandExecutor<()>>,
    config: Config,
}

pub fn main() {
    let config_file = &Config::get_config_path();
    if let Err(err) = match Config::from_file(config_file) {
        Ok(cfg) => cli::parse(
            &Dependencies {
                backup: Box::new(Backup {}),
                create: Box::new(Create {}),
                verify: Box::new(Verify {}),
                version: Box::new(Version {}),
                config: cfg,
            },
            args_os().collect::<Vec<OsString>>(),
        ),
        Err(err) => Err(Cli::command().error(
            clap::error::ErrorKind::Io,
            format!("{} {}", err.to_string(), config_file.display()),
        )),
    } {
        err.exit();
    }
}
