//! Conversations Overall Report
//!
//! API docs: <https://developer.helpscout.com/help-desk-api/reports/conversations/conversations/>
//!
//! ## Usage
//!
//! ```rust
//! extern crate helpscout;
//! extern crate chrono;
//! extern crate time;
//!
//! use chrono::prelude::*;
//! use time::Duration;
//!
//! use helpscout::HelpScoutError;
//! use helpscout::api::report;
//! use helpscout::api::reports::conversations::overall::ConversationsReport;
//!
//! fn main() {
//!     let report = overall_report().expect("run conversations overall report");
//!     println!("{:#?}", report);
//!     assert!(report.current.customers > 0);
//! }
//!
//! fn overall_report() -> Result<ConversationsReport, HelpScoutError> {
//!     let client = helpscout::Client::example();
//!     let start = Utc::now() - Duration::days(10);
//!     let end = Utc::now();
//!     report(start, end)
//!         .conversations()
//!         .overall(&client)
//! }
//! ```
//!
//! ## Output
//!
//! ```rust,ignore
//! ConversationsReport {
//!     filter_tags: [
//!        FilterTag {
//!            id: 123,
//!            name: ""
//!        },
//!        // More
//!     ],
//!     company_id: None,
//!     busiest_day: BusyTimeStatistics {
//!         day: 3,
//!         hour: 0,
//!         count: 22
//!     },
//!     busiest_time_start: None,
//!     busiest_time_end: None,
//!     current: ConversationsTimeRangeStatistics {
//!         start_date: 2018-01-30T18:41:25Z,
//!         end_date: 2018-01-31T18:41:25Z,
//!         total_conversations: 51,
//!         conversations_created: 21,
//!         new_conversations: 31,
//!         customers: 10,
//!         conversations_per_day: 10
//!     },
//!     previous: None,
//!     delta: None,
//!     tags: TopStatistics {
//!         count: 20,
//!         top: [
//!             Statistics {
//!                 id: 1,
//!                 name: Some("tag a"),
//!                 count: 51,
//!                 previous_count: None,
//!                 percent: 39.56043956043956,
//!                 previous_percent: None,
//!                 delta_percent: None,
//!             },
//!             // More
//!         ]
//!     },
//!     customers: TopStatistics {
//!         count: 1816,
//!         top: [
//!             Statistics {
//!                 id: 1,
//!                 name: "John Smith",
//!                 count: 31,
//!                 previous_count: None,
//!                 percent: 42.22222222222222,
//!                 previous_percent: None,
//!                 delta_percent: None,
//!             },
//!             // More
//!         ]
//!     },
//!     replies: TopStatistics {
//!         count: 109,
//!         top: [
//!             ReplyStatistics {
//!                 id: 1,
//!                 name: "Saved reply",
//!                 mailbox_id: 1,
//!                 count: 16,
//!                 prevous_count: None,
//!                 percent: 0.88105726872247,
//!                 previous_percent: None,
//!                 delta_percent: None,
//!             },
//!             // More
//!         ]
//!     },
//!     workflows: TopStatistics {
//!         count: 240,
//!         top: [
//!             Statistics {
//!                 id: 1,
//!                 name: Some("workflow 1"),
//!                 count: 36,
//!                 previous_count: None,
//!                 percent: 40.0,
//!                 previous_percent: None,
//!                 delta_percent: None,
//!             },
//!             // More
//!         ]
//!     }
//! }
//! ```

use serde_json;
use chrono::{DateTime, Utc};

use client::Client;
use error::HelpScoutError;
use api::reports::FilterTag;
use super::{ConversationsReportBuilder, TopStatistics, Statistics, BusyTimeStatistics};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsReport {
    pub filter_tags: Vec<FilterTag>,
    pub company_id: Option<i64>,
    pub busiest_day: BusyTimeStatistics,
    pub busiest_time_start: Option<i32>,
    pub busiest_time_end: Option<i32>,
    pub current: ConversationsTimeRangeStatistics,
    pub previous: Option<ConversationsTimeRangeStatistics>,
    pub delta: Option<ConversationsMultipleTimeRangeStatistics>,
    pub tags: TopStatistics<Statistics>,
    pub customers: TopStatistics<Statistics>,
    pub replies: TopStatistics<ReplyStatistics>,
    pub workflows: TopStatistics<Statistics>,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsMultipleTimeRangeStatistics {
    pub total_conversations: f64,
    pub conversations_created: f64,
    pub new_conversations: f64,
    pub customers: f64,
    pub conversations_per_day: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyStatistics {
    pub name: Option<String>,
    pub count: i64,
    pub previous_count: Option<i64>,
    pub percent: f64,
    pub previous_percent: Option<f64>,
    pub delta_percent: Option<f64>,
    pub mailbox_id: i32,
}

impl ConversationsReportBuilder {
    pub fn overall(self, client: &Client) -> Result<ConversationsReport, HelpScoutError> {
        let res = client.get("reports/conversations.json", self)?;
        let conversations = serde_json::from_value(res.clone())?;
        Ok(conversations)
    }
}
