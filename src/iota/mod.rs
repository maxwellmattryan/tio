use chrono::{DateTime, NaiveDateTime, Utc};
use iota_client::bee_message::{
    address::Address,
    input::Input,
    output::Output,
    payload::Payload,
    prelude::{Essence, IndexationPayload, TransactionPayload},
    Message, MessageId,
};

use crate::{error::Error, iota::client::build_client};

pub mod client;

/// Broadcast a message with given data to a specific IOTA network.
pub async fn broadcast_message(index: &str, data: &str, node_url: &str) {
    let iota = build_client(node_url).await;
    let m = match iota
        .message()
        .with_index(index)
        .with_data(data.as_bytes().to_vec())
        .finish()
        .await
    {
        Ok(m) => m,
        Err(_) => panic!("{:?}", Error::CannotBroadcastMessage),
    };

    let size = index.as_bytes().len() + data.as_bytes().len();
    println!(
        "--- Data Message ---\n\
        ID: {}\n\
        Index: {}\n\
        Data: {}\n\
        Size: {} byte(s)",
        m.id().0,
        index,
        data,
        size,
    )
}

fn print_message_payload(payload: &Payload) {
    match payload {
        Payload::Indexation(p) => unsafe {
            let data_payload: &IndexationPayload = p.as_ref();

            let index = match String::from_utf8(data_payload.index().iter().cloned().collect()) {
                Ok(s) => s,
                Err(_) => panic!("{:?}", Error::MessageDataIndexInvalid),
            };
            let data = match String::from_utf8(data_payload.data().iter().cloned().collect()) {
                Ok(s) => s,
                Err(_) => String::from_utf8_unchecked(data_payload.data().iter().cloned().collect()),
            };
            let size = index.as_bytes().len() + data.as_bytes().len();

            println!(
                "--- Data Payload ---\n\
                Index: {}\n\
                Data: {}\n\
                Size: {} byte(s)",
                index, data, size,
            )
        },
        Payload::Transaction(p) => {
            println!("--- UTXO Payload ---");

            let tx: &TransactionPayload = p.as_ref();
            match tx.essence() {
                Essence::Regular(e) => {
                    if e.inputs().len() > 0 {
                        println!("Input(s):");
                        for input in e.inputs() {
                            match input {
                                Input::Utxo(i) => {
                                    println!("{}", i.to_string());
                                }
                                _ => (),
                            }
                        }
                    }

                    if e.outputs().len() > 0 {
                        println!("\nOutput(s):");
                        for output in e.outputs() {
                            match output {
                                Output::SignatureLockedSingle(sls) => match sls.address() {
                                    Address::Ed25519(a) => {
                                        println!(
                                            "Address: {}, Type: {}, Amount: {}i",
                                            a.to_string(),
                                            "Ed25519".to_string(),
                                            sls.amount()
                                        );
                                    }
                                },
                                _ => (),
                            }
                        }
                    }

                    match e.payload() {
                        Some(payload) => {
                            println!();
                            print_message_payload(payload)
                        }
                        None => (),
                    }
                }
            }
        }
        _ => panic!("{:?}", Error::MessageWrongPayload),
    }
}

/// Search for a message on a specified IOTA network given its hash ID.
pub async fn find_message(message_id: &[u8; 32], node_url: &str) {
    let iota = build_client(node_url).await;
    let id = MessageId::new(*message_id);
    let message: Message = match iota.get_message().data(&id).await {
        Ok(m) => m,
        Err(_) => panic!("{:?}", Error::MessageNotFound),
    };
    let payload: &Payload = match message.payload() {
        Some(p) => p,
        None => panic!("{:?}", Error::MessageEmpty),
    };

    print_message_payload(payload)
}

/// Query a node for its network information.
pub async fn get_info(node_url: &str) {
    let iota = build_client(node_url).await;

    let network_info = match iota.get_network_info().await {
        Ok(ni) => ni,
        Err(_) => panic!("{:?}", Error::CannotGetNetworkInfo),
    };
    let node_info = match iota.get_info().await {
        Ok(ni) => ni,
        Err(_) => panic!("{:?}", Error::CannotGetNodeInfo),
    };

    let timestamp = node_info
        .nodeinfo
        .latest_milestone_timestamp
        .to_string()
        .as_str()
        .parse::<i64>()
        .unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);

    println!(
        "--- Network Info ---\n\
        ID: {}\n\
        Bech32 HRP: {}\n\
        \n--- Node Info ---\n\
        URL: {}\n\
        Software: {} {}\n\
        Stats: {:.1} MPS @ {:.2}%\n\
        Milestones: No. {} @ {}",
        network_info.network_id.unwrap(),
        network_info.bech32_hrp,
        node_info.url,
        node_info.nodeinfo.name,
        node_info.nodeinfo.version,
        node_info.nodeinfo.referenced_messages_per_second,
        node_info.nodeinfo.referenced_rate,
        node_info.nodeinfo.latest_milestone_index,
        datetime.format("%Y-%m-%d %H:%M:%S"),
    );
}
