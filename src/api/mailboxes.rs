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

#[derive(Debug, Serialize, Deserialize)]
pub struct MailboxRef {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

/// List Mailboxes
///
/// API docs:
/// <https://developer.helpscout.com/help-desk-api/mailboxes/list/>
///
/// ```rust
/// extern crate helpscout;
///
/// use helpscout::{Client, Collection, HelpScoutError};
/// use helpscout::api::mailboxes::{self, Mailbox};
///
/// fn main() {
///     let mailboxes = list_mailboxes().expect("list mailboxes");
///     assert!(mailboxes.items.len() > 0);
/// }
///
/// fn list_mailboxes() -> Result<Collection<Mailbox>, HelpScoutError> {
///     let client = helpscout::Client::example();
///     mailboxes::list(&client)
/// }
/// ```
pub fn list(client: &Client) -> Result<Collection<Mailbox>, HelpScoutError> {
    let res = client.get("mailboxes.json", ())?;
    let mailboxes = serde_json::from_value(res.clone())?;
    Ok(mailboxes)
}

/// Get mailbox
///
/// API docs:
/// <https://developer.helpscout.com/help-desk-api/mailboxes/get/>
///
/// ```rust
/// extern crate helpscout;
///
/// use helpscout::{Client, Item, HelpScoutError};
/// use helpscout::api::mailboxes::{self, Mailbox};
///
/// fn main() {
///     let mailbox = get_mailbox().expect("get mailbox");
///     assert!(mailbox.item.id > 0);
/// }
///
/// fn get_mailbox() -> Result<Item<Mailbox>, HelpScoutError> {
///     let client = helpscout::Client::example();
///
///     // Grab the list of mailboxes to fetch ids because
///     // we don't already know the ID ahead of time. You
///     // may already have the ID handy.
///     let mailboxes = mailboxes::list(&client)?;
///     mailboxes::get(&client, mailboxes.items[0].id)
/// }
/// ```
pub fn get(client: &Client, id: i32) -> Result<Item<Mailbox>, HelpScoutError> {
    let res = client.get(&format!("mailboxes/{}.json", id), ())?;
    let mailbox = serde_json::from_value(res.clone())?;
    Ok(mailbox)
}

/// Get Folders
///
/// API docs:
/// <https://developer.helpscout.com/help-desk-api/mailboxes/folders/>
///
/// ```rust
/// extern crate helpscout;
///
/// use helpscout::{Client, Collection, HelpScoutError};
/// use helpscout::api::mailboxes::{self, Folder};
///
/// fn main() {
///     let folders = get_folders().expect("get folders");
///     assert!(folders.items.len() > 0);
/// }
///
/// fn get_folders() -> Result<Collection<Folder>, HelpScoutError> {
///     let client = helpscout::Client::example();
///     let mailboxes = mailboxes::list(&client)?;
///     mailboxes::get_folders(&client, mailboxes.items[0].id)
/// }
/// ```
pub fn get_folders(client: &Client, mailbox_id:i32) -> Result<Collection<Folder>, HelpScoutError>{
    let res = client.get(&format!("mailboxes/{}/folders.json", mailbox_id), ())?;
    let folders = serde_json::from_value(res.clone())?;
    Ok(folders)
}
