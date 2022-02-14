use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_guid: super::guid::GUID,
    pub username: String,
    pub display_name: String,
}
