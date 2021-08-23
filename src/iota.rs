use iota_client::{
    Client, bee_message::{
        MessageId, payload::Payload, prelude::IndexationPayload
    }
};

use crate::{
    error::{Error, Result},
};

/// The constant value for `tio`'s data indexation
const TIO_INDEX: &str = "TIO_INDEX_TEST";

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

/// Arguments for client configuration
#[derive(Debug, structopt::StructOpt)]
pub struct ClientArgs {
    /// IOTA Tangle network to use ("mainnet", "devnet", and "testnet")
    #[structopt(short, long, parse(try_from_str=try_network_from_str))]
    pub network: Option<Network>,
}

impl ClientArgs {
    pub fn unpack_network(&self) -> &Network {
        match self.network {
            Some(ref n) => n,
            None => &Network::Testnet, // Change back to `Mainnet` after first "release"
        }
    }
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

/// Initialize a client <-> node connection
pub async fn init(network: &Network) {
    let iota = build_client(network).await;
    match iota.get_info().await {
        Ok(i) => println!("NODE_INFO: {:#?}\n", &i.nodeinfo),
        Err(_) => panic!("{:?}", Error::CannotGetNodeInfo),
    }
}

/// Broadcast a message with given data to a specific IOTA network 
pub async fn broadcast(data: &String, network: &Network) {
    let size = data.as_bytes().len();
    println!("CONTENT: \"{}\"\nSIZE: {} byte(s)\n", data, size);

    let iota = build_client(network).await;
    let m = match iota
        .message()
        .with_index(TIO_INDEX)
        .with_data(data.as_bytes().to_vec())
        .finish()
        .await {
            Ok(m) => m,
            Err(_) => panic!("{:?}", Error::CannotBroadcastMessage),
        };

    println!("HASH: {}\n", m.id().0);
}

/// Search for a message on a specified IOTA network given its hash ID
pub async fn search(hash: &[u8; 32], network: &Network) {
    let id = MessageId::new(*hash);
    let iota = build_client(network).await;

    let message = match iota.get_message().data(&id).await {
        Ok(m) => m,
        Err(_) => panic!("{:?}", Error::MessageNotFound),
    };
    let payload: &Payload = match message.payload() {
        Some(p) => p,
        None => panic!("{:?}", Error::MessageEmpty),
    };
    let data: &IndexationPayload = match payload {
        Payload::Indexation(p) => p.as_ref(),
        _ => panic!("{:?}", Error::MessageWrongPayload),
    };
    let string = match String::from_utf8(data.data().iter().cloned().collect()) {
        Ok(s) => s,
        Err(_) => panic!("{:?}", Error::MessageDataInvalid),
    };

    let size = string.as_bytes().len();
    println!("CONTENT: {:#?}\nSIZE: {} byte(s)\n", string, size);
}
