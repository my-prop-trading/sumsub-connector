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
    pub applicant_type: ApplicantType,

    #[serde(rename = "correlationId")]
    pub correlation_id: String,

    #[serde(rename = "levelName")]
    pub level_name: String,

    #[serde(rename = "sandboxMode")]
    pub sandbox_mode: bool,

    #[serde(rename = "type")]
    pub webhookType: WebhookType,

    #[serde(rename = "reviewResult")]
    pub review_result: Option<WebhookPayloadReviewModel>,

    #[serde(rename = "reviewStatus")]
    pub review_status: ReviewStatus,

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReviewStatus {
    #[serde(rename = "init")]
    Init = 0,
    #[serde(rename = "pending")]
    Pending = 1,
    #[serde(rename = "prechecked")]
    Prechecked = 2,
    #[serde(rename = "queued")]
    Queued =3,
    #[serde(rename = "completed")]
    Completed = 4,
    #[serde(rename = "onHold")]
    OnHold = 5,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebhookType {
    #[serde(rename = "applicantReviewed")]
    ApplicantReviewed = 0,
    #[serde(rename = "applicantPending")]
    ApplicantPending = 1,
    #[serde(rename = "applicantCreated")]
    ApplicantCreated = 2,
    #[serde(rename = "applicantOnHold")]
    ApplicantOnHold = 3,
    #[serde(rename = "applicantPersonalInfoChanged")]
    ApplicantPersonalInfoChanged = 4,
    #[serde(rename = "applicantPrechecked")]
    ApplicantPrechecked = 5,
    #[serde(rename = "applicantDeleted")]
    ApplicantDeleted = 6,
    #[serde(rename = "applicantLevelChanged")]
    ApplicantLevelChanged = 7,
    #[serde(rename = "videoIdentStatusChanged")]
    VideoIdentStatusChanged = 8,
    #[serde(rename = "applicantReset")]
    ApplicantReset = 9,
    #[serde(rename = "applicantActionPending")]
    ApplicantActionPending = 10,
    #[serde(rename = "applicantActionReviewed")]
    ApplicantActionReviewed = 11,
    #[serde(rename = "applicantActionOnHold")]
    ApplicantActionOnHold = 12,
    #[serde(rename = "applicantTravelRuleStatusChanged")]
    ApplicantTravelRuleStatusChanged = 13,
    #[serde(rename = "applicantWorkflowCompleted")]
    ApplicantWorkflowCompleted = 14,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ApplicantType {
    #[serde(rename = "company")]
    Company = 0,
    #[serde(rename = "individual")]
    Individual = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetApplicantIdRequest {
    #[serde(rename = "applicantId")]
    pub applicant_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetApplicantIdResponse {
    #[serde(rename = "id")]
    pub applicant_id: String,
    #[serde(rename = "externalUserId")]
    pub client_id: String,
    #[serde(rename = "fixedInfo")]
    pub fixed_info: Option<FixedInfoModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FixedInfoModel
{
    #[serde(rename ="firstName")]
    pub first_name: String,

    #[serde(rename = "lastName")]
    pub last_name: String,
}