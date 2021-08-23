use std::convert::TryInto;

use async_trait::async_trait;
use hex::decode;

use crate::{
    cli::{Command},
    error::{Error, Result},
    iota::{ClientArgs, init, search, Network},
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

/// Arguments for the `search` command
#[derive(Debug, structopt::StructOpt)]
pub struct SearchArgs {
    /// Hash of a message (must be hexadecimal string of exactly 32 bytes)
    #[structopt(parse(try_from_str=try_hash_from_str))]
    pub hash: String,
}

impl SearchArgs {
    pub fn unpack_hash(&self) -> [u8; 32] {
        let boxed = decode(&self.hash).unwrap().into_boxed_slice();
        let boxed_arr: Box<[u8; 32]> = match boxed.try_into() {
            Ok(ba) => ba,
            Err(_) => panic!("{:?}", Error::MessageHashInvalid(self.hash.clone())),
        };
        
        *boxed_arr
    }
}

/// `search` subcommand that finds a message on the IOTA Tangle
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
        let n: &Network = self.client.unpack_network();
        init(n).await;

        let h: &[u8; 32] = &self.search.unpack_hash();
        Ok(search(h, n).await)
    }
}
