//! TOTP (Time-based One-Time Password) service for 2FA (Epic 9, Story 9.1).

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::Rng;
use thiserror::Error;
use totp_rs::{Algorithm, Secret, TOTP};

/// Errors that can occur during TOTP operations.
#[derive(Debug, Error)]
pub enum TotpError {
    #[error("Failed to generate secret: {0}")]
    SecretGenerationError(String),
    #[error("Failed to create TOTP instance: {0}")]
    TotpCreationError(String),
    #[error("Invalid TOTP code")]
    InvalidCode,
    #[error("Failed to hash backup code: {0}")]
    HashError(String),
    #[error("Failed to verify backup code: {0}")]
    VerifyError(String),
}

/// Service for TOTP operations.
#[derive(Clone)]
pub struct TotpService {
    /// Issuer name for the TOTP (shows in authenticator app)
    issuer: String,
    /// Number of backup codes to generate
    backup_code_count: usize,
    /// Length of each backup code
    backup_code_length: usize,
}

impl TotpService {
    /// Create a new TOTP service.
    pub fn new(issuer: String) -> Self {
        Self {
            issuer,
            backup_code_count: 10,
            backup_code_length: 8,
        }
    }

    /// Generate a new TOTP secret.
    /// Returns a base32-encoded secret suitable for authenticator apps.
    pub fn generate_secret(&self) -> Result<String, TotpError> {
        let secret = Secret::generate_secret();
        Ok(secret.to_encoded().to_string())
    }

    /// Verify a TOTP code against a secret.
    /// Uses a 30-second window and allows for 1 step skew (±30 seconds).
    pub fn verify_code(&self, secret: &str, code: &str) -> Result<bool, TotpError> {
        let secret_bytes = Secret::Encoded(secret.to_string())
            .to_bytes()
            .map_err(|e| TotpError::TotpCreationError(e.to_string()))?;

        let totp = TOTP::new(
            Algorithm::SHA1,
            6,  // digits
            1,  // skew (±1 step = ±30 seconds)
            30, // step in seconds
            secret_bytes,
            Some(self.issuer.clone()), // issuer
            "".to_string(),            // account_name (not used for verification)
        )
        .map_err(|e| TotpError::TotpCreationError(e.to_string()))?;

        Ok(totp.check_current(code).unwrap_or(false))
    }

    /// Generate a URI for QR code display.
    /// This URI can be encoded into a QR code for authenticator apps to scan.
    pub fn generate_qr_uri(&self, email: &str, secret: &str) -> Result<String, TotpError> {
        let secret_bytes = Secret::Encoded(secret.to_string())
            .to_bytes()
            .map_err(|e| TotpError::TotpCreationError(e.to_string()))?;

        let totp = TOTP::new(
            Algorithm::SHA1,
            6,  // digits
            1,  // skew
            30, // step
            secret_bytes,
            Some(self.issuer.clone()), // issuer
            email.to_string(),         // account_name
        )
        .map_err(|e| TotpError::TotpCreationError(e.to_string()))?;

        // Generate the otpauth:// URI
        Ok(totp.get_url())
    }

    /// Generate backup codes.
    /// Returns a tuple of (plain codes for display, hashed codes for storage).
    pub fn generate_backup_codes(&self) -> Result<(Vec<String>, Vec<String>), TotpError> {
        let mut plain_codes = Vec::with_capacity(self.backup_code_count);
        let mut hashed_codes = Vec::with_capacity(self.backup_code_count);

        for _ in 0..self.backup_code_count {
            let code = self.generate_random_code();
            let hashed = self.hash_backup_code(&code)?;
            plain_codes.push(code);
            hashed_codes.push(hashed);
        }

        Ok((plain_codes, hashed_codes))
    }

    /// Verify a backup code against a list of hashed codes.
    /// Returns the index of the matching code if found.
    pub fn verify_backup_code(
        &self,
        code: &str,
        hashed_codes: &[String],
    ) -> Result<Option<usize>, TotpError> {
        // Normalize the code (remove any dashes/spaces, uppercase)
        let normalized_code = code.replace(['-', ' '], "").to_uppercase();

        for (index, hashed) in hashed_codes.iter().enumerate() {
            // Skip empty/used codes
            if hashed.is_empty() {
                continue;
            }

            if self.verify_backup_code_hash(&normalized_code, hashed)? {
                return Ok(Some(index));
            }
        }

        Ok(None)
    }

    /// Generate a random backup code.
    fn generate_random_code(&self) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
        let mut rng = rand::thread_rng();

        (0..self.backup_code_length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Hash a backup code using Argon2.
    fn hash_backup_code(&self, code: &str) -> Result<String, TotpError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(code.as_bytes(), &salt)
            .map_err(|e| TotpError::HashError(e.to_string()))?;
        Ok(hash.to_string())
    }

    /// Verify a backup code against its hash.
    fn verify_backup_code_hash(&self, code: &str, hash: &str) -> Result<bool, TotpError> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| TotpError::VerifyError(e.to_string()))?;
        Ok(Argon2::default()
            .verify_password(code.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

impl Default for TotpService {
    fn default() -> Self {
        Self::new("Property Management".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret() {
        let service = TotpService::default();
        let secret = service.generate_secret().unwrap();
        assert!(!secret.is_empty());
        // Base32 encoded secrets should only contain valid chars
        assert!(secret
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '='));
    }

    #[test]
    fn test_generate_qr_uri() {
        let service = TotpService::default();
        let secret = service.generate_secret().unwrap();
        let uri = service
            .generate_qr_uri("test@example.com", &secret)
            .unwrap();

        assert!(uri.starts_with("otpauth://totp/"));
        assert!(uri.contains("test@example.com") || uri.contains("test%40example.com"));
        assert!(uri.contains("issuer=Property"));
    }

    #[test]
    fn test_generate_backup_codes() {
        let service = TotpService::default();
        let (plain, hashed) = service.generate_backup_codes().unwrap();

        assert_eq!(plain.len(), 10);
        assert_eq!(hashed.len(), 10);

        // Each plain code should be 8 characters
        for code in &plain {
            assert_eq!(code.len(), 8);
        }

        // Hashed codes should be Argon2 format
        for hash in &hashed {
            assert!(hash.starts_with("$argon2"));
        }
    }

    #[test]
    fn test_verify_backup_code() {
        let service = TotpService::default();
        let (plain, hashed) = service.generate_backup_codes().unwrap();

        // Should find the first code
        let result = service.verify_backup_code(&plain[0], &hashed).unwrap();
        assert_eq!(result, Some(0));

        // Should find the last code
        let result = service.verify_backup_code(&plain[9], &hashed).unwrap();
        assert_eq!(result, Some(9));

        // Invalid code should return None
        let result = service.verify_backup_code("INVALID1", &hashed).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_verify_backup_code_with_dashes() {
        let service = TotpService::default();
        let (plain, hashed) = service.generate_backup_codes().unwrap();

        // Should work with dashes
        let code_with_dashes = format!("{}-{}", &plain[0][..4], &plain[0][4..]);
        let result = service
            .verify_backup_code(&code_with_dashes, &hashed)
            .unwrap();
        assert_eq!(result, Some(0));
    }
}
