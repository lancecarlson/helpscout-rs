extern crate helpscout;

extern crate dotenv;

#[cfg(test)]
mod customers {
    use dotenv::dotenv;
    use std::env;

    use super::helpscout::{Client, HelpScoutError};
    use super::helpscout::api::customers::{self};

    #[test]
    fn list() {
        dotenv().ok();

        let API_KEY = env::var("API_KEY").expect("to have API_KEY set");

        let mut c = Client::new(&API_KEY);
        let customers = customers::list(&c).expect("Customers to be listed");
        //println!("{:?}", customers.items[0]);
        assert!(customers.items.len() > 0);
    }
}