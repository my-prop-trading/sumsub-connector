use crate::rest::config::SumsubConfig;
use crate::rest::endpoints::SumsubEndpoint;
use crate::rest::errors::Error;
use crate::rest::request_signer::RequestSigner;
use error_chain::bail;
use flurl::{FlUrl, FlUrlResponse};
use http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::time::SystemTime;

use super::models::*;

#[derive(Clone)]
pub struct SumsubRestClient {
    signer: RequestSigner,
    app_token: String,
    host: String,
}

impl SumsubRestClient {
    pub fn new(private_key: String, pub_key: String) -> Self {
        Self::new_with_config(private_key, pub_key, SumsubConfig::default())
    }

    pub fn new_with_config(secret_key: String, app_token: String, config: SumsubConfig) -> Self {
        Self {
            signer: RequestSigner::new(secret_key),
            app_token,
            host: config.rest_api_host
        }
    }

    pub async fn create_access_token(
        &self,
        client_id: impl Into<String>,
        level_name: impl Into<String>,
        external_action_id: Option<String>,
    ) -> Result<CreateAccessTokenResponse, Error> {
        let query_params = CreateAccessTokenRequest {
            client_id: client_id.into(),
            level_name: level_name.into(),
            external_action_id,
        };

        let query_params_string = serde_qs::to_string(&query_params).unwrap();
        let query_params_string = format!("?{}", query_params_string);
        let resp: CreateAccessTokenResponse = self
            .post_signed(SumsubEndpoint::AccessTokens, &query_params_string)
            .await?;

        Ok(resp)
    }

    pub async fn get_applicant_data(
        &self,
        applicant_id: impl Into<String>,
    ) -> Result<GetApplicantIdResponse, Error> {
        let query_params = GetApplicantIdRequest {
            applicant_id: applicant_id.into(),
        };
        let query_params_string = format!("/{}/one", query_params.applicant_id.clone());
        let resp: GetApplicantIdResponse = self
            .get_signed(SumsubEndpoint::Applicants, &query_params_string)
            .await?;

        println!("{:?}", serde_json::to_string(&resp).unwrap());

        Ok(resp)
    }

    pub async fn get_applicant_status(
        &self,
        applicant_id: impl Into<String>,
    ) -> Result<GetApplicantStatusResponse, Error> {
        let query_params = GetApplicantStatusRequest {
            applicant_id: applicant_id.into(),
        };
        let query_params_string = format!("/{}/status", query_params.applicant_id.clone());
        let resp: GetApplicantStatusResponse = self
            .get_signed(SumsubEndpoint::Applicants, &query_params_string)
            .await?;

        println!("{:?}", serde_json::to_string(&resp).unwrap());

        Ok(resp)
    }

    pub async fn get_applicant_docs_status(
        &self,
        applicant_id: impl Into<String>,
    ) -> Result<GetApplicantDocsStatusResponse, Error> {
        let query_params = GetApplicantDocsStatusRequest {
            applicant_id: applicant_id.into(),
        };
        let query_params_string = format!("/{}/requiredIdDocsStatus", query_params.applicant_id.clone());
        let resp: GetApplicantDocsStatusResponse = self
            .get_signed(SumsubEndpoint::Applicants, &query_params_string)
            .await?;

        println!("{:?}", serde_json::to_string(&resp).unwrap());

        Ok(resp)
    }

    pub async fn create_applicant(
        &self,
        client_id: impl Into<String>,
        level_name: impl Into<String>,
        external_action_id: Option<String>,
    ) -> Result<CreateAccessTokenResponse, Error> {
        let query_params = CreateAccessTokenRequest {
            client_id: client_id.into(),
            level_name: level_name.into(),
            external_action_id,
        };
        let query_params_string = serde_qs::to_string(&query_params).unwrap();
        let query_params_string = format!("?{}", query_params_string);
        let resp: CreateAccessTokenResponse = self
            .post_signed(SumsubEndpoint::AccessTokens, &query_params_string)
            .await?;

        Ok(resp)
    }

