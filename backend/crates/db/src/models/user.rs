//! User model (Epic 1, Story 1.1).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User status enumeration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum UserStatus {
    Pending,
    Active,
    Suspended,
    Deleted,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Pending => "pending",
            UserStatus::Active => "active",
            UserStatus::Suspended => "suspended",
            UserStatus::Deleted => "deleted",
        }
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Supported locales for email templates.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum Locale {
    #[sqlx(rename = "sk")]
    Slovak,
    #[sqlx(rename = "cs")]
    Czech,
    #[sqlx(rename = "de")]
    German,
    #[sqlx(rename = "en")]
    #[default]
    English,
}

impl Locale {
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::Slovak => "sk",
            Locale::Czech => "cs",
            Locale::German => "de",
            Locale::English => "en",
        }
    }

    /// Parse locale from string.
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "sk" | "sk-sk" => Locale::Slovak,
            "cs" | "cs-cz" => Locale::Czech,
            "de" | "de-de" | "de-at" | "de-ch" => Locale::German,
            _ => Locale::English,
        }
    }
}

/// User entity from database.
#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub phone: Option<String>,
    pub status: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub invited_at: Option<DateTime<Utc>>,
    pub invited_by: Option<Uuid>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
    pub locale: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Check if user account is active and verified.
    pub fn is_active(&self) -> bool {
        self.status == "active" && self.email_verified_at.is_some()
    }

    /// Check if user can log in.
    pub fn can_login(&self) -> bool {
        self.status == "active"
    }

    /// Check if email is verified.
    pub fn is_verified(&self) -> bool {
        self.email_verified_at.is_some()
    }

    /// Get status as enum.
    pub fn status_enum(&self) -> UserStatus {
        match self.status.as_str() {
            "pending" => UserStatus::Pending,
            "active" => UserStatus::Active,
            "suspended" => UserStatus::Suspended,
            "deleted" => UserStatus::Deleted,
            _ => UserStatus::Pending,
        }
    }

    /// Get locale as enum.
    pub fn locale_enum(&self) -> Locale {
        Locale::parse(&self.locale)
    }
}

/// Data for creating a new user.
#[derive(Debug, Clone)]
pub struct CreateUser {
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub phone: Option<String>,
    pub locale: Locale,
}

/// Data for updating a user.
#[derive(Debug, Clone, Default)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub locale: Option<Locale>,
}

/// Email verification token entity.
#[derive(Debug, Clone, FromRow)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl EmailVerificationToken {
    /// Check if token is expired.
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    /// Check if token has been used.
    pub fn is_used(&self) -> bool {
        self.used_at.is_some()
    }

    /// Check if token is valid (not expired and not used).
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_used()
    }
}
