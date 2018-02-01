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

    // Additional fields that appear only when retrieving a single customer
    pub background: Option<String>,
    pub address: Option<CustomerAddress>,
    pub social_profiles: Option<Vec<CustomerSocialProfiles>>,
    pub emails: Option<Vec<CustomerEmail>>,
    pub phones: Option<Vec<CustomerPhone>>,//Always an empty array for some reason
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
pub struct CustomerSocialProfiles {
    pub id: Option<i32>,
    pub value: String,
    pub type_: CustomerSocialProfileType,
}

impl CustomerSocialProfiles {
    pub fn new(value: &str, type_: CustomerSocialProfileType) -> CustomerSocialProfiles {
        CustomerSocialProfiles {
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

pub fn list() -> CustomersListParamBuilder {
    let param_builder = CustomersListParamBuilder::new();
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
pub fn create (first_name: &str, last_name: &str, emails: Vec<CustomerEmail>) -> NewCustomer {
    let new_customer = NewCustomer::new(first_name, last_name, emails);
    new_customer
}

pub fn update(first_name: &str, last_name: &str, emails: Vec<CustomerEmail>) -> UpdatedCustomer {
    /*let body = serde_json::to_value(customer)?;
    let res = client.put(&format!("customers/{}.json", id), (), Some(body.to_string()))?;*/
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
    pub social_profiles: Option<Vec<CustomerSocialProfiles>>,
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

    pub fn social_profiles(&mut self, social_profiles: Vec<CustomerSocialProfiles>) -> &mut NewCustomer {
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
    pub social_profiles: Option<Vec<CustomerSocialProfiles>>,
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

    pub fn social_profiles(&mut self, social_profiles: Vec<CustomerSocialProfiles>) -> &mut UpdatedCustomer {
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