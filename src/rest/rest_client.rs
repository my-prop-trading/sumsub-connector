use crate::rest::config::SumsubConfig;
use crate::rest::endpoints::SumsubEndpoint;
use crate::rest::errors::Error;
use crate::rest::models::{CreateAccessTokenRequest, CreateAccessTokenResponse};
use crate::rest::request_signer::RequestSigner;
use error_chain::bail;
use http::header::CONTENT_TYPE;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::time::SystemTime;

#[derive(Clone)]
pub struct SumsubRestClient {
    signer: RequestSigner,
    app_token: String,
    host: String,
    inner_client: reqwest::Client,
}

impl SumsubRestClient {
    pub fn new(private_key: String, pub_key: String) -> Self {
        Self::new_with_config(private_key, pub_key, SumsubConfig::default())
    }

    pub fn new_with_config(secret_key: String, app_token: String, config: SumsubConfig) -> Self {
        Self {
            signer: RequestSigner::new(secret_key),
            app_token,
            host: config.rest_api_host,
            inner_client: reqwest::Client::new(),
        }
    }

    pub async fn create_access_tokens(
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

        let resp: CreateAccessTokenResponse = self
            .post_signed(SumsubEndpoint::AccessToken, query_params)
            .await?;

        Ok(resp)
    }

    pub async fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: SumsubEndpoint,
        query_params: CreateAccessTokenRequest,
    ) -> Result<T, Error> {
        let ts = self.get_request_time();
        let query_params_string = serde_qs::to_string(&query_params).unwrap();
        let sign = self.signer.generate_sign(
            http::Method::POST.as_str(),
            endpoint.clone(),
            &ts.clone(),
            &query_params_string.clone(),
        );
        let url_with_query: String = format!(
            "{}{}?{}",
            self.host,
            String::from(endpoint),
            query_params_string
        );

        let headers = self.build_headers(&ts.clone(), Some(&sign));
        let client = &self.inner_client;
        let response = client
            .post(&url_with_query)
            .headers(headers)
            //.query(&query_params.clone())
            .send()
            .await?;

        //println!("{:?}", response);

        self.handler(response, Some(query_params_string.clone()))
            .await
    }

    // pub async fn get<T: DeserializeOwned>(&self, endpoint: SumsubEndpoint) -> Result<T, Error> {
    //     let url: String = format!("{}{}", self.host, String::from(endpoint));
    //     println!("{}", url);

    //     let client = &self.inner_client;
    //     let headers = self.build_headers(&self.get_request_time(), None);
    //     let response = client.get(url.as_str()).headers(headers).send().await?;

    //     self.handler(response, None).await
    // }

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
            CONTENT_TYPE,
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
        response: Response,
        request_json: Option<String>,
    ) -> Result<T, Error> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>().await?),
            StatusCode::CREATED => {
                let json: Result<String, _> = response.text().await;
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
                let error = response.text().await?;
                bail!(format!(
                    "Received bad request status. Request: {:?}. Response: {:?}",
                    request_json, error
                ));
            }
            s => {
                let error = response.text().await?;

                bail!(format!("Received response code: {s:?} error: {error:?}"));
            }
        }
    }
}
