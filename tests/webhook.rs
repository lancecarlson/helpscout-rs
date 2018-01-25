extern crate helpscout;

extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate serde_json;

mod helper;

#[cfg(test)]
mod webhook {
    use helper;

    use super::helpscout::webhook::{self};

    #[test]
    fn validate_signature_validates() {
        let webhook_secret_key = "test";
        let data = json!({"ticket":{"id":"1","number":"2"},"customer":{"id":"1","fname":"Jackie","lname":"Chan","email":"jackie.chan@somewhere.com","emails":["jackie.chan@somewhere.com"]}});
        let signature = "+oNIxipGoqx4t2BmkBHbXKc6VHM=";
        assert!(webhook::validate_signature(webhook_secret_key, data.to_string().as_ref(), signature));
    }

    #[test]
    fn validate_signature_is_false_if_invalid() {
        //assert!(webhook::validate_signature("test", "test2"));
    }

}

