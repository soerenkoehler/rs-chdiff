use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    bin_name = "chdiff",
    about,
    version,
    disable_version_flag = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Command>,
    #[arg(long, help = "Print version")]
    pub version: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(visible_alias = "c")]
    Create(ArgsCreate),
    #[command(visible_alias = "v")]
    Verify(ArgsVerify),
    #[command(visible_alias = "b")]
    Backup(ArgsBackup),
}

#[derive(ValueEnum, Clone, Debug)]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
}

#[derive(Args, Debug)]
pub struct ArgsBackup {
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub struct ArgsCreate {
    #[arg(default_value = ".")]
    pub path: PathBuf,
    #[arg(short, long, value_enum, ignore_case = true, default_value = "sha256")]
    pub algorithm: HashAlgorithm,
}

#[derive(Args, Debug)]
pub struct ArgsVerify {
    #[arg(default_value = ".")]
    pub path: PathBuf,
}
