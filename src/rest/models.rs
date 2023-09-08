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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookPayload {
    #[serde(rename = "applicantId")]
    pub applicant_id: String,

    #[serde(rename = "inspectionId")]
    pub inspection_id: String,

    #[serde(rename = "applicantType")]
    pub applicant_type: String,

    #[serde(rename = "correlationId")]
    pub correlation_id: String,

    #[serde(rename = "levelName")]
    pub level_name: String,

    #[serde(rename = "sandboxMode")]
    pub sandbox_mode: bool,

    #[serde(rename = "type")]
    pub type_: String,

    #[serde(rename = "reviewResult")]
    pub review_result: Option<WebhookPayloadReviewModel>,

    #[serde(rename = "reviewStatus")]
    pub review_status: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "createdAtMs")]
    pub created_at_ms: String,

    #[serde(rename = "externalUserId")]
    pub external_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookPayloadReviewModel {
    #[serde(rename = "moderationComment")]
    pub moderation_comment: String,

    #[serde(rename = "clientComment")]
    pub client_comment: String,

    #[serde(rename = "reviewAnswer")]
    pub review_answer: String,

    #[serde(rename = "rejectLabels")]
    pub reject_labels: Vec<String>,

    #[serde(rename = "reviewRejectType")]
    pub review_reject_type: String,

    #[serde(rename = "buttonIds")]
    pub button_ids: Vec<String>,
}
