pub mod conversations;
pub mod customers;
pub mod mailboxes;
pub mod users;
pub mod person;
pub mod reports;
pub mod teams;
pub mod tags;

use chrono::{DateTime, Utc};

use self::users::UsersBuilder;
use self::customers::CustomersListParamBuilder;
use self::reports::ReportBuilder;
use self::teams::TeamsBuilder;

pub fn users() -> UsersBuilder {
    UsersBuilder::default()
}

pub fn teams() -> TeamsBuilder {
    TeamsBuilder::default()
}

pub fn customers() -> CustomersListParamBuilder {
    CustomersListParamBuilder::default()
}

pub fn report(start: DateTime<Utc>, end: DateTime<Utc>) -> ReportBuilder {
    ReportBuilder::new(start, end)
}
