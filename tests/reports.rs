extern crate helpscout;

extern crate dotenv;

#[cfg(test)]
mod reports {
    use dotenv::dotenv;
    use std::env;

    use super::helpscout::Client;
    use super::helpscout::api::reports::{self};

    #[test]
    fn list_and_get() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");

        let c = Client::new(&api_key);
        let reports = reports::conversations_overall(&c).expect("Grab reports for testing");

        println!("{:?}", reports);
        //assert!(reports.items.len() > 0);
    }
}
