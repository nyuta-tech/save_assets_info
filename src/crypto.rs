use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn sign_hmac(api_secret: &str, msg: &str) -> String {
    // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(api_secret.as_bytes()).expect("Success");
    mac.update(msg.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let signature = format!("{:X}", code_bytes);

    signature.to_lowercase()
}
