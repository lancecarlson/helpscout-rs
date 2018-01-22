use serde_json;

use error::HelpScoutError;
use client::{Client, Status};

const PREFIX: &'static str = "mailboxes";

#[derive(Debug, Deserialize)]
pub struct ItemList<T> {
    page: i32,
    pages: i32,
    count: i32,
    items: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct Conversation {
    id: String,
}

pub fn list(client: &Client, mailbox_id: &str) -> Result<(Status, ItemList<Conversation>), HelpScoutError> {
    let (status, res) = client.get(PREFIX, &format!("{}/conversations.json", mailbox_id), None)?;

    println!("status {:?}", status);
    println!("result - {:?}", res);
    let conversations = serde_json::from_value(res.clone())?;

    Ok((status, conversations))
}
