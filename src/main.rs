mod backup;
mod cli;
mod create;
mod verify;

pub fn main() {
    cli::parse();
}
