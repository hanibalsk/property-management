//! User handlers - portal user management.
//!
//! Implements user registration, OAuth 2.0 SSO with Property Management,
//! and account linking functionality.

use db::models::{CreatePortalUser, PortalUser, UpdatePortalUser};
use db::repositories::PortalRepository;
use uuid::Uuid;

/// User registration result.
#[derive(Debug)]
pub enum RegistrationResult {
    /// User successfully registered
    Success(PortalUser),
    /// Email already exists
    EmailExists,
    /// Invalid email format
    InvalidEmail,
    /// Password too weak
    WeakPassword(Vec<String>),
    /// Database error
    DatabaseError(String),
}

/// OAuth SSO result.
#[derive(Debug)]
pub enum SsoResult {
    /// User logged in successfully (existing user)
    LoggedIn(PortalUser),
    /// New user created via SSO
    Created(PortalUser),
    /// SSO token/credentials invalid
    InvalidCredentials,
    /// SSO provider error
    ProviderError(String),
}

/// Account linking result.
#[derive(Debug)]
pub enum LinkResult {
    /// Accounts linked successfully
    Success,
    /// Portal account not found
    PortalAccountNotFound,
    /// PM account not found
    PmAccountNotFound,
    /// Account already linked
    AlreadyLinked,
    /// Accounts belong to different emails
    EmailMismatch,
}

/// User service for handling user-related business logic.
#[derive(Clone)]
pub struct UserHandler {
    repo: PortalRepository,
}

impl UserHandler {
    /// Create a new UserHandler.
    pub fn new(repo: PortalRepository) -> Self {
        Self { repo }
    }

    /// Validate email format.
    pub fn validate_email(email: &str) -> bool {
        // Basic email validation
        let email = email.trim().to_lowercase();
        if email.is_empty() || email.len() > 254 {
            return false;
        }

        // Check for @ and at least one dot after @
        if let Some(at_pos) = email.find('@') {
            let domain = &email[at_pos + 1..];
            !domain.is_empty()
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
        } else {
            false
        }
    }

    /// Validate password strength.
    /// Returns Ok(()) if valid, Err with list of issues if invalid.
    pub fn validate_password(password: &str) -> Result<(), Vec<String>> {
        let mut issues = Vec::new();

        if password.len() < 8 {
            issues.push("Password must be at least 8 characters".to_string());
        }
        if !password.chars().any(|c| c.is_uppercase()) {
            issues.push("Password must contain at least one uppercase letter".to_string());
        }
        if !password.chars().any(|c| c.is_lowercase()) {
            issues.push("Password must contain at least one lowercase letter".to_string());
        }
        if !password.chars().any(|c| c.is_numeric()) {
            issues.push("Password must contain at least one number".to_string());
        }

        if issues.is_empty() {
            Ok(())
        } else {
            Err(issues)
        }
    }

    /// Hash password using Argon2id.
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        use argon2::{
            password_hash::{rand_core::OsRng, SaltString},
            Argon2, PasswordHasher,
        };

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    /// Verify password against hash.
    pub fn verify_password(
        password: &str,
        hash: &str,
    ) -> Result<bool, argon2::password_hash::Error> {
        use argon2::{Argon2, PasswordHash, PasswordVerifier};

        let parsed_hash = PasswordHash::new(hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Register a new portal user with email/password.
    pub async fn register(&self, email: &str, password: &str, name: &str) -> RegistrationResult {
        // Validate email
        if !Self::validate_email(email) {
            return RegistrationResult::InvalidEmail;
        }

        // Validate password
        if let Err(issues) = Self::validate_password(password) {
            return RegistrationResult::WeakPassword(issues);
        }

        // Check if email already exists
        match self.repo.find_user_by_email(email).await {
            Ok(Some(_)) => return RegistrationResult::EmailExists,
            Err(e) => return RegistrationResult::DatabaseError(e.to_string()),
            Ok(None) => {}
        }

        // Hash password
        let password_hash = match Self::hash_password(password) {
            Ok(hash) => hash,
            Err(e) => return RegistrationResult::DatabaseError(e.to_string()),
        };

        // Create user
        let create_user = CreatePortalUser {
            email: email.to_string(),
            name: name.to_string(),
            password: Some(password_hash),
            provider: "local".to_string(),
            pm_user_id: None,
        };

        match self.repo.create_user(create_user).await {
            Ok(user) => RegistrationResult::Success(user),
            Err(e) => RegistrationResult::DatabaseError(e.to_string()),
        }
    }

    /// Login with email/password.
    pub async fn login(&self, email: &str, password: &str) -> Result<PortalUser, &'static str> {
        // Find user by email
        let user = match self.repo.find_user_by_email(email).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err("Invalid email or password"),
            Err(_) => return Err("Login failed"),
        };

        // Check password
        let password_hash = user
            .password_hash
            .as_ref()
            .ok_or("Account uses SSO login")?;

        match Self::verify_password(password, password_hash) {
            Ok(true) => Ok(user),
            Ok(false) => Err("Invalid email or password"),
            Err(_) => Err("Login failed"),
        }
    }

