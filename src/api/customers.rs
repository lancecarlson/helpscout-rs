//! Customer Endpoints
use serde_json;
use chrono::{DateTime, Utc};
use date_format::*;

use error::HelpScoutError;
use client::Client;
use envelope::{Collection, Item};

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerSocialProfileType {
    Twitter,
    Facebook,
    Linkedin,
    Aboutme,
    Google,
    Googleplus,
    Tungleme,
    Quora,
    Foursquare,
    Youtube,
    Flickr,
    Other,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerEmailLocationType {
    Home,
    Work,
    Other,
}

impl Default for CustomerEmailLocationType {
    fn default() -> CustomerEmailLocationType {CustomerEmailLocationType::Work}
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerChatType {
    Aim,
    Gtalk,
    Icq,
    Xmpp,
    Msn,
    Skype,
    Yahoo,
    Qq,
    Other,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerPhoneLocationType {
    Home,
    Work,
    Mobile,
    Fax,
    Pager,
    Other,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerPhotoType {
    Unknown,
    Gravatar,
    Twitter,
    Facebook,
    Googleprofile,
    Googleplus,
    Linkedin,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerGender {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub photo_url: Option<String>,
    pub gender: CustomerGender,
    pub age: Option<String>,
    pub organization: Option<String>,
    pub job_title: Option<String>,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,

    // Additional fields that appear only when retrieving a single customer/via get
    pub background: Option<String>,
    pub address: Option<CustomerAddress>,
    pub social_profiles: Option<Vec<CustomerSocialProfile>>,
    pub emails: Option<Vec<CustomerEmail>>,
    pub phones: Option<Vec<CustomerPhone>>,//Always return from get as an empty array for some reason
    pub chats: Option<Vec<CustomerChat>>,
    pub websites: Option<Vec<CustomerWebsite>>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddress {
    pub id: Option<i32>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String, 
    pub lines: Vec<String>, //Street address/apartment numbers, etc
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
}

impl CustomerAddress {
    pub fn new(city: &str, state: &str, country: &str, postal_code: &str, lines: Vec<String>, created_at: DateTime<Utc>) -> CustomerAddress {
        CustomerAddress {
            id: None,
            city: city.into(),
            state: state.into(),
            country: country.into(),
            postal_code: postal_code.into(),
            lines: lines,
            created_at: created_at,
            modified_at: None,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSocialProfile {
    pub id: Option<i32>,
    pub value: String,
    pub type_: CustomerSocialProfileType,
}

impl CustomerSocialProfile {
    pub fn new(value: &str, type_: CustomerSocialProfileType) -> CustomerSocialProfile {
        CustomerSocialProfile {
            id: None,
            value: value.into(),
            type_: type_,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CustomerEmail {
    pub id: Option<i32>,
    pub value: String,
    pub location: CustomerEmailLocationType,
}

impl CustomerEmail {
    pub fn new(email: &str, location: CustomerEmailLocationType) -> CustomerEmail {
        CustomerEmail {
            id: None,
            value: email.into(),
            location: location,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CustomerPhone {
    pub id: Option<i32>,
    pub value: String,
    pub location: CustomerPhoneLocationType,
}

impl CustomerPhone {
    pub fn new(value: &str, location: CustomerPhoneLocationType) -> CustomerPhone {
        CustomerPhone {
            id: None,
            value: value.into(),
            location: location,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CustomerChat {
    pub id: Option<i32>,
    pub value: String,
    pub type_: CustomerChatType,
}

impl CustomerChat {
    pub fn new(value: &str, type_: CustomerChatType) -> CustomerChat {
        CustomerChat {
            id: None,
            value: value.into(),
            type_: type_,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CustomerWebsite {
    pub id: Option<i32>,
    pub value: String,
}

impl CustomerWebsite {
    pub fn new(value: &str) -> CustomerWebsite {
        CustomerWebsite {
            id: None,
            value: value.into(),
        }
    }
}

/// List Customers
///
/// API docs:
/// <https://developer.helpscout.com/help-desk-api/customers/list/>
///
/// ```rust
/// extern crate helpscout;
///
/// use helpscout::{Client, Collection, HelpScoutError};
/// use helpscout::api::customers::{self, Customer};
///
/// fn main() {
///     let mailboxes = list_mailboxes().expect("list mailboxes");
///     assert!(mailboxes.items.len() > 0);
/// }
///
/// fn list_mailboxes() -> Result<Collection<Customer>, HelpScoutError> {
///     let client = helpscout::Client::example();
///     
///     //Grab list of customers with no parameters.
///     //You can add parameters to narrow the results through
///     //things such as .first_name() before sending the request.
///     //Check out the customer list api docs for more possible parameters. 
///     customers::list().send(&client)
/// }
/// ```
pub fn list() -> CustomersListParamBuilder {
    let param_builder = CustomersListParamBuilder::new();
    param_builder
}


/// List Customers by Mailbox
///
/// API docs:
/// <https://developer.helpscout.com/help-desk-api/customers/list-mailbox/>
///
/// ```rust
/// extern crate helpscout;
///
/// use helpscout::{Client, Collection, HelpScoutError};
/// use helpscout::api::mailboxes::{self, Mailbox};
/// use helpscout::api::customers::{self, Customer};
///
/// fn main() {
///     let customers = list_customers_by_mailbox().expect("Customers for the mailbox to be listed");
///     assert!(customers.items.len() > 0);
/// }
///
/// fn list_customers_by_mailbox() -> Result<Collection<Customer>, HelpScoutError> {
///     let client = helpscout::Client::example();
/// 
///     //Grab list of mailboxes under an account to provide a mailbox ID for testing.
///     let mailboxes = mailboxes::list(&client);
///     
///     //Return list of customers under the specified mailbox.
///     customers::list_by_mailbox(&client, mailboxes.items[0].id)
/// }
/// ```
pub fn list_by_mailbox(client: &Client, mailbox_id: i32) -> Result<Collection<Customer>, HelpScoutError> {
    let res = client.get(&format!("mailboxes/{}/customers.json", mailbox_id), ())?;
    let customers = serde_json::from_value(res.clone())?;
    Ok(customers)
}

/// Get Customer
///
/// API docs:
/// <https://developer.helpscout.com/help-desk-api/customers/get/>
///
/// ```rust
/// extern crate helpscout;
///
/// use helpscout::{Client, Collection, HelpScoutError};
/// use helpscout::api::customers::{self, Customer};
///
/// fn main() {
///     let customer = get_customer().expect("Customers for the mailbox to be listed");
///     assert!(customer.item.id > 0);
/// }
///
/// fn get_customer() -> Result<Collection<Customer>, HelpScoutError> {
///     let client = helpscout::Client::example();
/// 
///     //Grab list of customers with no parameters to get a specific customer id.
///     let customers = customers::list().send(&client)
///     
///     //Return a single customer with the corresponding id, including the
///     //extra  mentioned in the customer object documentation.
///     customers::get(&client, customers.items[0].id)
/// }
/// ```
pub fn get(client: &Client, id: i32) -> Result<Item<Customer>, HelpScoutError> {
    let res = client.get(&format!("customers/{}.json", id), ())?;
    let customer = serde_json::from_value(res.clone())?;
    Ok(customer)
}

/// Create Customer
///
/// API docs:
/// <https://developer.helpscout.com/help-desk-api/customers/create/>
///
/// ```rust
/// extern crate helpscout;
///
/// use uuid::Uuid;
/// use helpscout::{Client, Collection, HelpScoutError};
/// use helpscout::api::customers::{self, Customer, CustomerEmail, CustomerEmailLocationType, CustomerSocialProfile, CustomerSocialProfileType};
///
/// fn main() {
///     let customers = create_customer().expect("Customer to be created, then the new customer to be returned through the list function");
///     assert!(customers.items.len() > 0);
/// }
///
/// fn create_customer() -> Result<Collection<Customer>, HelpScoutError> {
/// 
///     //Create unique email to run the example multiple times.        
///     let random_email_string = format!("guh{}@example.com", Uuid::new_v4());
///     let customer_email = CustomerEmail::new(&random_email_string, CustomerEmailLocationType::Work);
/// 
///     //Note that for each of these objects we're passing them as a vector.
///     //You can include more than one of these objects in the vec before sending it off as a request.
///     let customer_social_profiles = vec![CustomerSocialProfile::new("https://twitter.com/TwaikuGC", CustomerSocialProfileType::Twitter)];
///     
///     //Create new customer object and upload it to HelpScout, then return the customer in a list
///     customers::create("Mega", "Dog", vec![customer_email] ).organization("megadog inc").job_title("MegaDoge").social_profiles(customer_social_profile).send(&c).expect("The new customer to be posted");
///     customers::list().last_name("Dog").page(1).send(&c).expect("Customers to be listed")
///     
/// 
/// }
/// ```
pub fn create (first_name: &str, last_name: &str, emails: Vec<CustomerEmail>) -> NewCustomer {
    let new_customer = NewCustomer::new(first_name, last_name, emails);
    new_customer
}


/// Update Customer
///
/// API docs:
/// <https://developer.helpscout.com/help-desk-api/customers/update/>
///
/// ```rust
/// extern crate helpscout;
///
///
/// use helpscout::{Client, Collection, HelpScoutError};
/// use helpscout::api::customers::{self, Customer, CustomerEmail, CustomerEmailLocationType};
///
/// fn main() {
///     let customers = update_customer().expect("Customer to be created, then the new customer to be returned through the list function");
///     assert!(customers.items.len() > 0);
/// }
///
/// fn update_customer() -> Result<Collection<Customer>, HelpScoutError> {
/// 
///     //Pull list and get unique customer id and object to run the example. 
///     let customers_list = customers::list().send(&c).expect("Customers to be listed");
///     let customer = customers::get(&c, customers_list.items[0].id);    
/// 
///     //Let's build a new test email to add to the existing customer. We can also pull an email object
///     //out of the customer object and change the variables within it such as its type or address.
///     let new_customer_emails = CustomerEmail::new("newtest@email.com", CustomerEmailLocationType::Work);
///     
///     //Pull the same customer last name, but update the first name to something new.
///     let customer_name = customers_list.items[0].last_name.clone().unwrap();
/// 
///     //Update customer based on id.
///     customers::update("UPDATEDTEST", &customer_name, vec![new_customer_emails]).organization("DOGS").background("UPDATEDTEST").send(&c, customers_list.items[0].id).expect("Customer to be updated");
///     
///     //Return list of customers that have our updated first name
///     customers::list().first_name("UPDATEDTEST").send(&c).expect("Updated customer to be listed")
///     
/// 
/// }
/// ```
pub fn update(first_name: &str, last_name: &str, emails: Vec<CustomerEmail>) -> UpdatedCustomer {
    let updated_customer = UpdatedCustomer::new(first_name, last_name, emails);
    updated_customer
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomer {
    #[serde(rename = "type")]
    pub first_name: String,
    pub last_name: String,
    pub emails: Vec<CustomerEmail>,
    pub organization: Option<String>,
    pub job_title: Option<String>,
    pub background: Option<String>,
    pub address: Option<CustomerAddress>,
    pub social_profiles: Option<Vec<CustomerSocialProfile>>,
    pub phones: Option<Vec<CustomerPhone>>,
    pub chats: Option<Vec<CustomerChat>>,
    pub websites: Option<Vec<CustomerWebsite>>,

}

impl NewCustomer {
    pub fn new(first_name: &str, last_name: &str, emails: Vec<CustomerEmail>) -> NewCustomer {
        NewCustomer {
            first_name: first_name.into(),
            last_name: last_name.into(),
            emails: emails,
            organization: None,
            job_title: None,
            background: None,
            address: None,
            social_profiles: None,
            phones: None,
            chats: None,
            websites: None,
        }
    }

    pub fn organization(&mut self, organization: &str) -> &mut NewCustomer {
        self.organization = Some(organization.into());
        self
    }

    pub fn job_title(&mut self, job_title: &str) -> &mut NewCustomer {
        self.job_title = Some(job_title.into());
        self
    }

    pub fn background(&mut self, background: &str) -> &mut NewCustomer {
        self.background = Some(background.into());
        self
    }

    pub fn address(&mut self, address: CustomerAddress) -> &mut NewCustomer {
        self.address = Some(address);
        self
    }

    pub fn social_profiles(&mut self, social_profiles: Vec<CustomerSocialProfile>) -> &mut NewCustomer {
        self.social_profiles = Some(social_profiles);
        self
    }

    pub fn phones(&mut self, phones: Vec<CustomerPhone>) -> &mut NewCustomer {
        self.phones = Some(phones);
        self
    }

    pub fn chats(&mut self, chats: Vec<CustomerChat>) -> &mut NewCustomer {
        self.chats = Some(chats);
        self
    }

    pub fn websites(&mut self, websites: Vec<CustomerWebsite>) -> &mut NewCustomer {
        self.websites = Some(websites);
        self
    }

    pub fn send(&self, client: &Client) -> Result<(), HelpScoutError> {
        let body = serde_json::to_value(self)?;
        //println!("{:?}", body);
        let res = client.post("customers.json", (), Some(body.to_string()))?;
        Ok(())
    
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedCustomer {
    pub first_name: String,
    pub last_name: String,
    pub emails: Vec<CustomerEmail>,
    pub organization: Option<String>,
    pub job_title: Option<String>,
    pub background: Option<String>,
    pub address: Option<CustomerAddress>,
    pub social_profiles: Option<Vec<CustomerSocialProfile>>,
    pub phones: Option<Vec<CustomerPhone>>,
    pub chats: Option<Vec<CustomerChat>>,
    pub websites: Option<Vec<CustomerWebsite>>,

}

impl UpdatedCustomer {
    pub fn new(first_name: &str, last_name: &str, emails: Vec<CustomerEmail>) -> UpdatedCustomer {
        UpdatedCustomer {
            first_name: first_name.into(),
            last_name: last_name.into(),
            emails: emails,
            organization: None,
            job_title: None,
            background: None,
            address: None,
            social_profiles: None,
            phones: None,
            chats: None,
            websites: None,
        }
    }

    pub fn first_name(&mut self, first_name: &str) -> &mut UpdatedCustomer {
        self.first_name = first_name.into();
        self
    }

    pub fn last_name(&mut self, last_name: &str) -> &mut UpdatedCustomer {
        self.last_name = last_name.into();
        self
    }

    pub fn email(&mut self, emails: Vec<CustomerEmail>) -> &mut UpdatedCustomer{
        self.emails = emails;
        self
    }

    pub fn organization(&mut self, organization: &str) -> &mut UpdatedCustomer {
        self.organization = Some(organization.into());
        self
    }

    pub fn job_title(&mut self, job_title: &str) -> &mut UpdatedCustomer {
        self.job_title = Some(job_title.into());
        self
    }

    pub fn background(&mut self, background: &str) -> &mut UpdatedCustomer {
        self.background = Some(background.into());
        self
    }

    pub fn address(&mut self, address: CustomerAddress) -> &mut UpdatedCustomer {
        self.address = Some(address);
        self
    }

    pub fn social_profiles(&mut self, social_profiles: Vec<CustomerSocialProfile>) -> &mut UpdatedCustomer {
        self.social_profiles = Some(social_profiles);
        self
    }

    pub fn phones(&mut self, phones: Vec<CustomerPhone>) -> &mut UpdatedCustomer {
        self.phones = Some(phones);
        self
    }

    pub fn chats(&mut self, chats: Vec<CustomerChat>) -> &mut UpdatedCustomer {
        self.chats = Some(chats);
        self
    }

    pub fn websites(&mut self, websites: Vec<CustomerWebsite>) -> &mut UpdatedCustomer {
        self.websites = Some(websites);
        self
    }

    pub fn send(&self, client: &Client, id: i32) -> Result<(), HelpScoutError> {
        let body = serde_json::to_value(self)?;
        let res = client.put(&format!("customers/{}.json", id), (), Some(body.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomersListParamBuilder {

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    #[serde(with = "optional_date_format")]
    pub modified_since: Option<DateTime<Utc>>,
    pub page: Option<i32>,
}

impl CustomersListParamBuilder {
    pub fn new() -> CustomersListParamBuilder {
        CustomersListParamBuilder {
            first_name: None,
            last_name: None,
            email: None,
            modified_since: None,
            page: None,
        }
    }

    pub fn first_name(&mut self, first_name: &str) -> &mut CustomersListParamBuilder {
        self.first_name = Some(first_name.into());
        self
    }

    pub fn last_name(&mut self, last_name: &str) -> &mut CustomersListParamBuilder {
        self.last_name = Some(last_name.into());
        self
    }

    pub fn email(&mut self, email: &str) -> &mut CustomersListParamBuilder {
        self.email = Some(email.into());
        self
    }

    pub fn modified_since(&mut self, modified_since: DateTime<Utc>) -> &mut CustomersListParamBuilder {
        self.modified_since = Some(modified_since);
        self
    }

    pub fn page(&mut self, page: i32) -> &mut CustomersListParamBuilder {
        self.page = Some(page);
        self
    }

    pub fn params(&self) -> Option<Vec<(String, String)>> {
        let params: Vec<(String, String)> = vec![];
        Some(params)
    }

    pub fn send(&self, client: &Client) -> Result<Collection<Customer>, HelpScoutError> {
        let res = client.get("customers.json", &self)?;
        let customers = serde_json::from_value(res.clone())?;
        Ok(customers)
    }

}