    pub async fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: SumsubEndpoint,
        query_params_string: &str,
    ) -> Result<T, Error> {
        let ts = self.get_request_time();
        //let query_params_string = serde_qs::to_string(&query_params).unwrap();
        let sign = self.signer.generate_sign(
            http::Method::POST.as_str(),
            endpoint.clone(),
            &ts.clone(),
            query_params_string,
        );
        let url_with_query: String = format!(
            "{}{}{}",
            self.host,
            String::from(endpoint),
            query_params_string
        );

        let headers = self.build_headers(&ts.clone(), Some(&sign));
        let mut client = FlUrl::new(url_with_query); // Make client mutable so we can modify it
        
        // Iterate over the headers and add them to the client
        for (key, value) in headers.iter() {
            let header_value = value.to_str().unwrap_or_default();
            client = client.with_header(key.clone().to_string(), header_value); // Modify the client, no move
        }
        
        let mut debug_info = "".to_string();
        let empty_json: Value = json!({}); // Create an empty JSON object
        
        let response = client
            .post_json_with_debug(&empty_json, &mut debug_info) // Use the same client here
            .await;
        match response {
            Ok(res) => {
                self.handler(res, Some(query_params_string.to_owned()))
                .await
            },
            Err(err) => bail!("Failed to call post: {:?}", err),
        }
    }

    pub async fn get_signed<T: DeserializeOwned>(
        &self,
        endpoint: SumsubEndpoint,
        query_params_string: &str,
    ) -> Result<T, Error> {
        let ts = self.get_request_time();
        let sign = self.signer.generate_sign(
            http::Method::GET.as_str(),
            endpoint.clone(),
            &ts.clone(),
            query_params_string,
        );
        let url_with_query: String = format!(
            "{}{}{}",
            self.host,
            String::from(endpoint),
            query_params_string
        );

        let headers = self.build_headers(&ts.clone(), Some(&sign));
        let mut client = FlUrl::new(url_with_query);
        for (key, value) in headers.iter() {
            let header_value = value.to_str().unwrap_or_default();
            client = client.with_header(key.clone().to_string(), header_value);
        }

        let response = client
            .get()
            .await;
        
        match response {
            Ok(res) => {
                let response = self
                    .handler(res, Some(query_params_string.to_owned()))
                    .await;
            response
            },
            Err(err) => bail!("Failed to call get: {:?}", err),
        }
    }

    fn get_request_time(&self) -> String {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        ts.to_string()
    }

    fn build_headers(&self, ts: &str, sign: Option<&str>) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_str("application/json").unwrap(),
        );

        custom_headers.insert(
            HeaderName::from_static("x-app-token"),
            HeaderValue::from_str(&self.app_token).unwrap(),
        );

        custom_headers.insert(
            HeaderName::from_static("x-app-access-ts"),
            HeaderValue::from_str(&ts).unwrap(),
        );

        if let Some(sign) = sign {
            custom_headers.insert(
                HeaderName::from_static("x-app-access-sig"),
                HeaderValue::from_str(sign).unwrap(),
            );
        }

        custom_headers
    }

    async fn handler<T: DeserializeOwned>(
        &self,
        mut response: FlUrlResponse,
        request_json: Option<String>,
    ) -> Result<T, Error> {
        // Get the status code from the response
        let code = match StatusCode::from_u16(response.get_status_code()) {
            Ok(cd) => cd,
            Err(err) => bail!("Failed to read status result: {:?}", err),
        };

        match code {
            StatusCode::OK => {
                let json = response.get_json::<T>().await;
                match json {
                    Ok(data) => Ok(data),  // If successful, return the data
                    Err(err) => bail!("Failed to deserialize body {:?}", err),  // If failure, handle error
                }
            },
            StatusCode::CREATED => {
                let body = match response.receive_body().await {
                    Ok(res) => res.clone(),
                    Err(err) => bail!("Failed to receive_body: {:?}", err),
                };

                let json: Result<String, _> = String::from_utf8(body);
                let Ok(json) = json else {
                    bail!("Failed to read response body");
                };
                let body: Result<T, _> = serde_json::from_str(&json);
                if let Err(err) = body {
                    bail!("Failed to deserialize body {:?}: {}", err, json);
                }

                Ok(body.unwrap())
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                let body = match response.receive_body().await {
                    Ok(res) => res.clone(),
                    Err(err) => bail!("Failed to receive_body: {:?}", err),
                };
                let error = String::from_utf8(body);
                bail!(format!(
                    "Received bad request status. Request: {:?}. Response: {:?}",
                    request_json, error
                ));
            }
            s => {
                let body = match response.receive_body().await {
                    Ok(res) => res.clone(),
                    Err(err) => bail!("Failed to receive_body: {:?}", err),
                };
                let error = String::from_utf8(body);

                bail!(format!("Received response code: {s:?} error: {error:?}"));
            }
        }
    }
}
