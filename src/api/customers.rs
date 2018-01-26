use serde_json;
use chrono::{DateTime, Utc};
use date_format::*;

use error::HelpScoutError;
use client::Client;
use envelope::{Collection, Item};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerEmailLocationType {
    Home,
    Work,
    Other,
}

impl Default for CustomerEmailLocationType {
    fn default() -> CustomerEmailLocationType { CustomerEmailLocationType::Work}
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerPhoneLocationType {
    Home,
    Work,
    Mobile,
    Fax,
    Pager,
    Other,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerGender {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: i32,
    #[serde(rename = "type")]
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

    // Additional fields that appear only when retrieving a single customer
    pub background: Option<String>,
    pub address: Option<CustomerAddress>,
    pub social_profiles: Option<Vec<CustomerSocialProfiles>>,
    pub emails: Option<Vec<CustomerEmail>>,
    pub phones: Option<Vec<CustomerPhone>>,//Always an empty array for some reason
    pub chats: Option<Vec<CustomerChat>>,
    pub websites: Option<Vec<CustomerWebsite>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerAddress {
    pub id: i32,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String, 
    pub lines: Vec<String>, //Street address/apartment numbers, etc
    pub created_at: DateTime<Utc    >,
    pub modified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSocialProfiles {
    pub id: i32,
    pub value: String,
    pub type_: CustomerSocialProfileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerEmail {
    pub id: i32,
    pub value: String,
    pub location: CustomerEmailLocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerPhone {
    pub id: i32,
    pub value: String,
    pub location: CustomerPhoneLocationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerChat {
    pub id: i32,
    pub value: String,
    pub location: CustomerChatType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerWebsite {
    pub id: i32,
    pub value: String,
}

pub fn list() -> CustomersListParamBuilder {
    let mut param_builder = CustomersListParamBuilder::new();
    //println!("{:?}", param_builder);
    param_builder
}

pub fn list_by_mailbox(client: &Client, mailbox_id: i32) -> Result<Collection<Customer>, HelpScoutError> {
    let res = client.get(&format!("mailboxes/{}/customers.json", mailbox_id), ())?;
    let customers = serde_json::from_value(res.clone())?;
    Ok(customers)
}

pub fn get(client: &Client, id: i32) -> Result<Item<Customer>, HelpScoutError> {
    let res = client.get(&format!("customers/{}.json", id), ())?;
    let customer = serde_json::from_value(res.clone())?;
    Ok(customer)
}

pub fn create(client: &Client, customer: &NewCustomer, reload: Option<bool>) -> Result<(), HelpScoutError> {
    let body = serde_json::to_value(customer)?;
    let res = client.post("customers.json", (), Some(body.to_string()))?;
    Ok(())
}

#[derive(Debug, Serialize)] //Not satisfied
#[serde(rename_all = "camelCase")]
pub struct NewCustomer {
    #[serde(rename = "type")]
    pub first_name: String,
    pub last_name: String,
    pub emails: Vec<NewCustomerEmail>,
    pub organization: Option<String>,
    pub job_title: Option<String>,
    pub background: Option<String>,
    pub address: Option<CustomerAddress>,
    pub social_profiles: Option<Vec<CustomerSocialProfiles>>,
    pub phones: Option<Vec<CustomerPhone>>,
    pub chats: Option<Vec<CustomerChat>>,
    pub websites: Option<Vec<CustomerWebsite>>,

}

impl NewCustomer {
    pub fn new(first_name: String, last_name: String, emails: Vec<NewCustomerEmail>) -> NewCustomer {
        NewCustomer {
            first_name: first_name,
            last_name: last_name,
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
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomerEmail {
    pub value: String,
    pub location: CustomerEmailLocationType,
}

impl NewCustomerEmail {
    pub fn new(email: String, location: CustomerEmailLocationType) -> NewCustomerEmail {
        NewCustomerEmail {
            value: email,
            location: location,
        }
    }
}

/*fn parse_params(first_name: Option<&str>,last_name: Option<&str>,email: Option<&str>, page: Option<i32> ) -> Option<Vec<(String, String)>> {
    let mut params: Vec<(String, String)> = vec![];

    if let Some(first_name) = first_name {
        params.push(("firstName".into(), format!("{}", first_name)));
    }

    if let Some(last_name) = last_name {
        params.push(("lastName".into(), format!("{}", last_name)));
    }

    if let Some(email) = email {
        params.push(("email".into(), format!("{}", email)));
    }

    /*if let Some(modified_since) = modified_since {
        params.push(("modifiedSince".into(), format!("{}", modified_since)));
    }*/

    if let Some(page) = page {
        params.push(("page".into(), format!("{}", page)));
    }

    if params.len() > 0 {
        Some(params)
    } else {
        None
    }
}*/

//Reminder 


#[derive(Debug, Serialize)]
pub struct CustomersListParamBuilder {

    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    #[serde(with = "optional_date_format")]
    pub modified_since: Option<DateTime<Utc>>,
    pub page: Option<i32>,
}
/*
impl ConversationsReportBuilder {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> ConversationsReportBuilder {
        ConversationsReportBuilder {
            start: start,
            end: end,
            mailboxes: None,
            tags: None,
            types: None,
            folders: None,
            previous_start: None,
            previous_end: None,
        }
    }

    pub fn previous(&mut self, previous_start: DateTime<Utc>, previous_end: DateTime<Utc>) -> &ConversationsReportBuilder {
        self.previous_start = Some(previous_start);
        self.previous_end = Some(previous_end);
        self
    }
}*/
impl CustomersListParamBuilder {
    pub fn new() -> CustomersListParamBuilder {
        CustomersListParamBuilder {
            first_name:None,
            last_name:None,
            email:None,
            modified_since:None,
            page:None,
        }
    }

    pub fn first_name(&mut self, first_name: &str) -> &CustomersListParamBuilder {
        self.first_name = Some(first_name.into());
        self
    }

    pub fn last_name(&mut self, last_name: &str) -> &mut CustomersListParamBuilder {
        self.last_name = Some(last_name.into());
        self
    }

    pub fn email(&mut self, email: &str) -> &CustomersListParamBuilder {
        self.email = Some(email.into());
        self
    }

    pub fn modified_since(&mut self, modified_since: DateTime<Utc>) -> &CustomersListParamBuilder {
        self.modified_since = Some(modified_since);
        self
    }

    pub fn page(&mut self, page: i32) -> &CustomersListParamBuilder {
        self.page = Some(page);
        self
    }

    pub fn params(&self) -> Option<Vec<(String, String)>> {
        let mut params: Vec<(String, String)> = vec![];
        Some(params)
    }

    pub fn send(&self, client: &Client) -> Result<Collection<Customer>, HelpScoutError> {
    //println!("{:?}", param_builder);
    let res = client.get("customers.json", self)?;
    let customers = serde_json::from_value(res.clone())?;
    Ok(customers)
    }

    /*pub fn previous(&mut self, previous_start: Option<DateTime<Utc>>, previous_end: Option<DateTime<Utc>>) -> &CustomersListParamBuilder {
        self.previous = Some(PreviousRange{previous_start, previous_end});
        self
    }*/
}