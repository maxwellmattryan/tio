use async_trait::async_trait;

use crate::{
    cli::{Command}, 
    error::{Error, Result},
    iota::{ClientArgs, broadcast, init, Network},
};

/// The maximum number of bytes allowed for a data message's payload.
pub const MAX_DATA_BYTES: usize = 4096;

/// The maximum number of bytes allowed for a data message's index.
pub const MAX_DATA_INDEX_BYTES: usize = 64;

fn try_data_from_str(arg: &str) -> Result<String> {
    let data = arg.to_string();
    let size = data.as_bytes().len();

    match size {
        s if s < MAX_DATA_BYTES => Ok(data),
        _ => Err(Error::MessageDataTooLarge(size)),
    }
}

fn try_data_index_from_str(arg: &str) -> Result<String> {
    let data_index = arg.to_string();
    let size = data_index.as_bytes().len();

    match size {
        s if s < MAX_DATA_INDEX_BYTES => Ok(data_index),
        _ => Err(Error::MessageDataIndexTooLarge(size))
    }
}

/// Arguments for the `broadcast` subcommand.
#[derive(Debug, structopt::StructOpt)]
pub struct BroadcastArgs {
    /// Data to use as the message payload (must be < 4kb and UTF-8 encoded).
    #[structopt(parse(try_from_str=try_data_from_str))]
    pub data: Option<String>,

    /// Data index to use in placing the data.
    #[structopt(parse(try_from_str=try_data_index_from_str))]
    pub data_index: Option<String>,
}

impl BroadcastArgs {
    pub fn unpack_args(&self) -> (String, String) {
        let data = match &self.data {
            Some(d) => d.clone(),
            None => String::from("TIO_MESSAGE"),
        };
        let data_index = match &self.data_index {
            Some(di) => di.clone(),
            None => String::from("TIO_DATA")
        };

        (data, data_index)
    }
}

/// `broadcast` subcommand that sends messages to the IOTA Tangle.
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
        let network: &Network = self.client.unpack_network();
        init(network).await;

        let (data, data_index) = &self.broadcast.unpack_args();
        Ok(broadcast(data, data_index, network).await)
    }
}
