//! Productivity Overall Report
//!
//! API docs: <https://developer.helpscout.com/help-desk-api/reports/productivity/productivity/>
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
//! use helpscout::api::reports::productivity::overall::ProductivityReport;
//!
//! fn main() {
//!     let report = overall_report().expect("run productivity overall report");
//!     println!("{:#?}", report);
//!     //assert!(report.current.total_conversations > 0);
//! }
//!
//! fn overall_report() -> Result<ProductivityReport, HelpScoutError> {
//!     let client = helpscout::Client::example();
//!     let start = Utc::now() - Duration::days(1);
//!     let end = Utc::now();
//!     report(start, end)
//!         .productivity()
//!         .overall(&client)
//! }
//! ```
//!
//! ## Output
//!
//! ```rust,ignore
//! ProductivityReport {
//!     filter_tags: [
//!         FilterTag {
//!             id: 1943928,
//!             name: "aci"
//!         },
//!         // More
//!     ],
//!     current: ProductivityTimeRangeStatistics {
//!         start_date: Some(
//!             2018-01-31T19:16:57Z
//!         ),
//!         end_date: Some(
//!             2018-02-01T19:16:57Z
//!         ),
//!         total_conversations: Some(
//!             23
//!         ),
//!         resolution_time: Some(
//!             191616.1538461539
//!         ),
//!         replies_to_resolve: Some(
//!             5.5384615384615383
//!         ),
//!         response_time: Some(
//!             43911.0
//!         ),
//!         first_response_time: Some(
//!             11059.0
//!         ),
//!         resolved: Some(
//!             23
//!         ),
//!         resolved_on_first_reply: Some(
//!             7
//!         ),
//!         closed: Some(
//!             80
//!         ),
//!         replies_sent: Some(
//!             21
//!         ),
//!         handle_time: Some(
//!             219
//!         ),
//!         percent_resolved_on_first_reply: Some(
//!             40.76923076923077
//!         )
//!     },
//!     previous: Some(
//!         ProductivityTimeRangeStatistics {
//!             start_date: None,
//!             end_date: None,
//!             total_conversations: None,
//!             resolution_time: None,
//!             replies_to_resolve: None,
//!             response_time: Some(
//!                 0.0
//!             ),
//!             first_response_time: Some(
//!                 0.0
//!             ),
//!             resolved: None,
//!             resolved_on_first_reply: None,
//!             closed: Some(
//!                 0
//!             ),
//!             replies_sent: Some(
//!                 0
//!             ),
//!             handle_time: Some(
//!                 0
//!             ),
//!             percent_resolved_on_first_reply: None
//!         }
//!     ),
//!     delta: None
//! }
//! ```

use serde_json;
use chrono::{DateTime, Utc};

use client::Client;
use error::HelpScoutError;
use api::reports::FilterTag;
use super::{ProductivityReportBuilder};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductivityReport {
    pub filter_tags: Vec<FilterTag>,
    pub current: ProductivityTimeRangeStatistics,
    pub previous: Option<ProductivityTimeRangeStatistics>,
    pub delta: Option<ProductivityMultipleTimeRangeStatistics>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductivityTimeRangeStatistics {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub total_conversations: Option<i32>,
    pub resolution_time: Option<f64>,
    pub replies_to_resolve: Option<f64>,
    pub response_time: Option<f64>,
    pub first_response_time: Option<f64>,
    pub resolved: Option<i32>,
    pub resolved_on_first_reply: Option<i32>,
    pub closed: Option<i32>,
    pub replies_sent: Option<i32>,
    pub handle_time: Option<i32>,
    pub percent_resolved_on_first_reply: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductivityMultipleTimeRangeStatistics {
    pub total_conversations: f64,
    pub replies_sent: f64,
    pub first_response_time: f64,
    pub resolved: f64,
    pub replies_to_resolve: f64,
    pub closed: f64,
    pub resolved_on_first_reply: f64,
    pub response_time: f64,
    pub handle_time: f64,
    pub resolution_time: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTimeStatistics {
    pub count: i32,
    pub previous_count: i32,
    pub ranges: Vec<ResponseTimeRangeStatistics>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTimeRangeStatistics {
    pub id: i32, // TODO: Convert to enum?
    pub count: i32,
    pub previous_count: i32,
    pub percent: f64,
    pub previous_percent: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleTimeStatistics {
    pub count: i32,
    pub previous_count: i32,
    pub ranges: Vec<HandleTimeRangeStatistics>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HandleTimeRangeStatistics {
    pub id: i32, // TODO: Convert to enum?
    pub count: i32,
    pub previous_count: i32,
    pub percent: f64,
    pub previous_percent: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepliesToResolveStatistics {
    pub count: i32,
    pub previous_count: i32,
    pub ranges: Vec<RepliesToResolveRangeStatistics>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepliesToResolveRangeStatistics {
    pub id: i32, // TODO: Convert to enum?
    pub count: i32,
    pub previous_count: i32,
    pub percent: f64,
    pub previous_percent: f64,
    pub resolution_time: f64,
}

impl ProductivityReportBuilder {
    pub fn overall(self, client: &Client) -> Result<ProductivityReport, HelpScoutError> {
        let res = client.get("reports/productivity.json", self)?;
        let productivity = serde_json::from_value(res.clone())?;
        Ok(productivity)
    }
}
