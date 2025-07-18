use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;
use crate::errors::AppError;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(
    payload: &[u8],
    signature: &str,
    secret: &str,
) -> Result<(), AppError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| AppError::ConfigError("Invalid HMAC key".into()))?;

    mac.update(payload);

    let provided_signature = signature
        .strip_prefix("sha256=")
        .ok_or(AppError::Unauthorized)?;

    let calculated_signature = hex::encode(mac.finalize().into_bytes());

    // Constant-time comparison
    if subtle::ConstantTimeEq::ct_eq(
        calculated_signature.as_bytes(),
        provided_signature.as_bytes(),
    ).into() {
        Ok(())
    } else {
        Err(AppError::Unauthorized)
    }
}
