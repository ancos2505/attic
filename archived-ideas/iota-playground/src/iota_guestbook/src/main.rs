mod model;

use iota_client::{bee_message::payload::Payload, Client, Seed};
use std::env;

use crate::model::GuestbookEntry;
type AppResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> AppResult<()> {
    let iota = Client::builder()
        .with_node("https://api.thin-hornet-0.h.chrysalis-devnet.iota.cafe") // Use the devnet for testing
        .unwrap()
        .finish()
        .await
        .unwrap();

    let iota_seed_str = env::var("IOTA_SEED").expect("IOTA_SEED must be set");
    let seed = Seed::from_bytes(iota_seed_str.as_bytes());

    // Add a new entry
    let entry = GuestbookEntry {
        name: "Your Name".to_string(),
        message: "Hello, IOTA!".to_string(),
    };
    let message = iota
        .message()
        .with_seed(&seed)
        .with_output(
            "atoi1qzvhzy5pn58576356vdhja494ql54d554jqcpf5fy4aueuhr50emq3p",
            0,
        )? // Output to a dummy address
        .with_index("GUESTBOOK")
        .with_data(serde_json::to_string(&entry)?.into())
        .finish()
        .await
        .unwrap();

    println!("Message ID: {}", message.id().0);

    let indexation_keys = ["GUESTBOOK"];
    let message_ids = [];
    // Retrieve entries
    let messages = iota
        .find_messages(&indexation_keys, &message_ids)
        .await
        .unwrap();

    for message in messages {
        let payload: &Payload = message
            .payload()
            .as_ref()
            .ok_or(anyhow::Error::msg("Payload not found"))?;
        let payload_json = serde_json::to_string(payload)?;
        let entry: GuestbookEntry = serde_json::from_str(&payload_json)?;
        println!("{}: {}", entry.name, entry.message);
    }
    Ok(())
}
