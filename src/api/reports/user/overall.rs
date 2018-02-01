//! User/Team Report
//!
//! API docs: <https://developer.helpscout.com/help-desk-api/reports/user/user/>
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
//! use helpscout::{Client, HelpScoutError};
//! use helpscout::api::users::User;
//! use helpscout::api::report;
//! use helpscout::api::reports::user::overall::UserReport;
//!
//! fn main() {
//!     let report = overall_report().expect("run user overall report");
//!     println!("{:#?}", report);
//!     assert!(report.current.total_replies > 0);
//! }
//!
//! fn overall_report() -> Result<UserReport, HelpScoutError> {
//!     let client = Client::example();
//!     let start = Utc::now() - Duration::days(1);
//!     let end = Utc::now();
//!     let user = get_user(&client)?;
//!     report(start, end)
//!         .user()
//!         .overall(&client, user.id)
//! }
//!
//! // Grab a user to report on
//! fn get_user(client: &Client) -> Result<User, HelpScoutError> {
//!     let users = helpscout::api::users::list(&client, None, None)?;
//!     let user = users.items[0].clone();
//!     Ok(user)
//! }
//! ```
//!
//! ## Output
//!
//! ```rust,ignore
//! UserReport {
//!     filter_tags: [
//!         FilterTag {
//!             id: 1,
//!             name: "filter tag 1"
//!         },
//!         // More
//!     ],
//!     user: UserDetail {
//!         id: 1,
//!         has_photo: true,
//!         created_at: 2015-10-28T16:43:54Z,
//!         name: "John Smith",
//!         total_customers_helped: 500,
//!         photo_url: "https://example.com/users/123.456.png"
//!     },
//!     current: UserTimeRangeStatistics {
//!         start_date: 2018-01-30T22:38:28Z,
//!         end_date: 2018-01-31T22:38:28Z,
//!         total_days: 1,
//!         resolved: 1,
//!         conversations_created: 0,
//!         closed: 0,
//!         ratings: None,
//!         total_replies: 3,
//!         resolved_on_first_reply: 0,
//!         percent_resolved_on_first_reply: 0.0,
//!         replies_to_resolve: 3.0,
//!         handle_time: 123.66666666666669,
//!         happiness_score: 0.0,
//!         response_time: 42622.0,
//!         resolution_time: 98294.0,
//!         replies_per_day: 3.0,
//!         customers_helped: 2,
//!         total_conversations: 8,
//!         conversations_per_day: 1.0,
//!         busiest_day: 1
//!     },
//!     previous: None,
//!     deltas: None
//! }
//! ```

use serde_json;
use chrono::{DateTime, Utc};

use client::Client;
use error::HelpScoutError;
use api::reports::FilterTag;
use super::UserReportBuilder;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReport {
    pub filter_tags: Vec<FilterTag>,
    pub user: UserDetail,
    pub current: UserTimeRangeStatistics,
    pub previous: Option<UserTimeRangeStatistics>,
    pub deltas: Option<UserMultiTimeRangeStatistics>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTimeRangeStatistics {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_days: i32,
    pub resolved: i32,
    pub conversations_created: i32,
    pub closed: i32,
    pub ratings: Option<Vec<Rating>>,
    pub total_replies: i32,
    pub resolved_on_first_reply: i32,
    pub percent_resolved_on_first_reply: f64,
    pub replies_to_resolve: f64,
    pub handle_time: f64,
    pub happiness_score: f64,
    pub response_time: f64,
    pub resolution_time: f64,
    pub replies_per_day: f64,
    pub customers_helped: i32,
    pub total_conversations: i32,
    pub conversations_per_day: f64,
    pub busiest_day: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMultiTimeRangeStatistics {
    pub total_conversations: f64,
    pub customers_helped: f64,
    pub happiness_score: f64,
    pub replies_per_day: f64,
    pub resolved_on_first_reply: f64,
    pub handle_time: f64,
    pub conversations_per_day: f64,
    pub resolved: f64,
    pub replies_to_resolve: f64,
    pub active_conversations: f64,
    pub total_replies: f64,
    pub closed: f64,
    pub response_time: f64,
    pub resolution_time: f64,
    pub conversations_created: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub replies_sent: f64,
    pub first_response_time: f64,
    pub resolve_time: f64,
    pub rating_id: String,
    pub response_time: f64,
}

/// This is NOT a User. It includes additional reporting
/// and statistics.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    pub id: i32,
    pub has_photo: bool,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub total_customers_helped: i32,
    pub photo_url: String,
}

impl UserReportBuilder {
    pub fn overall(mut self, client: &Client, user: i32) -> Result<UserReport, HelpScoutError> {
        self.user = user;

        let res = client.get("reports/user.json", self)?;
        let user = serde_json::from_value(res.clone())?;
        Ok(user)
    }
}
