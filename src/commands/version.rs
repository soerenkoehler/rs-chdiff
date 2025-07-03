use crate::commands::CommandExecutor;
use clap::{crate_name, crate_version};

pub struct Version {}

impl CommandExecutor<()> for Version {
    fn execute(&self, _: &crate::Dependencies, _: ()) {
        println!("{} {}", crate_name!(), crate_version!());
    }
}
