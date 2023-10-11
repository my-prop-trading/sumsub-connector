use sha1::{Sha1, Digest};
use sha2::{Sha256, Sha512};
use hmac::{Hmac, Mac};


pub fn validate_webhook (
    x_payload_digest_alg: &String, 
    x_signature: &String, 
    secret_key: &String, 
    body: &[u8]
) -> Result<(), Box<dyn std::error::Error>> {
    let algo: String = match x_payload_digest_alg.as_ref() {
                "HMAC_SHA1_HEX" => "sha1".to_string(),
                "HMAC_SHA256_HEX" => "sha256".to_string(),
                "HMAC_SHA512_HEX" => "sha512".to_string(),
                _ => return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unsupported algorithm",
                ))),
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

        _ => return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Unsupported algorithm {}", algo),
        ))),
    };
}

