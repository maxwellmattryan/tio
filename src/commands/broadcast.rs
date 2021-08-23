use async_trait::async_trait;

use crate::{
    cli::{Command}, 
    error::{Error, Result}, 
    iota::{ClientArgs, broadcast, init, Network},
};

fn try_data_from_str(arg: &str) -> Result<String> {
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
    /// Data to use as the message payload (must be < 4kb and UTF-8 encoded)
    #[structopt(parse(try_from_str=try_data_from_str))]
    pub data: Option<String>,
}

impl BroadcastArgs {
    pub fn unpack_data(&self) -> String {
        match self.data {
            Some(ref m) => m.to_string(),
            None => "TIO_MESSAGE".to_string(),
        }
    }
}

/// `broadcast` subcommand that sends messages to the IOTA Tangle
#[derive(structopt::StructOpt)]
pub struct BroadcastCommand {
    #[structopt(flatten)]
    pub broadcast: BroadcastArgs,

    #[structopt(flatten)]
    pub client: ClientArgs,
}

#[async_trait]
impl Command for BroadcastCommand {
    async fn run(&self) -> Result<()> {
        let n: &Network = self.client.unpack_network();
        init(n).await;

        let d: &String = &self.broadcast.unpack_data();
        Ok(broadcast(d, n).await)
    }
}
