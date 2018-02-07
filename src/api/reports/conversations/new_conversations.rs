//! New Conversations Report
//! 
//! API docs: <https://developer.helpscout.com/help-desk-api/reports/conversations/new/>
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
//! use helpscout::api::reports::conversations::new_conversations::NewConversationsReport;
//!
//! fn main() {
//!     let report = new_conversations_report().expect("run new conversations report");
//!     println!("{:#?}", report);
//!     assert!(report.current.len() > 0);
//! }
//!
//! fn new_conversations_report() -> Result<NewConversationsReport, HelpScoutError> {
//!     let client = helpscout::Client::example();
//!     let start = Utc::now() - Duration::days(10);
//!     let end = Utc::now();
//!     report(start, end)
//!         .conversations()
//!         .new_conversations(&client)
//! }
//! ```
//!
//! ## Output
//!
//! ```rust,ignore
//! NewConversationsReport {
//!     current: [
//!         NewConversationsStatistics {
//!             start: 2018-01-30T18:41:25Z,
//!             count: 12,
//!         },
//!         // More
//!     ],
//!     previous: None
//! }
//! ```

use serde_json;

use client::Client;
use error::HelpScoutError;
use super::{ConversationsReportBuilder, NewConversationsStatistics};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewConversationsReport {
    pub current: Vec<NewConversationsStatistics>,
    pub previous: Option<NewConversationsStatistics>,
}


impl ConversationsReportBuilder {
    pub fn new_conversations(self, client: &Client) -> Result<NewConversationsReport, HelpScoutError> {
        let res = client.get("reports/conversations/new.json", self)?;
        let conversations = serde_json::from_value(res.clone())?;
        Ok(conversations)
    }
}