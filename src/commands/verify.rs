use std::path::Path;

use crate::cli::ArgsVerify;
use crate::digest::Digest;

pub(crate) fn run(args: ArgsVerify) {
    println!("verify (wip) {:?}", args);

    let _digest = Digest::from_dir(Path::new(&args.path));
}
