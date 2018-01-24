extern crate helpscout;

extern crate dotenv;
extern crate chrono;
extern crate time;

#[cfg(test)]
mod reports {
    use dotenv::dotenv;
    use std::env;
    use chrono::prelude::*;
    use time::Duration;

    use super::helpscout::Client;
    use super::helpscout::api::reports::{self};

    #[test]
    fn list_and_get() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");

        let c = Client::new(&api_key);

        let start = Utc::now() - Duration::days(1);
        let end = Utc::now();
        let builder = reports::ConversationsReportBuilder::new(start, end);
        let reports = reports::conversations_overall(&c, builder).expect("Grab reports for testing");

        println!("{:?}", reports);
        //assert!(reports.items.len() > 0);
    }
}
