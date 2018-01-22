#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonType {
    User,
    Customer,
    Team
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    // Undocumented
    pub emails: Option<Vec<String>>,
    pub phone: Option<String>,
    #[serde(rename = "type")]
    pub person_type: Option<PersonType>,
    pub photo_url: Option<String>,
}
