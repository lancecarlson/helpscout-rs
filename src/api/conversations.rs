use serde_json;
use chrono::{DateTime, Utc};

use error::HelpScoutError;
use client::Client;
use envelope::{Collection, Item};
use api::person::Person;
use api::mailboxes::MailboxRef;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationType {
    Email,
    Chat,
    Phone,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationStatus {
    Active,
    Pending,
    Closed,
    Spam,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationSourceVia {
    Customer,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationThreadType {
    LineItem,
    Note,
    Message,
    Chat,
    Customer,
    ForwardParent,
    ForwardChild,
    Phone,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationThreadStatus {
    NoChange,
    Active,
    Pending,
    Closed,
    Spam,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConversationActionType {
    MovedFromMailbox,
    Merged,
    Imported,
    Workflow,
    ImportedExternal,
    ChangedTicketCustomer,
    DeletedTicket,
    RestoreTicket,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationThreadState {
    Published,
    Draft,
    UnderReview,
    Hidden,
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
    pub user_modified_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub closed_by: Option<Person>,
    pub source: ConversationSource,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub tags: Vec<String>,

    // Additional fields that appear only when retrieving a single Conversation
    // TODO: complete
    pub threads: Vec<ConversationThread>,
    //pub custom_fields: Vec<CustomField>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewConversation {
    #[serde(rename = "type")]
    pub conversation_type: Option<ConversationType>,
    pub customer: Person,
    pub subject: String,
    pub mailbox: MailboxRef,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,

    // Additional fields that appear only when retrieving a single Conversation
    pub threads: Vec<NewConversationThread>,
//    pub custom_fields: Option<Vec<CustomField>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: i32,
    pub hash: String,
    pub mime_type: String,
    pub filename: String,
    pub size: String,
    pub width: String,
    pub height: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AttachmentData {
    pub id: i32,

    // base64 encoded data
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct ConversationSource {
    #[serde(rename = "type")]
    pub conversation_source_type: ConversationSourceType,
    pub via: ConversationSourceVia,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationThread {
    pub id: i32,
    #[serde(rename = "type")]
    pub conversation_thread_type: ConversationThreadType,
    pub assigned_to: Person,
    pub status: ConversationThreadStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Person,
    pub source: ConversationSource,
    pub action_type: ConversationActionType,
    pub action_source_id: Option<i32>,
    pub from_mailbox: MailboxRef,
    pub state: ConversationThreadState,
    pub customer: Person,
    pub body: String,
    pub to: Vec<String>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub attachments: Option<Vec<Attachment>>,
    pub saved_reply_id: i32,
    pub created_by_customer: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewConversationThread {
    pub created_by: Person,
    pub conversation_thread_type: ConversationThreadType,
    pub body: String,
    pub assigned_to: Option<Person>,
    pub status: Option<ConversationThreadStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub attachments: Option<Vec<Attachment>>,
}

pub fn list(client: &Client, mailbox_id: i32) -> Result<Collection<Conversation>, HelpScoutError> {
    let res = client.get(&format!("mailboxes/{}/conversations.json", mailbox_id), None)?;
    let conversations = serde_json::from_value(res.clone())?;
    Ok(conversations)
}

pub fn get(client: &Client, id: i32) -> Result<Item<Conversation>, HelpScoutError> {
    let res = client.get(&format!("conversations/{}.json", id), None)?;
    let conversation = serde_json::from_value(res.clone())?;
    Ok(conversation)
}

pub fn create(client: &Client, conversation: &NewConversation, imported: Option<bool>, auto_reply: Option<bool>, reload: Option<bool>) -> Result<(), HelpScoutError> {
    let body = serde_json::to_value(conversation)?;
    let res = client.post("conversations.json", None, Some(body.to_string()))?;
    Ok(())
}

/*
pub fn update(client: &Client, id: i32) -> Result<Item<Conversation>, HelpScoutError> {
    let res = client.get(&format!("conversations/{}.json", id), None)?;
    let conversation = serde_json::from_value(res.clone())?;
    Ok(conversation)
}
*/
pub fn delete(client: &Client, id: i32) -> Result<Item<Conversation>, HelpScoutError> {
    let res = client.get(&format!("conversations/{}.json", id), None)?;
    let conversation = serde_json::from_value(res.clone())?;
    Ok(conversation)
}

pub fn get_attachment_data(client: &Client, id: i32) -> Result<Item<AttachmentData>, HelpScoutError> {
    let res = client.get(&format!("attachments/{}/data.json", id), None)?;
    let attachment_data = serde_json::from_value(res.clone())?;
    Ok(attachment_data)
}
