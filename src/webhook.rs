use ring::{digest, hmac, rand};

pub fn validate_signature(webhook_secret_key: &str, data: &str, header_signature: &str) -> bool {
    let v_key = hmac::VerificationKey::new(&digest::SHA1, webhook_secret_key.as_ref());
    let verify = hmac::verify(&v_key, data.as_bytes(), header_signature.as_ref());
    println!("verify - {:?}", verify);
    // hmac sha1 on webhook key
    // hmac update on data
    // base64 encode hmac digest
    // compare value to signature
    true
}
