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
        assert!(conversation.item.id > 0);

        //let attachment_data = conversations::get_attachment_data(&c, conversations.items[0].id).expect("Attachment data from a conversation");

        //println!("{:?}", attachment_data);
        //assert!(attachment_data.item.id > 0);
    }

/*    #[test]
    fn create_update_delete() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");
        let mut c = Client::new(&api_key);

        let mailboxes = mailboxes::list(&c).expect("Grab mailboxes for testing");

        let created_by = &Person{id: 1234, email: "customer@example.com", person_type: "customer"};

        let thread = &NewConversationThread {
                    conversation_thread_type: "customer",
                    created_by: {
                        id: 1234,
                        email: "customer@example.com",
                        type: "customer"
                    },
                    body: "I need your help with an issue I'm having.",
                    assignedTo: {
                        id: 2222
                    },
                    status: "active",
                    createdAt: "2012-07-23T12:34:12Z",
                    cc: [
                        "user1@example.com",
                        "user2@example.com"
                    ],
                    bcc: [
                        "user3@example.com",
                        "user4@example.com"
                    ],
                    attachments: [
                        {
                            hash: "7gjj3dg7fs3cvi956jjgfsw"
                        },
                        {
                            hash: "hfsf63fjgle8jglglksd285"
                        }
                    ]
        };

        let conversation = &Conversation{
            customer: &Customer {id: 1234, email: "customer@example.com"},
            subject: "TEST - I need help",
            mailbox: {id: mailboxes[0].item.id},
            tags: ["tag1", "tag2"],
            status: ConversationStatus::Active,
//            createdAt: "2012-07-23T12:34:12Z",
            threads: vec![
            ],
            customFields : [
                {
                    fieldId: 10,
                    value: "Custom Support"
                },
                {
                    fieldId: 11,
                    value : "10.5"
                },
                {
                    fieldId: 12,
                    value : "2015-12-01 12:20:20"
                }
            ]
        };
    }*/
}
