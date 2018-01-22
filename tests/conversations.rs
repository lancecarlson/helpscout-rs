extern crate helpscout;

extern crate dotenv;

#[cfg(test)]
mod conversations {
    use dotenv::dotenv;
    use std::env;

    use super::helpscout::{Client, Status, HelpScoutError};
    use super::helpscout::api::conversations::{self};

    #[test]
    fn list() {
        dotenv().ok();

        let API_KEY = env::var("API_KEY").expect("to have API_KEY set");

        let mut c = Client::new(&API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, _) = conversations::list(&c, "123").expect("Conversations to be listed");

        println!("status - {:?}", status);
        //assert!(status);
    }
}
