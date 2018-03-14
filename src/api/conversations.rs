use serde_json;
use chrono::{DateTime, Utc};

use date_format::*;
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
    Workflows,
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

// Implemented to satisfy Default, which is customer in this case.
impl Default for ConversationThreadType {
    fn default() -> ConversationThreadType { ConversationThreadType::Customer }
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
    OriginalCreator,
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
    pub threads: Option<Vec<ConversationThread>>,
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
    pub tags: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,

    // Additional fields that appear only when retrieving a single Conversation
    pub threads: Vec<NewConversationThread>,
//    pub custom_fields: Option<Vec<CustomField>>,
}

impl NewConversation {
    pub fn new(customer: Person, subject: String, mailbox: MailboxRef, threads: Vec<NewConversationThread>) -> NewConversation {
        NewConversation {
            conversation_type: None,
            customer: customer,
            subject: subject,
            mailbox: mailbox,
            tags: None,
            created_at: None,
            threads: threads,
        }
    }
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
    pub assigned_to: Option<Person>,
    pub status: ConversationThreadStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_by: Option<Person>,
    pub source: Option<ConversationSource>,
    pub action_type: Option<ConversationActionType>,
    pub action_source_id: Option<i32>,
    pub from_mailbox: Option<MailboxRef>,
    pub state: Option<ConversationThreadState>,
    pub customer: Option<Person>,
    pub body: Option<String>,
    pub to: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub attachments: Option<Vec<Attachment>>,
    pub saved_reply_id: Option<i32>,
    pub created_by_customer: bool,
}

#[derive(Debug, Default, Serialize)]
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

impl NewConversationThread {
    pub fn new(conversation_thread_type: ConversationThreadType, created_by: Person, body: String) -> NewConversationThread {
        NewConversationThread {
            conversation_thread_type: conversation_thread_type,
            created_by: created_by,
            body: body,
            .. NewConversationThread::default()
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomField {
    pub field_id: i32,
    pub name: String,
    pub value: String,
    pub field_type: String,
    pub label: String,
}

#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationListParamBuilder {
    pub(crate) status: Option<String>,
    pub(crate) tag: Option<String>,
    #[serde(with = "optional_date_format")]
    pub(crate) modified_since: Option<DateTime<Utc>>,
    pub(crate) page: Option<i32>,
}

impl ConversationListParamBuilder {
    pub fn new() -> ConversationListParamBuilder {
        ConversationListParamBuilder {
            status: None,
            tag: None,
            modified_since: None,
            page: None,
            .. ConversationListParamBuilder::default()
        }
    }

    pub fn status(&mut self, status: &str) -> &mut ConversationListParamBuilder {
        self.status = Some(status.into());
        self
    }

    pub fn tag(&mut self, tag: &str) -> &mut ConversationListParamBuilder {
        self.tag = Some(tag.into());
        self
    }

    pub fn modified_since(&mut self, modified_since: DateTime<Utc>) -> &mut ConversationListParamBuilder {
        self.modified_since = Some(modified_since);
        self
    }

    pub fn page(&mut self, page: i32) -> &mut ConversationListParamBuilder {
        self.page = Some(page);
        self
    }
}
pub fn list(client: &Client, mailbox_id: i32, params: &mut ConversationListParamBuilder) -> Result<Collection<Conversation>, HelpScoutError> {
    let res = client.get(&format!("mailboxes/{}/conversations.json", mailbox_id), params)?;
    let conversations = serde_json::from_value(res.clone())?;
    Ok(conversations)
}

pub fn get(client: &Client, id: i32) -> Result<Item<Conversation>, HelpScoutError> {
    let res = client.get(&format!("conversations/{}.json", id), ())?;
    let conversation = serde_json::from_value(res.clone())?;
    Ok(conversation)
}

pub fn create(client: &Client, conversation: &NewConversation, imported: Option<bool>, auto_reply: Option<bool>, reload: Option<bool>) -> Result<(), HelpScoutError> {
    let body = serde_json::to_value(conversation)?;
    let res = client.post("conversations.json", (), Some(body.to_string()))?;
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
    let res = client.get(&format!("conversations/{}.json", id), ())?;
    let conversation = serde_json::from_value(res.clone())?;
    Ok(conversation)
}

pub fn get_attachment_data(client: &Client, id: i32) -> Result<Item<AttachmentData>, HelpScoutError> {
    let res = client.get(&format!("attachments/{}/data.json", id), ())?;
    let attachment_data = serde_json::from_value(res.clone())?;
    Ok(attachment_data)
}
