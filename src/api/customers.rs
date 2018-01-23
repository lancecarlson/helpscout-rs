use serde_json;
use chrono::{DateTime, Utc};

use error::HelpScoutError;
use client::Client;
use envelope::Collection;
//use api::person::Person;
//use api::mailboxes::MailboxRef;

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
    pub photo_url: Option<String>,
    pub gender: CustomerGender,
    pub age: Option<String>,
    pub organization: Option<String>,
    pub job_title: Option<String>,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,

    // Additional fields that appear only when retrieving a single customer
    // TODO: complete
    //pub background: Option<String>,
    //pub address: CustomerAddress,
    //pub social_profiles: Vec<CustomerSocialProfiles>
    //pub emails: Vec<CustomerEmail>,
    //pub phones: Vec<CustomerPhone>,
    //pub chats: Vec<CustomerChat>,
    //pub websites: Vec<CustomerWebsite>,
}

pub fn list(client: &Client) -> Result<Collection<Customer>, HelpScoutError> {
    let res = client.get("customers.json", None)?;
    let customers = serde_json::from_value(res.clone())?;
    Ok(customers)
}
