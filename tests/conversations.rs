extern crate helpscout;

#[cfg(test)]
mod conversations {
    const API_KEY: &'static str = "123";

    use super::helpscout::{Client, Status, HelpScoutError};
    use super::helpscout::api::conversations::{self};

    #[test]
    fn list() {
        let mut c = Client::new(API_KEY);
        c.retry_wait = 3000;
        c.retry_count = 10;
        let (status, _) = conversations::list(&c, "123").expect("Conversations to be listed");

        println!("status - {:?}", status);
        //assert!(status);
    }
}
