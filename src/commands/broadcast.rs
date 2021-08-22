use async_trait::async_trait;

use crate::{
    cli::{Command}, 
    error::{Error, Result}, 
    iota::{ClientArgs, broadcast, init, Network},
};

fn try_message_from_str(arg: &str) -> Result<String> {
    let message = arg.to_string();
    let size = message.as_bytes().len();

    match size {
        s if s < 4096 => Ok(message),
        _ => Err(Error::MessageTooLarge("message".to_string(), size)),
    }
}

/// Arguments for the `broadcast` subcommand
#[derive(Debug, structopt::StructOpt)]
pub struct BroadcastArgs {
    /// Data to embed in message payload (must be < 4kb and UTF-8 encoded)
    #[structopt(
        parse(try_from_str=try_message_from_str)
    )]
    pub message: Option<String>,
}

/// `broadcast` subcommand that sends messages on the IOTA Tangle
#[derive(structopt::StructOpt)]
pub struct BroadcastCommand {
    #[structopt(flatten)]
    pub broadcast: BroadcastArgs,

    #[structopt(flatten)]
    pub client: ClientArgs,
}

#[async_trait]
impl Command for BroadcastCommand {
    async fn run(self) -> Result<()> {
        let network = match self.client.network {
            Some(ref n) => n,
            None => &Network::Devnet, // CHANGE BACK TO `Mainnet` later!
        };
        init(&network).await;

        let message = match self.broadcast.message {
            Some(m) => m,
            None => String::from("TIO_MESSAGE"),
        };
        broadcast(&message, &network).await;

        Ok(())
    }
}
