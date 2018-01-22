use serde_json;
use chrono::{DateTime, Utc};

use error::HelpScoutError;
use client::Client;
use envelope::{Collection, Item};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mailbox {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,

    // Appears in the Object documentation
    // https://developer.helpscout.com/help-desk-api/objects/mailbox/
    pub custom_fields: Option<Vec<CustomField>>,

    // Appears in the get documentation
    // https://developer.helpscout.com/help-desk-api/mailboxes/get/
    pub folders: Option<Vec<Folder>>,
}
#[derive(Debug, Deserialize)]
pub struct MailboxRef {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomFieldType {
    SingleLine,
    MultiLine,
    Data,
    Number,
    Dropdown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    pub id: i32,
    pub field_name: String,
    pub field_type: CustomFieldType,
    pub required: bool,
    pub order: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub folder_type: String,
    pub user_id: i32,
    pub total_count: i32,
    pub active_count: i32,
    pub modified_at: DateTime<Utc>,
}

pub fn list(client: &Client) -> Result<Collection<Mailbox>, HelpScoutError> {
    let res = client.get("mailboxes.json", None)?;
    let mailboxes = serde_json::from_value(res.clone())?;
    Ok(mailboxes)
}

pub fn get(client: &Client, id: i32) -> Result<Item<Mailbox>, HelpScoutError> {
    let res = client.get(&format!("mailboxes/{}.json", id), None)?;
    let mailbox = serde_json::from_value(res.clone())?;
    Ok(mailbox)
}
