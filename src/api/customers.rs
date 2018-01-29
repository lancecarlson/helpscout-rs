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
    fn default() -> CustomerEmailLocationType {CustomerEmailLocationType::Work}
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
    pub created_at: DateTime<Utc>,
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

pub fn create(client: &Client, customer: &NewCustomer, reload: Option<bool>) -> Result<(), HelpScoutError> {
    let body = serde_json::to_value(customer)?;
    let res = client.post("customers.json", (), Some(body.to_string()))?;
    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomer {
    #[serde(rename = "type")]
    pub first_name: String,
    pub last_name: String,
    pub emails: Vec<NewCustomerEmail>,
    pub organization: Option<String>,
    pub job_title: Option<String>,
    pub background: Option<String>,
    pub address: Option<NewCustomerAddress>,
    pub social_profiles: Option<Vec<NewCustomerSocialProfiles>>,
    pub phones: Option<Vec<NewCustomerPhone>>,
    pub chats: Option<Vec<NewCustomerChat>>,
    pub websites: Option<Vec<NewCustomerWebsite>>,

}

impl NewCustomer {
    pub fn create(first_name: &str, last_name: &str, emails: Vec<NewCustomerEmail>) -> NewCustomer {
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

    pub fn address(&mut self, address: NewCustomerAddress) -> &mut NewCustomer {
        self.address = Some(address);
        self
    }

    pub fn social_profiles(&mut self, social_profiles: Vec<NewCustomerSocialProfiles>) -> &mut NewCustomer {
        self.social_profiles = Some(social_profiles);
        self
    }

    pub fn phones(&mut self, phones: Vec<NewCustomerPhone>) -> &mut NewCustomer {
        self.phones = Some(phones);
        self
    }

    pub fn chats(&mut self, chats: Vec<NewCustomerChat>) -> &mut NewCustomer {
        self.chats = Some(chats);
        self
    }

    pub fn websites(&mut self, websites: Vec<NewCustomerWebsite>) -> &mut NewCustomer {
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

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomerEmail {
    pub value: String,
    pub location: CustomerEmailLocationType,
}

impl NewCustomerEmail {
    pub fn new(email: &str, location: CustomerEmailLocationType) -> NewCustomerEmail {
        NewCustomerEmail {
            value: email.into(),
            location: location,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomerAddress {
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String, 
    pub lines: Vec<String>, //Street address/apartment numbers, etc
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
}

impl NewCustomerAddress {
    pub fn new(city: &str, state: &str, country: &str, postal_code: &str, lines: Vec<String>, created_at: DateTime<Utc>) -> NewCustomerAddress {
        NewCustomerAddress {
            city: city.into(),
            state: state.into(),
            country: country.into(),
            postal_code: postal_code.into(),
            lines: lines,
            created_at: created_at,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomerPhone {
    pub value: String,
    pub location: CustomerPhoneLocationType,
}

impl NewCustomerPhone {
    pub fn new(value: &str, location: CustomerPhoneLocationType) -> NewCustomerPhone {
        NewCustomerPhone {
            value: value.into(),
            location: location,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomerSocialProfiles {
    pub value: String,
    pub type_: CustomerSocialProfileType,
}

impl NewCustomerSocialProfiles {
    pub fn new(value: &str, type_: CustomerSocialProfileType) -> NewCustomerSocialProfiles {
        NewCustomerSocialProfiles {
            value: value.into(),
            type_: type_,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomerChat {
    pub value: String,
    pub type_: CustomerChatType,
}

impl NewCustomerChat {
    pub fn new(value: &str, type_: CustomerChatType) -> NewCustomerChat {
        NewCustomerChat {
            value: value.into(),
            type_: type_,
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomerWebsite {
    pub value: String,
}

impl NewCustomerWebsite {
    pub fn new(value: &str) -> NewCustomerWebsite {
        NewCustomerWebsite {
            value: value.into(),
        }
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