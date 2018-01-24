use serde_json;
use chrono::{DateTime, Utc};

use error::HelpScoutError;
use client::Client;
use envelope::{Collection,Item  };

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomerEmailLocationType {
    Home,
    Work,
    Other,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
    pub full_name: Option<String>,//Usually just a concat of first and last name, but for some reason can exist when first name and last name are null, and sometimes can be
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSocialProfiles {
    pub id: i32,
    pub value: String,
    pub type_: CustomerSocialProfileType,
}

#[derive(Debug, Deserialize)]
pub struct CustomerEmail {
    pub id: i32,
    pub value: String,
    pub location: CustomerEmailLocationType,
}

#[derive(Debug, Deserialize)]
pub struct CustomerPhone {
    pub id: i32,
    pub value: String,
    pub location: CustomerPhoneLocationType,
}

#[derive(Debug, Deserialize)]
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

pub fn list(client: &Client, first_name: Option<&str>,last_name: Option<&str>,email: Option<&str>, page: Option<i32>) -> Result<Collection<Customer>, HelpScoutError> {
    let params = parse_params(first_name,last_name,email,page);
    let res = client.get("customers.json", params)?;
    let customers = serde_json::from_value(res.clone())?;
    Ok(customers)
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

fn parse_params(first_name: Option<&str>,last_name: Option<&str>,email: Option<&str>, page: Option<i32> ) -> Option<Vec<(String, String)>> {
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
}
