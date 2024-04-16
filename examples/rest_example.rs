use sumsub_connector::rest::config::SumsubConfig;
use sumsub_connector::rest::levels;
use sumsub_connector::rest::rest_client::SumsubRestClient;

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("SECRET_KEY").unwrap();
    let app_token = std::env::var("APP_TOKEN").unwrap();
    let client = SumsubRestClient::new_with_config(secret_key, app_token, SumsubConfig::test_env());
    //let client_id = Uuid::new_v4();
    let client_id = "7de6edc5-bece-4034-a77d-c118a979238c";
    create_poi_access_tokens(&client, client_id.to_string()).await;
    //create_poa_access_tokens(&client, client_id.to_string()).await;

    get_applicant_data(&client, "6569935f69a4d51a3f7a3702").await;
    //get_applicant_status(&client, "654c938106484f31efcf6e6c").await;
    //get_applicant_data(&client, "653124533e2aed1c2c40a0a2").await;
    
    //get_applicant_status(&client, "6569935f69a4d51a3f7a3702").await;
    //get_applicant_docs_status(&client, "6569935f69a4d51a3f7a3702").await;
}

async fn create_poi_access_tokens(client: &SumsubRestClient, client_id: String) {

    let default_level = levels::POI_LEVEL_NAME;
    let access_tokens = client
        .create_access_token(client_id, default_level, None)
        .await;

    println!("create_access_tokens result: {:?}", access_tokens);
}

async fn create_poa_access_tokens(client: &SumsubRestClient, client_id: String) {
    let default_level = levels::POA_LEVEL_NAME;
    let access_tokens = client
        .create_access_token(client_id, default_level, None)
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
    let applicant_data: Result<sumsub_connector::rest::models::GetApplicantStatusResponse, sumsub_connector::rest::errors::Error> = client
        .get_applicant_status(applicant_id.to_string())
        .await;

    println!("get_applicant_status result: {applicant_data:?}");
}

async fn get_applicant_docs_status(client: &SumsubRestClient, applicant_id: &str) {
    let applicant_data = client
        .get_applicant_docs_status(applicant_id.to_string())
        .await;

    println!("get_applicant_docs_status result: {applicant_data:?}");
}