use std::
    env::args_os
;

use commands::{backup::Backup, create::Create, verify::Verify};

mod cli;
mod commands;
mod config;
mod digest;

pub fn main() {
    cli::parse(args_os(), &Backup {}, &Create {}, &Verify {});
}
