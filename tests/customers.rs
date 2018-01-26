extern crate helpscout;
extern crate env_logger;
extern crate dotenv;

#[cfg(test)]
mod customers {
    use dotenv::dotenv;
    use std::env;

    use super::helpscout::{Client, HelpScoutError};
    use super::helpscout::api::customers::{self, CustomerEmailLocationType, NewCustomer, NewCustomerEmail};
    use super::helpscout::api::mailboxes::{self};


    #[test]
    fn list_and_get() {
        dotenv().ok();

        let API_KEY = env::var("API_KEY").expect("to have API_KEY set");

        let mut c = Client::new(&API_KEY);
        //println!("{:?}", param);
        let customers = customers::list().page(1).last_name("Smith").send(&c).expect("Customers to be listed");
        println!("{:?}", customers.items[0]);
        assert!(customers.items.len() > 0);

        let customer = customers::get(&c, customers.items[0].id).expect("To get a customer");
        //println!("{:?}", customer);
        assert!(customer.item.id > 0);


    }

    #[test]
    fn list_by_mailbox(){
        dotenv().ok();

        let API_KEY = env::var("API_KEY").expect("to have API_KEY set");

        let mut c = Client::new(&API_KEY);

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

        let customer_email = NewCustomerEmail::new("guh@example.com", CustomerEmailLocationType::Work);
        //let customers = customers::list().last_name("Dog").page(1).send(&c).expect("Customers to be listed");
        //println!("{:#?}", customers.items[0]);
        let customer =  NewCustomer::new("Mega", "Dog", vec![customer_email] ).organization("megadog inc").job_title("MegaDoge").send(&c).expect("The new customer to be posted");
        
        //let customers = customers::list().last_name("Dog").page(1).send(&c).expect("Customers to be listed");
        //println!("{:#?}", customers.items[0]);
        //println!("{:#?}", customer);
    }

}