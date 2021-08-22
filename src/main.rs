use structopt::StructOpt;

use tio::cli::Command;

#[tokio::main]
async fn main() -> Result<(), String> {
    tio::cli::Cli::from_args().run()
}
