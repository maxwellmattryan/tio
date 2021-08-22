use iota_client::Client;

use crate::{
    error::{Error, Result},
};

/// The types of available IOTA networks
#[derive(Debug)]
pub enum Network {
    Devnet,
    Mainnet,
    Testnet,
}

impl Network {
    pub fn url(&self) -> &str {
        match *self {
            Network::Devnet => "https://api.lb-0.h.chrysalis-devnet.iota.cafe",
            Network::Mainnet => "https://chrysalis-nodes.iota.org",
            Network::Testnet => "https://api.lb-0.testnet.chrysalis2.com",
        }
    }
}

fn try_network_from_str(arg: &str) -> Result<Network> {
    match arg {
        "d" | "devnet" => Ok(Network::Devnet),
        "m" | "mainnet" => Ok(Network::Mainnet),
        "t" | "testnet" => Ok(Network::Testnet),
        _ => Err(Error::NetworkInvalid(arg.to_string())),
    }
}

#[derive(Debug, structopt::StructOpt)]
pub struct ClientArgs {
    /// Particular IOTA network to broadcast to ("mainnet", "devnet", and "testnet")
    #[structopt(short, long, parse(try_from_str=try_network_from_str))]
    pub network: Option<Network>,
}

async fn build_client(network: &Network) -> Client {
    match Client::builder()
        .with_node(network.url())
        .unwrap()
        .finish()
        .await {
            Ok(c) => c,
            Err(_) => panic!("{:?}", Error::CannotBuildNodeClient) ,
        }
}

pub async fn init(network: &Network) {
    let iota = build_client(network).await;
    match iota.get_info().await {
        Ok(i) => println!("NODE_INFO: {:#?}\n", &i.nodeinfo),
        Err(_) => panic!("{:?}", Error::CannotGetNodeInfo),
    }
}

pub async fn broadcast(message: &String, network: &Network) {
    let size = message.as_bytes().len();
    println!("CONTENT: {}\nSIZE: {} byte(s)\n", message, size);

    let iota = build_client(network).await;
    let m = match iota
        .message()
        .with_index("TIO_DATA")
        .with_data(message.as_bytes().to_vec())
        .finish()
        .await {
            Ok(m) => m,
            Err(_) => panic!("{:?}", Error::CannotBroadcastMessage),
        };

    println!("HASH: {}\n", m.id().0);
}
