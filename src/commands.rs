pub(crate) mod backup;
pub(crate) mod create;
pub(crate) mod verify;

#[cfg(test)]
use mockall::automock;

use std::fmt::Debug;

use crate::Dependencies;

#[cfg_attr(test, automock)]
pub(crate) trait CommandExecutor<T: Debug> {
    fn execute(&self, deps:&Dependencies, args: T);
}
