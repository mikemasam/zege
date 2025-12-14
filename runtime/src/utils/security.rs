use anyhow::Result;
use anyhow::{anyhow, ensure};
use bcrypt::{DEFAULT_COST, hash, verify};

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
}
