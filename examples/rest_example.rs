use sumsub_connector::rest::config::SumsubConfig;
use sumsub_connector::rest::levels;
use sumsub_connector::rest::rest_client::SumsubRestClient;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let app_token = std::env::var("APP_TOKEN").unwrap();
    let client = SumsubRestClient::new_with_config(secret_key, app_token, SumsubConfig::test_env());
    create_access_tokens(&client).await;
}

async fn create_access_tokens(client: &SumsubRestClient) {
    let client_id = Uuid::new_v4();
    let default_level = levels::DEFAULT_LEVEL_NAME;
    let access_tokens = client
        .create_access_tokens(client_id.to_string(), default_level, None)
        .await;

    println!("create_access_tokens result: {access_tokens:?}");
}
