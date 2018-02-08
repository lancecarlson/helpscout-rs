//! Received Messages Report
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
//! use helpscout::api::reports::conversations::received_messages::ReceivedMessagesReport;
//!
//! fn main() {
//!     let report = received_messages_report().expect("run new conversations report");
//!     println!("{:#?}", report);
//!     assert!(report.current.len() > 0);
//! }
//!
//! fn received_messages_report() -> Result<ReceivedMessagesReport, HelpScoutError> {
//!     let client = helpscout::Client::example();
//!     let start = Utc::now() - Duration::days(30);
//!     let end = Utc::now();
//!     report(start, end)
//!         .conversations()
//!         .received_messages(&client)
//! }
//! ```
//!
//! ## Output
//!
//! ```rust,ignore
//! ReceivedMessagesReport {
//!     current: [
//!         DateMessageStatistics {
//!             date: 2018-01-30T18:41:25Z,
//!             messages: 12,
//!         },
//!         // More
//!     ],
//!     previous: None
//! }
//! ```

use serde_json;

use client::Client;
use error::HelpScoutError;
use super::{ConversationsReportBuilder, ReceivedMessagesStatistics};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceivedMessagesReport {
    pub current: Vec<ReceivedMessagesStatistics>,
    pub previous: Option<ReceivedMessagesStatistics>,
}


impl ConversationsReportBuilder {
    pub fn received_messages(self, client: &Client) -> Result<ReceivedMessagesReport, HelpScoutError> {
        let res = client.get("reports/conversations/received-messages.json", self)?;
        let conversations = serde_json::from_value(res.clone())?;
        Ok(conversations)
    }
}