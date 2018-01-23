#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonType {
    User,
    Customer,
    Team
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    // Undocumented
    pub emails: Option<Vec<String>>,
    pub phone: Option<String>,
    #[serde(rename = "type")]
    pub person_type: Option<PersonType>,
    pub photo_url: Option<String>,
}

impl Person {
    pub fn new(id: i32) -> Person {
        Person {
            id: id,
            first_name: None,
            last_name: None,
            email: None,
            emails: None,
            phone: None,
            person_type: None,
            photo_url: None,
        }
    }
}
