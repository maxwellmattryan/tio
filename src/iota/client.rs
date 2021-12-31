use std::str::FromStr;

use rand::Rng;
use url::Url;

use iota_client::Client;

use crate::error::{Error, Result};

/// The types of available IOTA networks.
#[derive(Debug, PartialEq)]
pub enum Network {
    ChrysalisMainnet,
    ChrysalisDevnet,
}

impl Network {
    fn urls(&self) -> [&str; 3] {
        match *self {
            Network::ChrysalisMainnet => [
                "https://chrysalis-nodes.iota.org",
                "https://chrysalis-nodes.iota.cafe",
                "https://mainnet-node.tanglebay.com",
            ],
            Network::ChrysalisDevnet => [
                "https://api.lb-0.h.chrysalis-devnet.iota.cafe",
                "https://api.lb-1.h.chrysalis-devnet.iota.cafe",
                "https://api.lb-1.h.chrysalis-devnet.iota.cafe",
            ],
        }
    }

    pub fn url(&self) -> &str {
        let rand_idx: usize = rand::thread_rng().gen_range(0..3);
        self.urls()[rand_idx]
    }
}

fn try_network_from_str(arg: &str) -> Result<Network> {
    match arg {
        "m" | "mainnet" => Ok(Network::ChrysalisMainnet),
        "d" | "devnet" => Ok(Network::ChrysalisDevnet),
        _ => Err(Error::NetworkInvalid(arg.to_string())),
    }
}

fn try_url_from_str(arg: &str) -> Result<Url> {
    match Url::from_str(arg) {
        Ok(s) => Ok(s),
        Err(_) => Err(Error::CannotParseNodeUrl),
    }
}

/// Arguments for network client configuration.
#[derive(Debug, structopt::StructOpt)]
pub struct ClientArgs {
    /// IOTA Tangle network to use ("mainnet" and "devnet").
    #[structopt(short, long, parse(try_from_str=try_network_from_str))]
    pub network: Option<Network>,

    /// Particular node URL to send API requests to.
    #[structopt(short, long, parse(try_from_str=try_url_from_str))]
    pub url: Option<Url>,
}

impl ClientArgs {
    pub fn unpack_network(&self) -> &Network {
        match self.network {
            Some(ref n) => n,
            None => &Network::ChrysalisDevnet,
        }
    }

    pub fn unpack_url(&self) -> &str {
        match self.url {
            Some(ref u) => u.as_str(),
            None => match self.network {
                Some(ref n) => n.url(),
                None => Network::ChrysalisDevnet.url(),
            }
        }
    }
}

pub(crate) async fn build_client(node_url: &str) -> Client {
    let client = Client::builder().with_node(node_url).unwrap().finish().await;
    match client {
        Ok(c) => c,
        Err(_) => panic!("{:?}", Error::CannotBuildNodeClient),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_network_from_str() {
        let mainnet = Network::ChrysalisMainnet;
        let devnet = Network::ChrysalisDevnet;
        let error = Error::NetworkInvalid(String::from(""));

        assert_eq!(mainnet, try_network_from_str("m").unwrap());
        assert_eq!(mainnet, try_network_from_str("mainnet").unwrap());

        assert_eq!(devnet, try_network_from_str("d").unwrap());
        assert_eq!(devnet, try_network_from_str("devnet").unwrap());

        assert_eq!(error, try_network_from_str("").unwrap_err());
    }
}
