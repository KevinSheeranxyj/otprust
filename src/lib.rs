

pub enum OtpError {
    InvalidSecret,
    InvalidOtp,
    OtpGenerationError,
}


pub fn generate_otp(secret: &str) -> Result<String, OtpError> {
    // Placeholder implementation
    if secret.is_empty() {
        return Err(OtpError::InvalidSecret);
    }
    // In a real implementation, you would decode the secret, compute the OTP, etc.
    

    // Decode the secret
    let decoded_secret = base64::decode(secret).map_err(|_| OtpError::InvalidSecret)?;

    Ok("123456".to_string())

}
