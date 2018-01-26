extern crate helpscout;

extern crate dotenv;
extern crate chrono;
extern crate time;

#[macro_use]
extern crate log;
extern crate env_logger;

mod helper;

#[cfg(test)]
mod reports {
    use chrono::prelude::*;
    use time::Duration;
    use helper;

    use super::helpscout::api::reports::{self};

    #[test]
    fn conversations_overall() {
        let c = helper::setup();
        let start = Utc::now() - Duration::days(1);
        let end = Utc::now();
        let builder = reports::ConversationsReportBuilder::new(start, end);
        let reports = reports::conversations_overall(&c, builder).expect("Grab reports for testing");

        //println!("{:?}", reports);
        assert!(reports.current.total_conversations > 0);

        let prev_start = Utc::now() - Duration::days(2);
        let prev_end = Utc::now() - Duration::days(1);

        let builder = reports::ConversationsReportBuilder::new(start, end)
            .previous(prev_start, prev_end);
        let reports = reports::conversations_overall(&c, builder).expect("Grab reports for testing");

        //println!("{:?}", reports);
        // Didn't get a previous report?
        //assert!(reports.previous.expect("To have a previous report").total_conversations > 0);
    }

    #[test]
    fn conversations_busy_times() {
        let c = helper::setup();
        let start = Utc::now() - Duration::days(1);
        let end = Utc::now();
        let builder = reports::ConversationsReportBuilder::new(start, end);
        let reports = reports::conversations_busy_times(&c, builder).expect("Grab reports for testing");

        //println!("{:?}", reports);
        assert_eq!(reports[0].day, 1);
    }
}
