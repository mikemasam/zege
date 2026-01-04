use anyhow::Result;
use anyhow::{anyhow, ensure};
use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde_json::Value;

pub struct Security {}
impl Security {
    pub fn hash_password(password: &str) -> Result<String> {
        let hashed = hash(password, DEFAULT_COST)?;
        Ok(hashed)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<()> {
        ensure!(verify(password, hash)?, "Invalid password");
        Ok(())
    }

    pub fn unzip_jwt(token: &str) -> Result<Value> {
        let secret = std::env::var("JWT_SECRET")?;
        let mut validation = Validation::default();
        validation.validate_exp = false;
        validation.insecure_disable_signature_validation();
        let token_data = decode::<Value>(
            token.replace("Bearer ", "").as_str(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )?;
        Ok(token_data.claims)
    }
}
