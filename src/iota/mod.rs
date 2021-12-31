use chrono::{DateTime, NaiveDateTime, Utc};
use iota_client::bee_message::{payload::Payload, prelude::IndexationPayload, MessageId};

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

    let size = data.as_bytes().len();
    println!(
        "--- Broadcast Result ---\n\
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

/// Search for a message on a specified IOTA network given its hash ID.
pub async fn find_message(message_id: &[u8; 32], node_url: &str) {
    let id = MessageId::new(*message_id);
    let iota = build_client(node_url).await;

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
    let index = match String::from_utf8(data.index().iter().cloned().collect()) {
        Ok(s) => s,
        Err(_) => panic!("{:?}", Error::MessageDataIndexInvalid),
    };
    let string = match String::from_utf8(data.data().iter().cloned().collect()) {
        Ok(s) => s,
        Err(_) => panic!("{:?}", Error::MessageDataInvalid),
    };

    let size = string.as_bytes().len();
    println!(
        "--- Search Result ---\n\
        Index: {}\n\
        Data: {}\n\
        Size: {} byte(s)",
        index, string, size,
    )
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
