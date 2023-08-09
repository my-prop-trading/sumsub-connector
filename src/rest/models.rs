use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAccessTokenRequest {
    #[serde(rename = "userId")]
    pub client_id: String,
    #[serde(rename = "levelName")]
    pub level_name: String,
    #[serde(rename = "externalActionId")]
    pub external_action_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAccessTokenResponse {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "userId")]
    pub client_id: String,
}
