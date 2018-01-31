pub mod conversations;

use chrono::{DateTime, Utc};

use error::HelpScoutError;
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
