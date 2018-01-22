use serde_json;
use chrono::{DateTime, Utc};

use error::HelpScoutError;
use client::Client;
use envelope::Collection;
use api::person::Person;
use api::mailboxes::MailboxRef;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationType {
    Email,
    Chat,
    Phone,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationStatus {
    Active,
    Pending,
    Closed,
    Spam,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationSourceType {
    Email,
    Web,
    Notification,
    #[serde(rename = "emailfwd")]
    EmailForward,
    Api,
    Chat,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationSourceVia {
    Customer,
    User,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub id: i32,
    #[serde(rename = "type")]
    pub conversation_type: ConversationType,
    pub folder_id: i32,
    pub is_draft: bool,
    pub number: i32,
    pub owner: Option<Person>,
    pub mailbox: MailboxRef,
    pub customer: Person,
    pub thread_count: i32,
    pub status: ConversationStatus,
    pub subject: String,
    pub preview: String,
    pub created_by: Person,
    pub created_at: DateTime<Utc>,
    // Deprecated in favor of user_modified_at ?
    pub modified_at: Option<DateTime<Utc>>,
    pub user_modified_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub closed_by: Option<Person>,
    pub source: ConversationSource,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub tags: Vec<String>,

    // Additional fields that appear only when retrieving a single Conversation
    // TODO: complete
    //pub threads: Vec<ConversationThread>,
    //pub custom_fields: Vec<CustomField>,
}

#[derive(Debug, Deserialize)]
pub struct ConversationSource {
    #[serde(rename = "type")]
    pub conversation_source_type: ConversationSourceType,
    pub via: ConversationSourceVia,
}

pub fn list(client: &Client, mailbox_id: i32) -> Result<Collection<Conversation>, HelpScoutError> {
    let res = client.get(&format!("mailboxes/{}/conversations.json", mailbox_id), None)?;
    let conversations = serde_json::from_value(res.clone())?;
    Ok(conversations)
}
