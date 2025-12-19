//! Common data types.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Physical address.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct Address {
    /// Street address line 1
    #[validate(length(min = 1, max = 255))]
    pub street1: String,

    /// Street address line 2 (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,

    /// City name
    #[validate(length(min = 1, max = 100))]
    pub city: String,

    /// State or province
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Postal/ZIP code
    #[validate(length(min = 1, max = 20))]
    pub postal_code: String,

    /// ISO 3166-1 alpha-2 country code
    #[validate(length(equal = 2))]
    pub country: String,
}

/// Monetary amount with currency.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct Money {
    /// Amount in smallest currency unit (e.g., cents)
    pub amount: i64,

    /// ISO 4217 currency code
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn eur(amount: i64) -> Self {
        Self::new(amount, Currency::EUR)
    }

    pub fn usd(amount: i64) -> Self {
        Self::new(amount, Currency::USD)
    }

    /// Get amount as floating point (for display).
    pub fn as_decimal(&self) -> f64 {
        self.amount as f64 / 100.0
    }
}

/// Currency codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum Currency {
    EUR,
    USD,
    GBP,
    CZK,
    PLN,
    HUF,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::EUR => write!(f, "EUR"),
            Currency::USD => write!(f, "USD"),
            Currency::GBP => write!(f, "GBP"),
            Currency::CZK => write!(f, "CZK"),
            Currency::PLN => write!(f, "PLN"),
            Currency::HUF => write!(f, "HUF"),
        }
    }
}

/// GPS coordinates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct GeoLocation {
    /// Latitude (-90 to 90)
    pub latitude: f64,

    /// Longitude (-180 to 180)
    pub longitude: f64,
}

impl GeoLocation {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
}

/// File attachment metadata.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Attachment {
    /// Unique identifier
    pub id: uuid::Uuid,

    /// Original filename
    pub filename: String,

    /// MIME type
    pub mime_type: String,

    /// File size in bytes
    pub size_bytes: i64,

    /// Download URL
    pub url: String,

    /// Upload timestamp
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
}

/// Pagination query parameters.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct PaginationQuery {
    /// Page number (1-indexed)
    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: i32,

    /// Items per page
    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_limit")]
    pub limit: i32,

    /// Sort field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// Sort direction
    #[serde(default)]
    pub sort_order: SortOrder,
}

fn default_page() -> i32 {
    1
}

fn default_limit() -> i32 {
    20
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 20,
            sort_by: None,
            sort_order: SortOrder::Asc,
        }
    }
}

impl PaginationQuery {
    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.limit) as i64
    }
}

/// Sort direction.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    #[default]
    Asc,
    Desc,
}

/// Pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    /// Current page number
    pub page: i32,

    /// Items per page
    pub limit: i32,

    /// Total number of items
    pub total_items: i64,

    /// Total number of pages
    pub total_pages: i32,

    /// Has next page
    pub has_next: bool,

    /// Has previous page
    pub has_previous: bool,
}

impl PaginationMeta {
    pub fn new(page: i32, limit: i32, total_items: i64) -> Self {
        let total_pages = ((total_items as f64) / (limit as f64)).ceil() as i32;
        Self {
            page,
            limit,
            total_items,
            total_pages,
            has_next: page < total_pages,
            has_previous: page > 1,
        }
    }
}

/// Paginated response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    /// Array of items
    pub data: Vec<T>,

    /// Pagination metadata
    pub pagination: PaginationMeta,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, page: i32, limit: i32, total_items: i64) -> Self {
        Self {
            data,
            pagination: PaginationMeta::new(page, limit, total_items),
        }
    }
}
