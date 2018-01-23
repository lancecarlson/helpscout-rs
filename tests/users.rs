extern crate helpscout;

extern crate dotenv;

#[cfg(test)]
mod users {
    use dotenv::dotenv;
    use std::env;

    use super::helpscout::Client;
    use super::helpscout::api::users::{self, UserType};

    #[test]
    fn list_and_get() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");

        let c = Client::new(&api_key);
        let users = users::list(&c, None, None).expect("Grab users for testing");

        //println!("{:?}", users);
        assert!(users.items.len() > 0);

        let user = users::get(&c, users.items[0].id).expect("Users to be listed");
        //println!("{:?}", user);
        assert!(user.item.id > 0)
    }
}
