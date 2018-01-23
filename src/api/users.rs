use serde_json;
use chrono::{DateTime, Utc};

use error::HelpScoutError;
use client::Client;
use envelope::{Collection, Item};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Team,
    User
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub timezone: String,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    #[serde(rename = "type")]
    pub user_type: UserType,
}

pub fn list(client: &Client, page: Option<i32>, user_type: Option<UserType>) -> Result<Collection<User>, HelpScoutError> {
    let mut params: Vec<(String, String)> = vec![];

    if let Some(page) = page {
        params.push(("page".into(), format!("{}", page)));
    }

/*    if let Some(user_type) = user_type {
        let value = serde_json::to_value(user_type)?;
        params.push(("type".into(), value.to_string()));
    }*/

    let res = client.get("users.json", None)?;
    let users = serde_json::from_value(res.clone())?;
    Ok(users)
}

pub fn get(client: &Client, id: i32) -> Result<Item<User>, HelpScoutError> {
    let res = client.get(&format!("users/{}.json", id), None)?;
    let user = serde_json::from_value(res.clone())?;
    Ok(user)
}

pub fn list_by_mailbox(client: &Client, mailbox_id: i32, page: Option<i32>, user_type: Option<UserType>) -> Result<Collection<User>, HelpScoutError> {
    let mut params: Vec<(String, String)> = vec![];

    if let Some(page) = page {
        params.push(("page".into(), format!("{}", page)));
    }

/*    if let Some(user_type) = user_type {
        let value = serde_json::to_value(user_type)?;
        params.push(("type".into(), value.to_string()));
    }*/

    let res = client.get("mailboxes/{}/users.json", None)?;
    let users = serde_json::from_value(res.clone())?;
    Ok(users)
}
