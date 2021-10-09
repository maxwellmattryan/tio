use structopt::StructOpt;

use tio::{
    cli::Command,
    error::{Error, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    match tio::cli::Cli::from_args().run().await {
        Ok(_) => Ok(()),
        Err(_) => panic!("{:?}", Error::Generic),
    }
}
