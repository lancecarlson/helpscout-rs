use chrono::{DateTime, Utc};

use date_format::*;

use super::ReportBuilder;

pub mod overall;
pub mod busy_times;
pub mod new_conversations;
pub mod received_messages;
pub mod drill_down;

#[derive(Debug, Clone, Deserialize)]
pub struct TopStatistics<T> {
    pub count: i64,
    pub top: Vec<T>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub id: i64,
    pub name: Option<String>,
    pub count: i64,
    pub previous_count: Option<i64>,
    pub percent: f64,
    pub previous_percent: Option<f64>,
    pub delta_percent: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct FieldStatistics {
    pub count: i64,
    pub fields: Vec<CustomFieldStatistics>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldStatistics {
    pub id: i64,
    pub name: String,
    pub mailbox_id: i32,
    pub values: Vec<Statistics>,
    pub summary: CustomFieldSummary
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFieldSummary {
    pub total: i64,
    pub total_answered: i64,
    pub previous_total: Option<i64>,
    pub previous_total_answered: Option<i64>,
    pub unanswered_delta: f64,
    pub unanswered_previous_percent: f64,
    pub unanswered_percent: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BusyTimeStatistics {
    pub day: i32,
    pub hour: i32,
    pub count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewConversationsStatistics {
    pub start: DateTime<Utc>,
    pub count: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReceivedMessagesStatistics {
    pub date: DateTime<Utc>,
    pub messages: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbbreviatedConversationsStatistics {
    pub id: i32,
    pub number: i32,
    #[serde(rename = "type")]
    pub conversation_type: ConversationType,
    pub mailboxid: i32,
    pub attachments: bool,
    pub subject: String,
    pub status: ConversationStatus,
    pub thread_count: i32,
    pub preview: String,
    pub customer_name: String,
    pub customer_email: String,
    pub customer_ids: Vec<i32>,
    pub modified_at: DateTime<Utc>,
    pub waiting_since: DateTime<Utc>,
    pub waiting_since_type: i32,
    pub assignedid: i32,
    pub tags: Vec<ColorTag>,
    pub assigned_name: Option<String>, 
}

// Color tags for drill down/abbreviated conversation reports
#[derive(Debug, Clone, Deserialize)]
pub struct ColorTag {
    pub id: i32,
    pub name: String,
    pub color: String,
}

// Optionally set this previous time range to compare against
#[derive(Debug, Default, Serialize)]
pub struct PreviousRange {
    pub previous_start: Option<DateTime<Utc>>,
    pub previous_end: Option<DateTime<Utc>>,
}

//Enum for display the interval which the statistics summarize in received/new conversations
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConvReportViewByType {
    Day,
    Week,
    Month,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationType {
    Email,
    Chat,
    Phone,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationStatus {
    Active,
    Pending,
    Closed,
    Spam,
}

#[serde(default)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsReportBuilder {
    #[serde(with = "date_format")]
    pub start: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub end: DateTime<Utc>,

    pub mailboxes: Option<String>,
    pub tags: Option<String>,
    pub types: Option<String>,
    pub folders: Option<String>,

    // Only some reports want these
    pub page: Option<i32>,
    pub rows: Option<i32>, //For drilldown: defaults to 10, maximum value is 50
    pub view_by: Option<ConvReportViewByType>,
    #[serde(with = "optional_date_format")]
    pub previous_start: Option<DateTime<Utc>>,
    #[serde(with = "optional_date_format")]
    pub previous_end: Option<DateTime<Utc>>,
}

impl ConversationsReportBuilder {
    /* Set methods */
    pub fn set_view_by(mut self, view_by: ConvReportViewByType) -> Self {
        self.view_by = Some(view_by);
        self
    }

    pub fn set_mailboxes(mut self, mailboxes: String) -> Self {
        self.mailboxes = Some(mailboxes);
        self
    }

    pub fn set_tags(mut self, tags: String) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn set_types(mut self, types: String) -> Self {
        self.types = Some(types);
        self
    }

    pub fn set_folders(mut self, folders: String) -> Self {
        self.folders = Some(folders);
        self
    }

    pub fn set_previous(mut self, previous_start: DateTime<Utc>, previous_end: DateTime<Utc>) -> Self {
        self.previous_start = Some(previous_start);
        self.previous_end = Some(previous_end);
        self
    }

    pub fn set_page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn set_rows(mut self, rows: i32) -> Self {
        self.rows = Some(rows);
        self
    }
}

impl From<ReportBuilder> for ConversationsReportBuilder {
    fn from(report: ReportBuilder) -> Self {
        ConversationsReportBuilder {
            start: report.start,
            end: report.end,
            mailboxes: None,
            tags: None,
            types: None,
            folders: None,
            view_by: None,
            page: None,
            rows: None,
            previous_start: None,
            previous_end: None,
        }
    }
}

