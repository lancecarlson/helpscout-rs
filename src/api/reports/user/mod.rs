use chrono::prelude::*;

use date_format::*;

use super::ReportBuilder;

pub mod overall;

#[serde(default)]
#[derive(Debug, Serialize)]
pub struct UserReportBuilder {
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

    pub(crate) user: i32,
    pub(crate) office_hours: Option<i32>,
}

impl UserReportBuilder {
    /* Set methods */
    pub fn set_mailboxes(mut self, mailboxes: String) -> Self {
        self.mailboxes = Some(mailboxes);
        self
    }

    pub fn set_tags(mut self, tags: String) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn set_types(mut self, types: String) -> Self {
        self.types = Some(types);
        self
    }

    pub fn set_folders(mut self, folders: String) -> Self {
        self.folders = Some(folders);
        self
    }

    pub fn set_previous(mut self, previous_start: DateTime<Utc>, previous_end: DateTime<Utc>) -> Self {
        self.previous_start = Some(previous_start);
        self.previous_end = Some(previous_end);
        self
    }

    pub fn set_office_hours(mut self, office_hours: bool) -> Self {
        self.office_hours = Some(match office_hours {
            true => 1,
            false => 0,
        });
        self
    }
}

impl From<ReportBuilder> for UserReportBuilder {
    fn from(report: ReportBuilder) -> Self {
        UserReportBuilder {
            start: report.start,
            end: report.end,
            mailboxes: None,
            tags: None,
            types: None,
            folders: None,
            previous_start: None,
            previous_end: None,
            user: 0,
            office_hours: None,
        }
    }
}
