use crate::errors::AppError;

pub fn verify_token(token: &str, expected_token: &str) -> Result<(), AppError> {
    if token != expected_token {
        Err(AppError::Unauthorized)
    } else {
        Ok(())
    }
}
