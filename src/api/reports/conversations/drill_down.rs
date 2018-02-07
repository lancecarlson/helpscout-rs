//! Conversations Drill-down
//! 
//! API docs: <https://developer.helpscout.com/help-desk-api/reports/conversations/received-messages/>
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
//! use helpscout::api::reports::conversations::drill_down::DrillDownConversationsReport;
//!
//! fn main() {
//!     let report = drill_down_report().expect("run drill down report");
//!     println!("{:#?}", report);
//!     assert!(report.conversations.pages > 0);
//! }
//!
//! fn drill_down_report() -> Result<DrillDownConversationsReport, HelpScoutError> {
//!     let client = helpscout::Client::example();
//!     let start = Utc::now() - Duration::days(40);
//!     let end = Utc::now();
//!     report(start, end)
//!         .conversations()
//!         .drill_down(&client)
//! }
//! ```
//! 
use serde_json;

use client::Client;
use error::HelpScoutError;
use super::{ConversationsReportBuilder, AbbreviatedConversationsStatistics};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrillDownConversationsReport {
    pub conversations: DrillDownConversationsEnvelope
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrillDownConversationsEnvelope {
    pub pages: i32,
    pub page: i32,
    pub count: i32,
    pub results: Vec<AbbreviatedConversationsStatistics>
}

impl ConversationsReportBuilder {
    pub fn drill_down(self, client: &Client) -> Result<DrillDownConversationsReport, HelpScoutError> {
        let res = client.get("reports/conversations/drilldown.json", self)?;
        let conversations = serde_json::from_value(res.clone())?;
        Ok(conversations)
    }
}