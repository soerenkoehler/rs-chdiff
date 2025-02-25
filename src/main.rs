mod cli;

use cli::Cli;
use clap::Parser;

fn main() {
    println!("{:?}", Cli::parse());
}
