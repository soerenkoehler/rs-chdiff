use std::{
    env::args_os,
    io::{Error, ErrorKind},
};

use commands::{backup::Backup, create::Create, verify::Verify};
use config::Config;

mod cli;
mod commands;
mod config;
mod digest;

pub fn main() {
    // TODO
    if let Err(e) = Config::from_file(Config::get_config_path()) {
        match e.downcast::<Error>() {
            Ok(ref e) => match e.kind() {
                ErrorKind::NotFound => println!("file not found: {}", e.to_string()),
                _ => println!("{:?}", e),
            },
            Err(e) => println!("{:?}", e),
        }
    };
    cli::parse(args_os(), &Backup {}, &Create {}, &Verify {});
}
