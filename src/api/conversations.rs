use serde_json;

use error::HelpScoutError;
use client::Client;
use envelope::Collection;

#[derive(Debug, Deserialize)]
pub struct Conversation {
    id: i32,
}

pub fn list(client: &Client, mailbox_id: i32) -> Result<Collection<Conversation>, HelpScoutError> {
    let res = client.get(&format!("mailboxes/{}/conversations.json", mailbox_id), None)?;

    println!("result - {:?}", res);
    let conversations = serde_json::from_value(res.clone())?;

    Ok(conversations)
}
