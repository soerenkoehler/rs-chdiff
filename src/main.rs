mod cli;
mod commands;
mod config;
mod digest;
mod filelist;

use std::env::args_os;

use cli::{ArgsBackup, ArgsCreate, ArgsVerify};
use commands::{CommandExecutor, backup::Backup, create::Create, verify::Verify};
use config::Config;

struct Dependencies {
    backup: Box<dyn CommandExecutor<ArgsBackup>>,
    create: Box<dyn CommandExecutor<ArgsCreate>>,
    verify: Box<dyn CommandExecutor<ArgsVerify>>,
    config: Config,
}

pub fn main() {
    cli::parse(
        &Dependencies {
            backup: Box::new(Backup {}),
            create: Box::new(Create {}),
            verify: Box::new(Verify {}),
            config: Config::from_file(&Config::get_config_path()),
        },
        args_os(),
    );
}
