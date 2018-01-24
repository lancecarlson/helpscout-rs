use serde_json;
use chrono::{DateTime, Utc};
use client::Client;

use error::HelpScoutError;
use date_format::date_format;

// Tags available for reporting
#[derive(Debug, Deserialize)]
pub struct FilterTag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TopStatistics<T> {
    pub count: i64,
    pub top: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub id: i64,
    pub name: Option<String>,
    pub count: i64,
    pub previous_count: i64,
    pub percent: f64,
    pub previous_percent: f64,
    pub delta_percent: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyStatistics {
    pub name: Option<String>,
    pub count: i64,
    pub previous_count: i64,
    pub percent: f64,
    pub previous_percent: f64,
    pub delta_percent: f64,
    pub mailbox_id: i32,
}

// Optionally set this previous time range to compare against
#[derive(Debug, Default, Serialize)]
pub struct PreviousRange {
    pub previous_start: Option<DateTime<Utc>>,
    pub previous_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ConversationsReportBuilder {
    #[serde(with = "date_format")]
    pub start: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub end: DateTime<Utc>,
    pub mailboxes: Option<String>,
    pub tags: Option<String>,
    pub types: Option<String>,
    pub folders: Option<String>,

    pub previous: Option<PreviousRange>,
}

impl ConversationsReportBuilder {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> ConversationsReportBuilder {
        ConversationsReportBuilder {
            start: start,
            end: end,
            mailboxes: None,
            tags: None,
            types: None,
            folders: None,
            previous: None,
        }
    }

    pub fn params(&self) -> Option<Vec<(String, String)>> {

        let mut params: Vec<(String, String)> = vec![];
        params.push(("start".into(), format!("{:?}", self.start)));
        params.push(("end".into(), format!("{:?}", self.end)));
        Some(params)
    }

    pub fn previous(&mut self, previous_start: Option<DateTime<Utc>>, previous_end: Option<DateTime<Utc>>) -> &ConversationsReportBuilder {
        self.previous = Some(PreviousRange{previous_start, previous_end});
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsReport {
    pub filter_tags: Vec<FilterTag>,
    pub company_id: i64,
    pub busiest_day: BusiestDay,
    pub busiest_time_start: i32,
    pub busiest_time_end: i32,
    pub current: ConversationsTimeRangeStatistics,
    pub previous: ConversationsTimeRangeStatistics,
    pub delta: ConversationsMultipleTimeRangeStatistics,
    pub tags: TopStatistics<Statistics>,
    pub customers: TopStatistics<Statistics>,
    pub replies: TopStatistics<ReplyStatistics>,
    pub workflows: TopStatistics<Statistics>,
}

#[derive(Debug, Deserialize)]
pub struct BusiestDay {
    pub day: i32,
    pub hour: i32,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsTimeRangeStatistics {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_conversations: i64,
    pub conversations_created: i64,
    pub new_conversations: i64,
    pub customers: i64,
    pub conversations_per_day: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMultipleTimeRangeStatistics {
    pub total_conversations: f64,
    pub conversations_created: f64,
    pub new_conversations: f64,
    pub customers: f64,
    pub conversations_per_day: f64,
}

pub fn conversations_overall(client: &Client, builder: ConversationsReportBuilder) -> Result<ConversationsReport, HelpScoutError> {
    let res = client.get("reports/conversations.json", builder.params())?;
    let conversations = serde_json::from_value(res.clone())?;
    Ok(conversations)
}
