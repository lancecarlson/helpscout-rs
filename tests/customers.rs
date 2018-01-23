extern crate helpscout;

extern crate dotenv;
gi
#[cfg(test)]
mod conversations {
    use dotenv::dotenv;
    use std::env;

    use super::helpscout::{Client, HelpScoutError};
    use super::helpscout::api::mailboxes::{self};
    use super::helpscout::api::customers::{self};

    #[test]
    fn list() {
        dotenv().ok();

        let API_KEY = env::var("API_KEY").expect("to have API_KEY set");

        let mut c = Client::new(&API_KEY);
        let mailboxes = mailboxes::list(&c).expect("To get a list of mailboxes for testing");
        let customers = customers::list(&c, mailboxes.items[0].id).expect("Conversations to be listed");
        println!("{:?}", customers.items[0]);
        assert!(customers.items.len() > 0);
    }
}