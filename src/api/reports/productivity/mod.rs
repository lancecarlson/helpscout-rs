use chrono::{DateTime, Utc};

use date_format::*;

use super::ReportBuilder;

pub mod overall;

#[serde(default)]
#[derive(Debug, Serialize)]
pub struct ProductivityReportBuilder {
    #[serde(with = "date_format")]
    pub(crate) start: DateTime<Utc>,
    #[serde(with = "date_format")]
    pub(crate) end: DateTime<Utc>,

    pub(crate) mailboxes: Option<String>,
    pub(crate) tags: Option<String>,
    pub(crate) types: Option<String>,
    pub(crate) folders: Option<String>,

    // Only some reports want these
    #[serde(with = "optional_date_format")]
    pub(crate) previous_start: Option<DateTime<Utc>>,
    #[serde(with = "optional_date_format")]
    pub(crate) previous_end: Option<DateTime<Utc>>,

    pub(crate) office_hours: Option<i32>,
}

impl ProductivityReportBuilder {
    /* Set methods */
    pub fn mailboxes(mut self, mailboxes: String) -> Self {
        self.mailboxes = Some(mailboxes);
        self
    }

    pub fn tags(mut self, tags: String) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn types(mut self, types: String) -> Self {
        self.types = Some(types);
        self
    }

    pub fn folders(mut self, folders: String) -> Self {
        self.folders = Some(folders);
        self
    }

    pub fn previous(mut self, previous_start: DateTime<Utc>, previous_end: DateTime<Utc>) -> Self {
        self.previous_start = Some(previous_start);
        self.previous_end = Some(previous_end);
        self
    }
}

impl From<ReportBuilder> for ProductivityReportBuilder {
    fn from(report: ReportBuilder) -> Self {
        ProductivityReportBuilder {
            start: report.start,
            end: report.end,
            mailboxes: None,
            tags: None,
            types: None,
            folders: None,
            previous_start: None,
            previous_end: None,
            office_hours: None,
        }
    }
}
