use std::path::Path;

use crate::cli::ArgsVerify;
use crate::digest::Digest;

use super::ExecutableCommand;

impl ExecutableCommand for ArgsVerify {
    fn execute(&self) {
        println!("verify (wip) {:?}", self);

        let _digest = Digest::from_dir(Path::new(&self.path));
    }
}
