use crate::commands::CommandExecutor;
use clap::{Error, crate_name, crate_version};

pub struct Version {}

impl CommandExecutor<()> for Version {
    fn execute(&self, _: &crate::Dependencies, _: ()) -> Result<(), Error> {
        println!("{} {}", crate_name!(), crate_version!());
        Ok(())
    }
}
