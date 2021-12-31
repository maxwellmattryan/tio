use async_trait::async_trait;

use crate::{
    cli::Command,
    error::{Error, Result},
    iota::{
        broadcast_message,
        client::{ClientArgs},
    },
};

/// The maximum number of bytes allowed for a data message's index.
pub const MAX_INDEX_BYTES: usize = 64;

/// The maximum number of bytes allowed for a data message's payload.
pub const MAX_DATA_BYTES: usize = 4096;

fn try_data_index_from_str(arg: &str) -> Result<String> {
    let index = arg.to_string();
    let size = index.as_bytes().len();

    match size {
        s if s < MAX_INDEX_BYTES => Ok(index),
        _ => Err(Error::MessageDataIndexTooLarge(size)),
    }
}

fn try_data_from_str(arg: &str) -> Result<String> {
    let data = arg.to_string();
    let size = data.as_bytes().len();

    match size {
        s if s < MAX_DATA_BYTES => Ok(data),
        _ => Err(Error::MessageDataTooLarge(size)),
    }
}

/// Arguments for the `broadcast` subcommand.
#[derive(Debug, structopt::StructOpt)]
pub struct BroadcastArgs {
    /// Indexation key used in the IOTA Tangle.
    #[structopt(parse(try_from_str=try_data_index_from_str))]
    pub index: Option<String>,

    /// UTF-8 encoded data embedded inside the indexation payload.
    #[structopt(parse(try_from_str=try_data_from_str))]
    pub data: Option<String>,
}

impl BroadcastArgs {
    pub fn unpack_args(&self) -> (&str, &str) {
        let index = match &self.index {
            Some(i) => i.as_str(),
            None => "tio-cli",
        };
        let data = match &self.data {
            Some(d) => d.as_str(),
            None => "tio-message",
        };

        (index, data)
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
        let node_url = self.client.unpack_url();
        println!("BROADCAST URL: {}", node_url);
        let (index, data) = self.broadcast.unpack_args();

        Ok(broadcast_message(index, data, node_url).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_index_from_str() {
        let valid_index: &str = "This is a valid index.";
        assert_eq!(String::from(valid_index), try_data_index_from_str(valid_index).unwrap());
    }

    #[test]
    fn test_try_data_from_str() {
        let valid_data: &str = "This is valid data.";
        assert_eq!(String::from(valid_data), try_data_from_str(valid_data).unwrap());
    }
}
