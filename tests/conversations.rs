extern crate helpscout;

extern crate dotenv;

#[cfg(test)]
mod conversations {
    use dotenv::dotenv;
    use std::env;

    use super::helpscout::{Client, HelpScoutError};
    use super::helpscout::api::mailboxes::{self};
    use super::helpscout::api::conversations::{self};

    #[test]
    fn list_and_get() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");

        let mut c = Client::new(&api_key);
        let mailboxes = mailboxes::list(&c).expect("Grab mailboxes for testing");
        let conversations = conversations::list(&c, mailboxes.items[0].id).expect("Conversations to be listed");

        //println!("{:?}", conversations);
        assert!(conversations.items.len() > 0);

        let conversation = conversations::get(&c, conversations.items[0].id).expect("To get a conversation");

        //println!("{:?}", conversation);
        assert!(conversation.item.id > 0)
    }
}
