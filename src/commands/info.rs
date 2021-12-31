use async_trait::async_trait;

use crate::{
    cli::Command,
    error::Result,
    iota::{
        client::{ClientArgs},
        get_info,
    },
};

/// `info` subcommand that queries for node information.
#[derive(structopt::StructOpt)]
pub struct InfoCommand {
    #[structopt(flatten)]
    pub client: ClientArgs,
}

#[async_trait]
impl Command for InfoCommand {
    async fn run(&self) -> Result<()> {
        let node_url = self.client.unpack_url();
        println!("INFO URL: {}", node_url);

        Ok(get_info(node_url).await)
    }
}
