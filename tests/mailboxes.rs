extern crate helpscout;

extern crate dotenv;

#[cfg(test)]
mod mailboxes {
    use dotenv::dotenv;
    use std::env;

    use super::helpscout::{Client, HelpScoutError};
    use super::helpscout::api::mailboxes::{self};

    #[test]
    fn list() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");

        let mut c = Client::new(&api_key);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let mailboxes = mailboxes::list(&c).expect("Mailboxes to be listed");

        assert!(mailboxes.items.len() > 0)
    }

    #[test]
    fn get() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");

        let mut c = Client::new(&api_key);
        let mailboxes = mailboxes::list(&c).expect("Grab mailboxes for testing");
        let mailbox = mailboxes::get(&c, mailboxes.items[0].id).expect("Mailboxes to be listed");
        //println!("{:?}", mailbox);
        assert!(mailbox.item.id > 0)
    }
}
