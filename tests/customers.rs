extern crate helpscout;
extern crate env_logger;
extern crate dotenv;
extern crate uuid;

#[cfg(test)]
mod customers {
    use dotenv::dotenv;
    use std::env;
    use uuid::Uuid;

    use super::helpscout::{Client, HelpScoutError};
    use super::helpscout::api::customers::{self, CustomerEmailLocationType, CustomerEmail, NewCustomer, CustomerSocialProfiles, CustomerSocialProfileType};
    use super::helpscout::api::mailboxes::{self};


    #[test]
    fn list_and_get() {
        dotenv().ok();

        let API_KEY = env::var("API_KEY").expect("to have API_KEY set");

        let c = Client::new(&API_KEY);
        //println!("{:?}", param);
        let customers = customers::list().send(&c).expect("Customers to be listed");
        //println!("{:?}", customers.items[0]);
        assert!(customers.items.len() > 0);

        let customer = customers::get(&c, customers.items[0].id).expect("To get a customer");
        //println!("{:?}", customer);
        assert!(customer.item.id > 0);


    }

    #[test]
    fn list_by_mailbox(){
        dotenv().ok();

        let API_KEY = env::var("API_KEY").expect("to have API_KEY set");

        let c = Client::new(&API_KEY);

        let mailboxes = mailboxes::list(&c).expect("Grab mailboxes for testing");
        let customers = customers::list_by_mailbox(&c, mailboxes.items[0].id).expect("Customers for the mailbox to be listed");
        //println!("{:?}", customers.items[0]);
        assert!(customers.items.len() > 0);
    }

    
    #[test]
    fn create() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");
        let c = Client::new(&api_key);
        
        let random_email_string = format!("guh{}@example.com", Uuid::new_v4());//Create unique email to run test multiple times
        let customer_email = CustomerEmail::new(&random_email_string, CustomerEmailLocationType::Work);
        let customer_social_profile = vec![CustomerSocialProfiles::new("https://twitter.com/TwaikuGC", CustomerSocialProfileType::Twitter)];

        let customer =  customers::create("Mega", "Dog", vec![customer_email] ).organization("megadog inc").job_title("MegaDoge").social_profiles(customer_social_profile).send(&c).expect("The new customer to be posted");
        
        let customers = customers::list().last_name("Dog").page(1).send(&c).expect("Customers to be listed");
        assert!(customers.items.len() > 0);
        //println!("{:#?}", customer);
    }

    #[test] 
    fn update() {
        dotenv().ok();
        let api_key: String = env::var("API_KEY").expect("to have API_KEY set");
        let c = Client::new(&api_key);
        
        let customers_list = customers::list().send(&c).expect("Customers to be listed");
        let customer = customers::get(&c, customers_list.items[0].id);
        
        //println!("{:#?}", customers_list);
        //This would work if all the HelpScout API made email a required field for all existing customer objects as they do for the update method
        //let mut customer_emails = customers_list.items[0].emails.clone().unwrap().to_vec();
        let new_customer_emails = CustomerEmail::new("newtest@email.com", CustomerEmailLocationType::Work);

        let customer_name = customers_list.items[0].last_name.clone().unwrap();
        let mut res = customers::update("UPDATEDTEST", &customer_name, vec![new_customer_emails]).organization("DOGS").background("UPDATEDTEST").send(&c, customers_list.items[0].id).expect("Customer to be updated");
        println!("{:#?}", res);
        let updated_list = customers::list().first_name("UPDATEDTEST").send(&c).expect("Updated customer to be listed");
        println!("{:#?}", updated_list);
        assert!(updated_list.items.len() > 0);
    }


}