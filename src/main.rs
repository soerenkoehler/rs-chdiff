use std::env::args_os;

mod cli;
mod commands;
mod digest;

pub fn main() {
    cli::parse(args_os());
}
