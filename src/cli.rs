use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about, version, long_version = "Y")]
pub(crate) struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(visible_alias = "c")]
    Create(ArgsCreate),
    #[command(visible_alias = "v")]
    Verify(ArgsVerify),
    #[command(visible_alias = "b")]
    Backup(ArgsBackup),
}

#[derive(Args, Debug)]
struct ArgsCreate {
    #[arg(default_value = ".")]
    path: String,
}

#[derive(Args, Debug)]
struct ArgsVerify {
    #[arg(default_value = ".")]
    path: String,
}

#[derive(Args, Debug)]
struct ArgsBackup {
    #[arg(default_value = ".")]
    path: String,
}
