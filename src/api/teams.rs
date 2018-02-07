//! Teams Endpoints
use serde_json;

use error::HelpScoutError;
use client::Client;
use envelope::{Collection, Item};
use api::users::User;


#[derive(Debug, Default, Clone, Serialize)]
pub struct TeamsBuilder {
    pub(crate) page: Option<i32>,
}
impl TeamsBuilder {
    pub(crate) fn new() -> TeamsBuilder {
        TeamsBuilder {
            page: None,
            .. TeamsBuilder::default()
        }
    }

    /// Set the page for list actions
    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// List Teams
    ///
    /// API docs: <https://developer.helpscout.com/help-desk-api/teamss/list/>
    ///
    /// ## Usage
    ///
    /// ```rust
    /// extern crate helpscout;
    ///
    /// use helpscout::{Client, Collection, HelpScoutError};
    /// use helpscout::api::users::User;
    /// use helpscout::api::teams::{self};
    /// 
    /// //Ensure you have a team created in your HelpScout control panel to test this endpoint
    /// fn main() {
    ///     let teams = list_teams().expect("return list of teams as user objects");
    ///     println!("{:#?}", teams);
    ///     assert!(teams.items.len() > 0);
    /// }
    ///
    /// 
    /// fn list_teams() -> Result<Collection<User>, HelpScoutError> {
    ///     let client = helpscout::Client::example();
    ///     helpscout::api::teams().list(&client)
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
    ///             first_name: "Designers",
    ///             last_name: "",
    ///             email: "",
    ///             role: "",
    ///             timezone: "America/New_York",
    ///             photo_url: Some(
    ///                 "https://example.com/users/123.1234.png"
    ///             ),
    ///             created_at: 2015-10-28T16:43:54Z,
    ///             modified_at: 2018-01-31T19:06:48Z,
    ///             user_type: Team
    ///         },
    ///         // More
    ///     ]
    /// }
    /// ```
    pub fn list(self, client: &Client) -> Result<Collection<User>, HelpScoutError> {
        let res = client.get("teams.json", self)?;
        let teams = serde_json::from_value(res.clone())?;
        Ok(teams)
    }

    /// Get Teams
    ///
    /// API docs: <https://developer.helpscout.com/help-desk-api/teams/get/>
    ///
    /// ## Usage
    ///
    /// ```rust
    /// extern crate helpscout;
    ///
    /// use helpscout::{Client, Item, HelpScoutError};
    /// use helpscout::api::users::User;
    /// 
    /// //Ensure you have a team created in your HelpScout control panel to test this endpoint
    /// fn main() {
    ///     let team = get_team().expect("get single team member as user object");
    ///     println!("{:#?}", team);
    ///     assert!(team.item.id > 0);
    /// }
    ///
    /// fn get_team() -> Result<Item<User>, HelpScoutError> {
    ///     let client = helpscout::Client::example();
    ///     let id = find_valid_team_user_from_list(&client)?;
    ///     helpscout::api::users().get(&client, id)
    /// }
    ///
    /// // Get first user so get a valid id
    /// fn find_valid_team_user_from_list(client: &Client) -> Result<i32, HelpScoutError> {
    ///     let teams = helpscout::api::teams().list(&client)?;
    ///     Ok(teams.items[0].id)
    /// }
    /// ```
    ///
    /// ## Output
    ///
    /// ```rust,ignore
    /// Item {
    ///     item: User {
    ///         id: 1234,
    ///         first_name: "Designers",
    ///         last_name: "",
    ///         email: "",
    ///         role: "",
    ///         timezone: "America/New_York",
    ///         photo_url: Some(
    ///             "https://example.net/users/123.123.png"
    ///         ),
    ///         created_at: 2015-10-28T16:43:54Z,
    ///         modified_at: 2018-01-31T19:06:48Z,
    ///         user_type: Team
    ///     }
    /// }
    /// ```
    pub fn get(self, client: &Client, id: i32) -> Result<Item<User>, HelpScoutError> {
        let res = client.get(&format!("teams/{}.json", id), ())?;
        let team = serde_json::from_value(res.clone())?;
        Ok(team)
    }

    /// List Team Members
    ///
    /// API docs: <https://developer.helpscout.com/help-desk-api/teams/team-members/>
    ///
    /// ## Usage
    ///
    /// ```rust
    /// extern crate helpscout;
    ///
    /// use helpscout::{Client, Collection, Item, HelpScoutError};
    /// use helpscout::api::users::User;
    ///
    /// 
    /// //Ensure you have a team created in your HelpScout control panel with a
    /// //user assigned to it to test this endpoint
    /// fn main() {
    ///     let team_members = list_members().expect("get collection of users that belong to this team");
    ///     println!("{:#?}", team_members);
    ///     assert!(team_members.items.len() > 0);
    /// }
    ///
    /// fn list_members() -> Result<Collection<User>, HelpScoutError> {
    ///     let client = helpscout::Client::example();
    ///     let id = find_valid_member_from_list(&client)?;
    ///     helpscout::api::teams().list_team_members(&client, id)
    /// }
    ///
    /// // Get first user so get a valid id
    /// fn find_valid_member_from_list(client: &Client) -> Result<i32, HelpScoutError> {
    ///     let users = helpscout::api::teams().list(&client)?;
    ///     Ok(users.items[0].id)
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
    ///             first_name: "Team",
    ///             last_name: "Member",
    ///             email: "team@member.com",
    ///             role: "",
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
    pub fn list_team_members(self, client: &Client, id: i32) -> Result<Collection<User>, HelpScoutError> {
        let res = client.get(&format!("teams/{}/members.json", id), ())?;
        let members = serde_json::from_value(res.clone())?;
        Ok(members)
    }
}