use clap::Error;

use super::CommandExecutor;
use crate::{Dependencies, cli::ArgsCreate};

pub struct Create {}

impl CommandExecutor<ArgsCreate> for Create {
    fn execute(&self, _deps: &Dependencies, args: ArgsCreate) -> Result<(), Error> {
        println!("create (wip) {:?}", args);
        Ok(())
    }
}
