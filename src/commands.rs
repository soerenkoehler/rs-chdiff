pub(crate) mod backup;
pub(crate) mod create;
pub(crate) mod verify;

#[cfg(test)]
mod commands_test;

use std::fmt::Debug;

pub(crate) trait CommandExecutor {
    fn execute<T: Debug>(&self, args: T);
}