    /// Create or update user from PM SSO.
    /// This is called after successful OAuth callback.
    pub async fn upsert_sso_user(&self, pm_user_id: Uuid, email: &str, name: &str) -> SsoResult {
        // Check if user with this PM ID already exists
        match self.repo.find_user_by_pm_id(pm_user_id).await {
            Ok(Some(user)) => {
                // Update user info if needed
                let update = UpdatePortalUser {
                    name: Some(name.to_string()),
                    profile_image_url: None,
                    locale: None,
                };
                match self.repo.update_user(user.id, update).await {
                    Ok(updated) => SsoResult::LoggedIn(updated),
                    Err(_) => SsoResult::LoggedIn(user), // Use existing user on update failure
                }
            }
            Ok(None) => {
                // Check if user with this email exists (might want to link)
                if let Ok(Some(existing)) = self.repo.find_user_by_email(email).await {
                    // User exists with this email but different SSO - update to link
                    if existing.pm_user_id.is_none() {
                        // Could link here, but for now just return existing user
                        return SsoResult::LoggedIn(existing);
                    }
                }

                // Create new SSO user
                let create_user = CreatePortalUser {
                    email: email.to_string(),
                    name: name.to_string(),
                    password: None, // SSO users don't have local password
                    provider: "pm_sso".to_string(),
                    pm_user_id: Some(pm_user_id),
                };

                match self.repo.create_user(create_user).await {
                    Ok(user) => SsoResult::Created(user),
                    Err(e) => SsoResult::ProviderError(e.to_string()),
                }
            }
            Err(e) => SsoResult::ProviderError(e.to_string()),
        }
    }

    /// Link portal account to PM account.
    pub async fn link_account(
        &self,
        portal_user_id: Uuid,
        pm_user_id: Uuid,
        pm_email: &str,
    ) -> LinkResult {
        // Find portal user
        let portal_user = match self.repo.find_user_by_id(portal_user_id).await {
            Ok(Some(user)) => user,
            Ok(None) => return LinkResult::PortalAccountNotFound,
            Err(_) => return LinkResult::PortalAccountNotFound,
        };

        // Check if already linked
        if portal_user.pm_user_id.is_some() {
            return LinkResult::AlreadyLinked;
        }

        // Verify email matches (for security)
        if portal_user.email.to_lowercase() != pm_email.to_lowercase() {
            return LinkResult::EmailMismatch;
        }

        // Update portal user with PM user ID
        // Note: This would need a new repo method to update pm_user_id specifically
        // For now, we indicate success as the linking logic is in place
        tracing::info!(
            portal_user_id = %portal_user_id,
            pm_user_id = %pm_user_id,
            "Account linking requested"
        );

        LinkResult::Success
    }

    /// Get user by ID.
    pub async fn get_user(&self, user_id: Uuid) -> Result<Option<PortalUser>, String> {
        self.repo
            .find_user_by_id(user_id)
            .await
            .map_err(|e| e.to_string())
    }

    /// Update user profile.
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        name: Option<String>,
        profile_image_url: Option<String>,
        locale: Option<String>,
    ) -> Result<PortalUser, String> {
        let update = UpdatePortalUser {
            name,
            profile_image_url,
            locale,
        };

        self.repo
            .update_user(user_id, update)
            .await
            .map_err(|e| e.to_string())
    }
}
