use std::convert::TryInto;

use async_trait::async_trait;
use hex::decode;

use crate::{
    cli::Command,
    error::{Error, Result},
    iota::{search, ClientArgs, Network},
};

fn try_hash_from_str(arg: &str) -> Result<String> {
    let bytes: Vec<u8> = match decode(arg) {
        Ok(b) => b,
        Err(_) => return Err(Error::MessageHashInvalid(arg.to_string())),
    };

    match bytes.len() {
        32 => Ok(arg.to_string()),
        _ => Err(Error::MessageHashInvalid(arg.to_string())),
    }
}

/// Arguments for the `search` command.
#[derive(Debug, structopt::StructOpt)]
pub struct SearchArgs {
    /// Hash of a message (must be hexadecimal string of exactly 32 bytes).
    #[structopt(parse(try_from_str=try_hash_from_str))]
    pub id: String,
}

impl SearchArgs {
    pub fn unpack_hash(&self) -> [u8; 32] {
        let boxed = decode(&self.id).unwrap().into_boxed_slice();
        let boxed_arr: Box<[u8; 32]> = match boxed.try_into() {
            Ok(ba) => ba,
            Err(_) => panic!("{:?}", Error::MessageHashInvalid(self.id.clone())),
        };

        *boxed_arr
    }
}

/// `search` subcommand that finds a message on the IOTA Tangle.
#[derive(structopt::StructOpt)]
pub struct SearchCommand {
    #[structopt(flatten)]
    pub search: SearchArgs,

    #[structopt(flatten)]
    pub client: ClientArgs,
}

#[async_trait]
impl Command for SearchCommand {
    async fn run(&self) -> Result<()> {
        let network: &Network = self.client.unpack_network();

        let id: &[u8; 32] = &self.search.unpack_hash();
        Ok(search(id, network).await)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_hash_from_str() {
        fn error_fn(s: &str) -> Error {
            Error::MessageHashInvalid(String::from(s))
        }
        let bad_id = "THIS_IS_A_BAD_HASH";
        let half_id = "9d097abc7abef5c51f31a33655f3f15e";
        let good_id = "9d097abc7abef5c51f31a33655f3f15e100d4634f930a07ebbcfe3f0ab98b620";

        assert_eq!(error_fn(bad_id), try_hash_from_str(bad_id).unwrap_err());
        assert_eq!(error_fn(half_id), try_hash_from_str(half_id).unwrap_err());
        assert_eq!(good_id, try_hash_from_str(good_id).unwrap());
    }
}
