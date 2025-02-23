use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(about, version, long_version = "Y")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create(CreateArgs),
    Verify(VerifyArgs),
}

#[derive(Args)]
struct VerifyArgs {}

#[derive(Args)]
struct CreateArgs {}

fn main() {
    let _ = Cli::parse();
}
