//! Help Scout Reports
//!
//! There are several different categories of reports. We've broken them
//! down into separate modules. Each category has several reports scoped
//! to that topic.
//!
//! - [Conversations](conversations/index.html)
//! - Docs (TODO)
//! - Happiness (TODO)
//! - Productivity (TODO)
//! - Company (TODO)
//! - User (TODO)
pub mod conversations;

use chrono::{DateTime, Utc};

use self::conversations::ConversationsReportBuilder;

#[derive(Debug, Clone)]
pub struct ReportBuilder {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl ReportBuilder {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> ReportBuilder {
        ReportBuilder {
            start: start,
            end: end,
        }
    }

    pub fn conversations(self) -> ConversationsReportBuilder {
        self.into()
    }
}

// Tags available for reporting
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTag {
    pub id: i32,
    pub name: String,
}
