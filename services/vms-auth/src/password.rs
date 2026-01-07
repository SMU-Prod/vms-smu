//! Password hashing and verification using Argon2

use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

/// Hash password using Argon2
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();

    Ok(password_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    #[test]
    fn test_hash_password() {
        let password = "test123";
        let hash = hash_password(password).unwrap();

        // Verify it's a valid Argon2 hash
        assert!(hash.starts_with("$argon2"));

        // Verify password
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        assert!(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok());
    }

    #[test]
    fn test_different_passwords_different_hashes() {
        let hash1 = hash_password("password1").unwrap();
        let hash2 = hash_password("password2").unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_same_password_different_salts() {
        let hash1 = hash_password("password").unwrap();
        let hash2 = hash_password("password").unwrap();
        // Same password should produce different hashes due to different salts
        assert_ne!(hash1, hash2);
    }
}
