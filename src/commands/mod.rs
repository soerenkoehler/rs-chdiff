pub mod backup;
pub mod create;
pub mod verify;
pub mod version;

use std::fmt::Debug;

use crate::Dependencies;

use clap::Error;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait CommandExecutor<T: Debug> {
    fn execute(&self, deps: &Dependencies, args: T) -> Result<(), Error>;
}
