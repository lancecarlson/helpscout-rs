extern crate helpscout;

extern crate dotenv;
extern crate chrono;
extern crate time;

extern crate log;
extern crate env_logger;

mod helper;

#[cfg(test)]
mod reports {
    use chrono::prelude::*;
    use time::Duration;
    use helper;

    use super::helpscout::api::report;

    #[test]
    fn conversations_overall() {
        let c = helper::setup();
        let start = Utc::now() - Duration::days(1);
        let end = Utc::now();
        // reports::range(start, end).conversations().overall();
        // report(start, end).conversations().overall()
        let reports = report(start, end)
            .conversations()
            .overall(&c)
            .expect("Grab reports for testing");

        //println!("{:?}", reports);
        assert!(reports.current.total_conversations > 0);

        let prev_start = Utc::now() - Duration::days(2);
        let prev_end = Utc::now() - Duration::days(1);

        let reports = report(start, end)
            .conversations()
            .set_previous(prev_start, prev_end)
            .overall(&c)
            .expect("Grab reports for testing");

        //println!("{:?}", reports);
        // Didn't get a previous report?
        //assert!(reports.previous.expect("To have a previous report").total_conversations > 0);
    }

    #[test]
    fn conversations_busy_times() {
        let c = helper::setup();
        let start = Utc::now() - Duration::days(1);
        let end = Utc::now();
        let reports = report(start, end)
            .conversations()
            .busy_times(&c)
            .expect("Grab reports for testing");

        //println!("{:?}", reports);
        assert_eq!(reports[0].day, 1);
    }
}
