use std::path::Path;

use crate::cli::ArgsVerify;
use crate::digest::Digest;

use super::ExecuteCommand;

impl ExecuteCommand for ArgsVerify {
    fn execute(&self) {
        println!("verify (wip) {:?}", self);

        let _digest = Digest::from_dir(Path::new(&self.path));
    }
}
