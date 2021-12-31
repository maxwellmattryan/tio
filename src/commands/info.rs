use async_trait::async_trait;

use crate::{
    cli::Command,
    error::{Result},
    iota::{
        client::{
            ClientArgs,
            Network,
        },
    },
};
use crate::iota::get_info;

/// `info` subcommand that queries for node information.
#[derive(structopt::StructOpt)]
pub struct InfoCommand {
    #[structopt(flatten)]
    pub client: ClientArgs,
}

#[async_trait]
impl Command for InfoCommand {
    async fn run(&self) -> Result<()> {
        let network: &Network = self.client.unpack_network();

        Ok(get_info(network).await)
    }
}
