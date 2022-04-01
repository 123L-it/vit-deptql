use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    // pub id: i32, TODO: missing user id from user service
    pub username: String,
    pub enabled: bool,
    pub account_non_locked: bool,
}
