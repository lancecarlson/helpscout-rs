pub mod conversations;

use chrono::{DateTime, Utc};

use error::HelpScoutError;
use self::conversations::ConversationsReportBuilder;

#[derive(Debug)]
#[derive(Clone)]
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
