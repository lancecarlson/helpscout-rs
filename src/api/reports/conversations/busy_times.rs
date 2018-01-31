//! Busiest Time of Day
//!
//! API docs: <https://developer.helpscout.com/help-desk-api/reports/conversations/busy-times/>
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
//! use helpscout::api::reports::conversations::BusyTimeStatistics;
//!
//! fn main() {
//!     let report = busy_times_report().expect("Busies time report");
//!     println!("{:#?}", report);
//!     assert!(report[0].day > 0);
//! }
//!
//! fn busy_times_report() -> Result<Vec<BusyTimeStatistics>, HelpScoutError> {
//!     let client = helpscout::Client::example();
//!     let start = Utc::now() - Duration::days(1);
//!     let end = Utc::now();
//!     report(start, end)
//!         .conversations()
//!         .busy_times(&client)
//! }
//! ```

use serde_json;

use client::Client;
use error::HelpScoutError;
use super::{ConversationsReportBuilder, BusyTimeStatistics};

impl ConversationsReportBuilder {
    pub fn busy_times(self, client: &Client) -> Result<Vec<BusyTimeStatistics>, HelpScoutError> {
        let res = client.get("reports/conversations/busy-times.json", self)?;
        let stats = serde_json::from_value(res.clone())?;
        Ok(stats)
    }
}
