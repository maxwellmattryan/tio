use iota_client::{
    bee_message::{payload::Payload, prelude::IndexationPayload, MessageId},
    Client,
};

use crate::error::{Error, Result};

/// The types of available IOTA networks.
#[derive(Debug, PartialEq)]
pub enum Network {
    Mainnet,
    Devnet,
}

impl Network {
    pub fn url(&self) -> &str {
        match *self {
            Network::Mainnet => "https://chrysalis-nodes.iota.org",
            Network::Devnet => "https://api.lb-0.h.chrysalis-devnet.iota.cafe",
        }
    }
}

fn try_network_from_str(arg: &str) -> Result<Network> {
    match arg {
        "m" | "mainnet" => Ok(Network::Mainnet),
        "d" | "devnet" => Ok(Network::Devnet),
        _ => Err(Error::NetworkInvalid(arg.to_string())),
    }
}

/// Arguments for client configuration.
#[derive(Debug, structopt::StructOpt)]
pub struct ClientArgs {
    /// IOTA Tangle network to use ("mainnet" and "devnet").
    #[structopt(short, long, parse(try_from_str=try_network_from_str))]
    pub network: Option<Network>,
}

impl ClientArgs {
    pub fn unpack_network(&self) -> &Network {
        match self.network {
            Some(ref n) => n,
            None => &Network::Devnet,
        }
    }
}

async fn build_client(network: &Network) -> Client {
    match Client::builder().with_node(network.url()).unwrap().finish().await {
        Ok(c) => c,
        Err(_) => panic!("{:?}", Error::CannotBuildNodeClient),
    }
}

/// Broadcast a message with given data to a specific IOTA network.
pub async fn broadcast(data: &String, data_index: &String, network: &Network) {
    let size = data.as_bytes().len();
    println!(
        "CONTENT: \"{}\"\nINDEX: \"{}\"\nSIZE: {} byte(s)\n",
        data, data_index, size
    );

    let iota = build_client(network).await;
    let m = match iota
        .message()
        .with_index(data_index)
        .with_data(data.as_bytes().to_vec())
        .finish()
        .await
    {
        Ok(m) => m,
        Err(_) => panic!("{:?}", Error::CannotBroadcastMessage),
    };

    println!("HASH: {}\n", m.id().0);
}

/// Search for a message on a specified IOTA network given its hash ID.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_network_from_str() {
        let mainnet = Network::Mainnet;
        let devnet = Network::Devnet;
        let error = Error::NetworkInvalid(String::from(""));

        assert_eq!(mainnet, try_network_from_str("m").unwrap());
        assert_eq!(mainnet, try_network_from_str("mainnet").unwrap());

        assert_eq!(devnet, try_network_from_str("d").unwrap());
        assert_eq!(devnet, try_network_from_str("devnet").unwrap());

        assert_eq!(error, try_network_from_str("").unwrap_err());
    }
}
