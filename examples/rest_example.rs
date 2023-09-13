use sumsub_connector::rest::config::SumsubConfig;
use sumsub_connector::rest::levels;
use sumsub_connector::rest::rest_client::SumsubRestClient;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let app_token = std::env::var("APP_TOKEN").unwrap();
    let client = SumsubRestClient::new_with_config(secret_key, app_token, SumsubConfig::test_env());
    // create_access_tokens(&client).await;
    // get_applicant_data(&client, "64fb3ea46911e17c9dd2eb93").await;
    // get_applicant_status(&client, "64fb3ea46911e17c9dd2eb93").await;
    get_applicant_data(&client, "650169a3dea0060dd881165a").await;
    get_applicant_status(&client, "650169a3dea0060dd881165a").await;
}

async fn create_access_tokens(client: &SumsubRestClient) {
    let client_id = Uuid::new_v4();
    let default_level = levels::POI_LEVEL_NAME;
    let access_tokens = client
        .create_access_token(client_id.to_string(), default_level, None)
        .await;

    println!("create_access_tokens result: {access_tokens:?}");
}

async fn get_applicant_data(client: &SumsubRestClient, applicant_id: &str) {
    let applicant_data = client
        .get_applicant_data(applicant_id.to_string())
        .await;

    println!("get_applicant_data result: {applicant_data:?}");
}


async fn get_applicant_status(client: &SumsubRestClient, applicant_id: &str) {
    let applicant_data = client
        .get_applicant_status(applicant_id.to_string())
        .await;

    println!("get_applicant_status result: {applicant_data:?}");
}