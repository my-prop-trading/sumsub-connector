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
    pub payload_type: WebhookType,

    #[serde(rename = "reviewResult")]
    pub review_result: Option<ReviewResultModel>,

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
pub struct ReviewResultModel {
    #[serde(rename = "moderationComment", skip_serializing_if = "Option::is_none")]
    pub moderation_comment: Option<String>,

    #[serde(rename = "clientComment", skip_serializing_if = "Option::is_none")]
    pub client_comment: Option<String>,

    #[serde(rename = "reviewAnswer")]
    pub review_answer: String,

    #[serde(rename = "rejectLabels", skip_serializing_if = "Option::is_none")]
    pub reject_labels: Option<Vec<String>>,

    #[serde(rename = "reviewRejectType", skip_serializing_if = "Option::is_none")]
    pub review_reject_type: Option<String>,

    #[serde(rename = "buttonIds", skip_serializing_if = "Option::is_none")]
    pub button_ids: Option<Vec<String>>,
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
    #[serde(rename = "inspectionId")]
    pub inspection_id: String,
    #[serde(rename = "externalUserId")]
    pub client_id: String,
    #[serde(rename = "sourceKey", skip_serializing_if = "Option::is_none")]
    pub source_key: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<MetadataModel>>,
    #[serde(rename = "fixedInfo", skip_serializing_if = "Option::is_none")]
    pub fixed_info: Option<InfoModel>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<InfoModel>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "requiredIdDocs", skip_serializing_if = "Option::is_none")]
    pub required_id_docs: Option<RequiredIdDocsModel>,
    #[serde(rename = "review")]
    pub review: Option<ReviewModel>,
    #[serde(rename = "questionnaires", skip_serializing_if = "Option::is_none")]
    pub questionnaires: Option<Vec<QuestionnairesModel>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataModel{
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoModel
{
    #[serde(rename ="firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "firstNameEn")]
    pub first_name_en: Option<String>,
    #[serde(rename = "lastNameEn")]
    pub last_name_en: Option<String>,
    #[serde(rename = "middleNameEn")]
    pub middle_name_en: Option<String>,
    #[serde(rename = "legalName")]
    pub legal_name: Option<String>,
    #[serde(rename = "gender")]
    pub gender: Option<String>,
    #[serde(rename = "dob")]
    pub dob: Option<String>,
    #[serde(rename = "placeOfBirth")]
    pub place_of_birth: Option<String>,
    #[serde(rename = "countryOfBirth")]
    pub country_of_birth: Option<String>,
    #[serde(rename = "stateOfBirth")]
    pub state_of_birth: Option<String>,
    #[serde(rename = "country")]
    pub country_iso3: Option<String>,
    #[serde(rename = "nationality")]
    pub nationality_iso3: Option<String>,
    #[serde(rename = "addresses")]
    pub addresses: Option<Vec<AddressModel>>,
    #[serde(rename = "idDocs")]
    pub id_docs: Option<Vec<DocumentModel>>,
    #[serde(rename = "tin")]
    pub tin: Option<String>

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequiredIdDocsModel{
    #[serde(rename = "docSets")]
    pub doc_sets: Option<Vec<IdDocSet>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IdDocSet {
    #[serde(rename = "idDocSetType")]
    pub id_doc_set_type: String,
    #[serde(rename = "types")]
    pub types: Vec<String>,
    #[serde(rename = "videoRequired", skip_serializing_if = "Option::is_none")]
    pub video_required: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewModel{
    #[serde(rename = "elapsedSincePendingMs", skip_serializing_if = "Option::is_none")]
    elapsed_since_pending_ms: Option<i64>,
    #[serde(rename = "elapsedSinceQueuedMs", skip_serializing_if = "Option::is_none")]
    elapsed_since_queued_ms: Option<i64>,
    #[serde(rename = "reprocessing", skip_serializing_if = "Option::is_none")]
    reprocessing: Option<bool>,
    #[serde(rename = "levelName")]
    pub level_name: String,
    #[serde(rename = "createDate")]
    create_date: String,
    #[serde(rename = "reviewDate", skip_serializing_if = "Option::is_none")]
    review_date: Option<String>,
    #[serde(rename = "expireDate", skip_serializing_if = "Option::is_none")]
    expire_date: Option<String>,
    #[serde(rename = "reviewResult", skip_serializing_if = "Option::is_none")]
    review_result: Option<ReviewResultModel>,
    #[serde(rename = "reviewStatus")]
    review_status: ReviewStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestionnairesModel{

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddressModel{
    #[serde(rename = "country")]
    pub country_iso3: Option<String>,
    #[serde(rename = "postCode")]
    pub post_code: Option<String>,
    #[serde(rename = "town")]
    pub town: Option<String>,
    #[serde(rename = "street")]
    pub street: Option<String>,
    #[serde(rename = "subStreet")]
    pub sub_street: Option<String>,
    #[serde(rename = "state")]
    pub state: Option<String>,
    #[serde(rename = "buildingName")]
    pub building_name: Option<String>,
    #[serde(rename = "flatNumber")]
    pub flat_number: Option<String>,
    #[serde(rename = "buildingNumber")]
    pub building_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentModel{
    #[serde(rename = "idDocType")]
    pub id_doc_type: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "firstNameEn")]
    pub first_name_en: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "lastNameEn")]
    pub last_name_en: String,
    #[serde(rename = "validUntil")]
    pub valid_until: String,
    #[serde(rename = "number")]
    pub number: String,
    #[serde(rename = "dob")]
    pub dob: String,
    #[serde(rename = "mrzLine1")]
    pub mrz_line1: String,
    #[serde(rename = "mrzLine2")]
    pub mrz_line2: String,
    #[serde(rename = "mrzLine3")]
    pub mrz_line3: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetApplicantStatusRequest {
    #[serde(rename = "applicantId")]
    pub applicant_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetApplicantStatusResponse {
    #[serde(rename = "createDate")]
    pub created_date: String,
    #[serde(rename = "reviewDate", skip_serializing_if = "Option::is_none")]
    pub review_date: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "reviewResult", skip_serializing_if = "Option::is_none")]
    pub review: Option<ReviewResultModel>,
    #[serde(rename = "reviewStatus")]
    pub review_status: ReviewStatus,
    #[serde(rename = "levelName", skip_serializing_if = "Option::is_none")]
    pub level_name: Option<String>,
    #[serde(rename = "attemptCnt", skip_serializing_if = "Option::is_none")]
    pub attempt_cnt: Option<i64>,
}

