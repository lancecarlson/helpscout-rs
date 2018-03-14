//! Mailbox Endpoints
use serde_json;
use chrono::{DateTime, Utc};

use error::HelpScoutError;
use client::Client;
use envelope::{Collection, Item};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: i32,
    pub tag: String,
    pub slug: String,
    pub color: String,
    pub count: i32,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
}

pub fn list(client: &Client) -> Result<Collection<Tag>, HelpScoutError> {
    let res = client.get("tags.json", ())?;
    let mailboxes = serde_json::from_value(res.clone())?;
    Ok(mailboxes)
}