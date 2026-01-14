
use hmac::{Hmac, Mac};
use sha2::Digest;


pub enum OtpError {
    InvalidSecret,
    InvalidOtp,
    OtpGenerationError,
}


/// Internal: RFC 4226 dynamic truncation
fn dynamic_truncate(hmac_result: &[u8]) -> u32 {
    let offset = (hmac_result[hmac_result.len() - 1] & 0x0f) as usize;
    let p = &hmac_result[offset..offset + 4];
    ((p[0] as u32 & 0x7f) << 24)
        | ((p[1] as u32) << 16)
        | ((p[2] as u32) << 8)
        | (p[3] as u32)
}


fn pow10(digits: u32) -> u32 {
    let mut v = 1;
    for _ in 0..digits {
        v *= 10;
    }
    v
}


/// Generate HOTP using a generic hash function
///
/// * `secret` – raw secret bytes
/// * `counter` – moving factor
/// * `digits` – number of digits (usually 6 or 8)
pub fn generate_hotp<D>(secret: &[u8], counter: u64, digits: u32) -> String
where
    D: Digest,
{
    let msg = counter.to_be_bytes();

    let mut mac = Hmac::<D>::new_from_slice(secret)
        .expect("HMAC supports arbitrary key size");
    mac.update(&msg);
    let result = mac.finalize().into_bytes();

    let code = dynamic_truncate(&result) % pow10(digits);
    format!("{:0width$}", code, width = digits as usize)
}


pub fn generate_otp(secret: &str) -> Result<String, OtpError> {
    // Placeholder implementation
    if secret.is_empty() {
        return Err(OtpError::InvalidSecret);
    }
    // In a real implementation, you would decode the secret, compute the OTP, etc.
    

    Ok("123456".to_string())

}

#[cfg(test)]
mod test {
    use super::*;


    
}