use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};

pub fn validate_webhook(
    x_payload_digest_alg: &String,
    x_signature: &String,
    secret_key: &String,
    body: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let algo: String = match x_payload_digest_alg.as_ref() {
        "HMAC_SHA1_HEX" => "sha1".to_string(),
        "HMAC_SHA256_HEX" => "sha256".to_string(),
        "HMAC_SHA512_HEX" => "sha512".to_string(),
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported algorithm",
            )))
        }
    };

    let x_signature_bytes = hex::decode(&x_signature).unwrap();

    match algo.as_str() {
        "sha1" => {
            let mut body_signature = Hmac::<Sha1>::new_from_slice(secret_key.as_bytes())?;
            body_signature.update(body);
            body_signature.verify_slice(&x_signature_bytes[..])?;

            return Ok(());
        }
        "sha256" => {
            let mut body_signature = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())?;
            body_signature.update(body);
            body_signature.verify_slice(&x_signature_bytes[..])?;

            return Ok(());
        }
        "sha512" => {
            let mut body_signature = Hmac::<Sha512>::new_from_slice(secret_key.as_bytes())?;
            body_signature.update(body);
            body_signature.verify_slice(&x_signature_bytes[..])?;

            return Ok(());
        }

        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unsupported algorithm {}", algo),
            )))
        }
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_valid_sha256() {
        let x_payload_digest_alg = "HMAC_SHA256_HEX".to_string();
        let secret_key = "ns5mar5x3nf47589uaet9bk5qjs".to_string();
        let body = b"{\n  \"applicantId\" : \"64fb2dde8de7445caabadb4a\",\n  \"inspectionId\" : \"64fb2dde8de7445caabadb4b\",\n  \"applicantType\" : \"individual\",\n  \"correlationId\" : \"56f7ebd7545bf38d8ea17f4781e053fc\",\n  \"levelName\" : \"proof-of-identity-level\",\n  \"sandboxMode\" : true,\n  \"externalUserId\" : \"level-ee704d24-dd24-45e4-b512-1a72576b1dfd\",\n  \"type\" : \"applicantCreated\",\n  \"reviewStatus\" : \"init\",\n  \"createdAt\" : \"2023-10-11 06:08:58+0000\",\n  \"createdAtMs\" : \"2023-10-11 06:08:58.546\",\n  \"clientId\" : \"platos_ltd\"\n}";
        let x_signature =
            "6fc6ad7cafb7e963ed6d4e06cdeab24a4a21239b4de83c6c2cd47b7e412eb3b4".to_string();

        assert!(validate_webhook(&x_payload_digest_alg, &x_signature, &secret_key, body).is_ok());
    }
}
