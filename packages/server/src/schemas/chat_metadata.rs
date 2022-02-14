use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatMetadata {
    pub chat_guid: super::guid::GUID,
    pub user: super::user::User,
    pub latest_message: String,
}
