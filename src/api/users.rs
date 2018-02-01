//! Users Endpoints
//!
//! - [List](struct.UsersBuilder.html#method.list)
//! - [Get](struct.UsersBuilder.html#method.get)
//! - [List By Mailbox](struct.UsersBuilder.html#method.list_by_mailbox)
use serde_json;
use chrono::{DateTime, Utc};

use error::HelpScoutError;
use client::Client;
use envelope::{Collection, Item};

#[derive(Debug, Default, Clone, Serialize)]
pub struct UsersBuilder {
    pub(crate) page: Option<i32>,
    #[serde(rename = "type")]
    pub(crate) user_type: Option<UserType>,
}

impl UsersBuilder {
    pub(crate) fn new() -> UsersBuilder {
        UsersBuilder {
            page: None,
            user_type: None,
            .. UsersBuilder::default()
        }
    }

    /// Set the page for list actions
    pub fn page(page: i32) -> Self {
        self.page = page;
        self
    }

    /// Set the user type for list actions
    pub fn user_type(user_type: UserType) -> Self {
        self.user_type = user_type;
        self
    }

    /// List Users
    ///
    /// API docs: <https://developer.helpscout.com/help-desk-api/users/list/>
    ///
    /// ## Usage
    ///
    /// ```rust
    /// extern crate helpscout;
    ///
    /// use helpscout::{Client, Collection, HelpScoutError};
    /// use helpscout::api::users::User;
    ///
    /// fn main() {
    ///     let users = list_users().expect("get list of users");
    ///     println!("{:#?}", users);
    ///     assert!(users.items.len() > 0);
    /// }
    ///
    /// fn list_users() -> Result<Collection<User>, HelpScoutError> {
    ///     let client = helpscout::Client::example();
    ///     helpscout::api::users().list(&client)
    /// }
    /// ```
    ///
    /// ## Output
    ///
    /// ```rust,ignore
    /// Collection {
    ///     page: 1,
    ///     pages: 1,
    ///     count: 8,
    ///     items: [
    ///         User {
    ///             id: 1,
    ///             first_name: "John",
    ///             last_name: "Smith",
    ///             email: "john@test.com",
    ///             role: "owner",
    ///             timezone: "America/New_York",
    ///             photo_url: Some(
    ///                 "https://example.com/users/123.1234.png"
    ///             ),
    ///             created_at: 2015-10-28T16:43:54Z,
    ///             modified_at: 2018-01-31T19:06:48Z,
    ///             user_type: User
    ///         },
    ///         // More
    ///     ]
    /// }
    /// ```
    pub fn list(self, client: &Client) -> Result<Collection<User>, HelpScoutError> {
        let res = client.get("users.json", self)?;
        let users = serde_json::from_value(res.clone())?;
        Ok(users)
    }

    /// Get User
    ///
    /// API docs: <https://developer.helpscout.com/help-desk-api/users/get/>
    ///
    /// ## Usage
    ///
    /// ```rust
    /// extern crate helpscout;
    ///
    /// use helpscout::{Client, Item, HelpScoutError};
    /// use helpscout::api::users::User;
    ///
    /// fn main() {
    ///     let user = get_user().expect("get user");
    ///     println!("{:#?}", user);
    ///     assert!(user.item.id > 0);
    /// }
    ///
    /// fn get_user() -> Result<Item<User>, HelpScoutError> {
    ///     let client = helpscout::Client::example();
    ///     let id = find_valid_user_from_list(&client)?;
    ///     helpscout::api::users().get(&client, id)
    /// }
    ///
    /// // Get first user so get a valid id
    /// fn find_valid_user_from_list(client: &Client) -> Result<i32, HelpScoutError> {
    ///     let users = helpscout::api::users().list(&client)?;
    ///     Ok(users.items[0].id)
    /// }
    /// ```
    ///
    /// ## Output
    ///
    /// ```rust,ignore
    /// Item {
    ///     item: User {
    ///         id: 1,
    ///         first_name: "John",
    ///         last_name: "Smith",
    ///         email: "test@test.com",
    ///         role: "owner",
    ///         timezone: "America/New_York",
    ///         photo_url: Some(
    ///             "https://example.net/users/123.123.png"
    ///         ),
    ///         created_at: 2015-10-28T16:43:54Z,
    ///         modified_at: 2018-01-31T19:06:48Z,
    ///         user_type: User
    ///     }
    /// }
    /// ```
    pub fn get(self, client: &Client, id: i32) -> Result<Item<User>, HelpScoutError> {
        let res = client.get(&format!("users/{}.json", id), ())?;
        let user = serde_json::from_value(res.clone())?;
        Ok(user)
    }

    /// List Users by Mailbox
    ///
    /// API docs: <https://developer.helpscout.com/help-desk-api/users/mailbox-users/>
    ///
    /// ## Usage
    ///
    /// ```rust
    /// extern crate helpscout;
    ///
    /// use helpscout::{Client, Collection, HelpScoutError};
    /// use helpscout::api::users::User;
    ///
    /// fn main() {
    ///     let users = list_users_by_mailbox().expect("list users by mailbox");
    ///     println!("{:#?}", users);
    ///     assert!(users.items.len() > 0);
    /// }
    ///
    /// fn list_users_by_mailbox() -> Result<Collection<User>, HelpScoutError> {
    ///     let client = helpscout::Client::example();
    ///     let id = find_valid_mailbox_id(&client)?;
    ///     helpscout::api::users().list_by_mailbox(&client, id)
    /// }
    ///
    /// // Get first user so get a valid id
    /// fn find_valid_mailbox_id(client: &Client) -> Result<i32, HelpScoutError> {
    ///     let mailboxes = helpscout::api::mailboxes::list(&client)?;
    ///     Ok(mailboxes.items[0].id)
    /// }
    /// ```
    ///
    /// Output is the same as [list](struct.UsersBuilder.html#output)
    pub fn list_by_mailbox(self, client: &Client, mailbox_id: i32) -> Result<Collection<User>, HelpScoutError> {
        let res = client.get(&format!("mailboxes/{}/users.json", mailbox_id), self)?;
        let users = serde_json::from_value(res.clone())?;
        Ok(users)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserType {
    Team,
    User
}

#[derive(Debug, Clone, Deserialize)]
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

