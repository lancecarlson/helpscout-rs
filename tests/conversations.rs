extern crate helpscout;

extern crate dotenv;

#[cfg(test)]
mod conversations {
    use dotenv::dotenv;
    use std::env;

    use super::helpscout::Client;
    use super::helpscout::api::person::Person;
    use super::helpscout::api::users::{self};
    use super::helpscout::api::mailboxes::{self, MailboxRef};
    use super::helpscout::api::customers::{self};
    use super::helpscout::api::conversations::{self, ConversationThreadType, NewConversation, NewConversationThread};

    #[test]
    fn list_and_get() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");

        let c = Client::new(&api_key);
        let mailboxes = mailboxes::list(&c).expect("Grab mailboxes for testing");
        let conversations = conversations::list(&c, mailboxes.items[0].id).expect("Conversations to be listed");

        //println!("{:?}", conversations);
        assert!(conversations.items.len() > 0);

        let conversation = conversations::get(&c, conversations.items[0].id).expect("To get a conversation");

        //println!("{:?}", conversation);
        assert!(conversation.item.id > 0);

        //let attachment_data = conversations::get_attachment_data(&c, conversations.items[0].id).expect("Attachment data from a conversation");

        //println!("{:?}", attachment_data);
        //assert!(attachment_data.item.id > 0);
    }

    #[test]
    fn create_update_delete() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");
        let c = Client::new(&api_key);

        let mailboxes = mailboxes::list(&c).expect("Grab mailboxes for testing");
        let mailbox_ref = MailboxRef{id: mailboxes.items[0].id, name: "".into()};
        let users = users::list(&c, None, None).expect("Grab users for testing");
        let customers = customers::list().send(&c).expect("Grab customers for testing");
        println!("{:#?}", customers);

        let created_by = Person::new(users.items[0].id);
        let customer = Person::new(customers.items[0].id);

        let thread = NewConversationThread::new(
            ConversationThreadType::Customer,
            created_by,
            "I need your help with an issue I'm having.".into()
        );

        let conversation = NewConversation::new(customer, "TESTING FROM RUST LIBRARY".into(), mailbox_ref, vec![thread]);

        println!("{:#?}", conversation);
    }
}
