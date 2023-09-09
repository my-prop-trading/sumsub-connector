use hmac::{Hmac, Mac};
use sha2::Sha256;

use super::endpoints::SumsubEndpoint;

#[derive(Clone)]
pub struct RequestSigner {
    secret_key: String,
}

impl RequestSigner {
    pub fn new(secret_key: String) -> Self {
        Self {
            secret_key: secret_key,
        }
    }

    pub fn generate_sign(
        &self,
        http_method: &str,
        endpoint: SumsubEndpoint,
        ts: &str,
        query_params: &str,
    ) -> String {
        let mut signed_key = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes()).unwrap();
        let string_to_sign: String = format!(
            "{}{}{}{}",
            ts,
            http_method,
            String::from(endpoint),
            query_params
        );

        println!("{:?}", string_to_sign);

        signed_key.update(string_to_sign.as_bytes());

        hex::encode(signed_key.finalize().into_bytes())
    }
}
