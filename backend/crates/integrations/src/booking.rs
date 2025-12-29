//! Booking.com integration client (Story 83.2).
//!
//! Implements Booking.com Connectivity API integration,
//! including OTA XML message handling for reservations and availability.

use chrono::{DateTime, Duration, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

// ============================================
// Error Types
// ============================================

/// Errors that can occur during Booking.com API operations.
#[derive(Debug, Error)]
pub enum BookingError {
    /// OAuth/authentication error.
    #[error("Authentication error: {0}")]
    Auth(String),

    /// API error from Booking.com.
    #[error("API error: {0}")]
    Api(String),

    /// Network/HTTP error.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// XML parsing error.
    #[error("XML error: {0}")]
    Xml(String),

    /// Rate limit exceeded.
    #[error("Rate limit exceeded, retry after {0} seconds")]
    RateLimited(u64),

    /// Push operation failed.
    #[error("Push failed: {0}")]
    PushFailed(String),

    /// Invalid OTA message.
    #[error("Invalid OTA message: {0}")]
    InvalidMessage(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

// ============================================
// Authentication Types
// ============================================

/// Booking.com API credentials.
#[derive(Debug, Clone)]
pub struct BookingCredentials {
    /// Hotel ID assigned by Booking.com.
    pub hotel_id: String,
    /// API username.
    pub username: String,
    /// API password.
    pub password: String,
    /// API endpoint URL.
    pub api_url: String,
}

impl BookingCredentials {
    /// Create credentials with the standard production API URL.
    pub fn new(hotel_id: String, username: String, password: String) -> Self {
        Self {
            hotel_id,
            username,
            password,
            api_url: "https://supply-xml.booking.com/hotels/xml".to_string(),
        }
    }

    /// Create credentials with a custom API URL (for testing).
    pub fn with_url(hotel_id: String, username: String, password: String, api_url: String) -> Self {
        Self {
            hotel_id,
            username,
            password,
            api_url,
        }
    }
}

// ============================================
// Property Types
// ============================================

/// A Booking.com property (hotel).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingProperty {
    /// Booking.com hotel ID.
    pub hotel_id: String,
    /// Property name.
    pub name: String,
    /// Property description.
    pub description: Option<String>,
    /// Star rating (0-5).
    pub star_rating: Option<i32>,
    /// Property type.
    pub property_type: String,
    /// Address.
    pub address: BookingAddress,
    /// Contact information.
    pub contact: BookingContact,
    /// Room types.
    pub room_types: Vec<BookingRoomType>,
    /// Facilities/amenities.
    pub facilities: Vec<String>,
    /// Check-in time.
    pub check_in_time: Option<String>,
    /// Check-out time.
    pub check_out_time: Option<String>,
    /// Last sync timestamp.
    pub synced_at: Option<DateTime<Utc>>,
}

/// Property address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingAddress {
    /// Street address.
    pub street: String,
    /// City.
    pub city: String,
    /// State/Province.
    pub state: Option<String>,
    /// Postal code.
    pub postal_code: String,
    /// Country code (ISO 3166-1 alpha-2).
    pub country_code: String,
    /// Latitude.
    pub latitude: Option<f64>,
    /// Longitude.
    pub longitude: Option<f64>,
}

/// Property contact information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingContact {
    /// Contact email.
    pub email: Option<String>,
    /// Contact phone.
    pub phone: Option<String>,
    /// Website URL.
    pub website: Option<String>,
}

/// A room type at a property.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingRoomType {
    /// Room type ID.
    pub id: String,
    /// Room type name.
    pub name: String,
    /// Room description.
    pub description: Option<String>,
    /// Maximum occupancy.
    pub max_occupancy: i32,
    /// Number of beds.
    pub bed_count: i32,
    /// Bed type description.
    pub bed_type: Option<String>,
    /// Room size in square meters.
    pub room_size: Option<f32>,
    /// Room amenities.
    pub amenities: Vec<String>,
    /// Total rooms of this type.
    pub total_rooms: i32,
}

// ============================================
// Reservation Types
// ============================================

/// A Booking.com reservation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingReservation {
    /// Reservation ID.
    pub reservation_id: String,
    /// Booking.com reservation number.
    pub booking_number: String,
    /// Hotel ID.
    pub hotel_id: String,
    /// Room type ID.
    pub room_type_id: String,
    /// Guest information.
    pub guest: BookingGuest,
    /// Check-in date.
    pub check_in: NaiveDate,
    /// Check-out date.
    pub check_out: NaiveDate,
    /// Number of nights.
    pub nights: i32,
    /// Reservation status.
    pub status: BookingReservationStatus,
    /// Number of rooms booked.
    pub rooms: i32,
    /// Number of adults.
    pub adults: i32,
    /// Number of children.
    pub children: i32,
    /// Total price.
    pub total_price: Decimal,
    /// Commission amount.
    pub commission: Decimal,
    /// Currency code.
    pub currency: String,
    /// Rate plan code.
    pub rate_plan: Option<String>,
    /// Meal plan code.
    pub meal_plan: Option<String>,
    /// Special requests.
    pub special_requests: Option<String>,
    /// Booking timestamp.
    pub booked_at: DateTime<Utc>,
    /// Last modification timestamp.
    pub modified_at: DateTime<Utc>,
    /// Cancellation deadline.
    pub cancel_deadline: Option<DateTime<Utc>>,
    /// Is non-refundable.
    pub is_non_refundable: bool,
}

/// Booking.com reservation status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BookingReservationStatus {
    /// New reservation, pending confirmation.
    New,
    /// Confirmed reservation.
    Confirmed,
    /// Modified reservation.
    Modified,
    /// Cancelled reservation.
    Cancelled,
    /// No-show.
    NoShow,
}

/// Booking.com guest information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingGuest {
    /// Guest first name.
    pub first_name: String,
    /// Guest last name.
    pub last_name: String,
    /// Guest email.
    pub email: Option<String>,
    /// Guest phone.
    pub phone: Option<String>,
    /// Guest country.
    pub country: Option<String>,
    /// Guest address.
    pub address: Option<String>,
    /// Guest city.
    pub city: Option<String>,
    /// Guest postal code.
    pub postal_code: Option<String>,
    /// Special notes about guest.
    pub notes: Option<String>,
}

// ============================================
// OTA Message Types
// ============================================

/// OTA Read Request for fetching reservations.
#[derive(Debug, Clone)]
pub struct OtaReadRQ {
    /// Hotel code.
    pub hotel_code: String,
    /// Start date for reservation search.
    pub start_date: NaiveDate,
    /// End date for reservation search.
    pub end_date: NaiveDate,
    /// Reservation status filter.
    pub status_filter: Option<String>,
}

impl OtaReadRQ {
    /// Convert to OTA XML format.
    pub fn to_xml(&self) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<OTA_ReadRQ xmlns="http://www.opentravel.org/OTA/2003/05" Version="1.0">
  <ReadRequests>
    <HotelReadRequest HotelCode="{}">
      <SelectionCriteria Start="{}" End="{}" DateType="Arrival"/>
    </HotelReadRequest>
  </ReadRequests>
</OTA_ReadRQ>"#,
            self.hotel_code, self.start_date, self.end_date
        )
    }
}

/// OTA Read Response containing reservations.
#[derive(Debug, Clone)]
pub struct OtaReadRS {
    /// Whether the request was successful.
    pub success: bool,
    /// Error message if failed.
    pub error: Option<String>,
    /// Reservations returned.
    pub reservations: Vec<BookingReservation>,
}

impl OtaReadRS {
    /// Parse from OTA XML format.
    ///
    /// # Arguments
    /// * `xml` - The XML response body
    ///
    /// # Returns
    /// Parsed response or error.
    pub fn from_xml(_xml: &str) -> Result<Self, BookingError> {
        // Stub implementation - actual XML parsing would use quick-xml or similar
        tracing::info!("Parsing OTA_ReadRS XML");

        Ok(Self {
            success: true,
            error: None,
            reservations: Vec::new(),
        })
    }
}

/// OTA Hotel Reservation Notification Request (push from Booking.com).
#[derive(Debug, Clone)]
pub struct OtaHotelResNotifRQ {
    /// Reservations in the notification.
    pub reservations: Vec<OtaReservationNotification>,
}

/// A single reservation notification.
#[derive(Debug, Clone)]
pub struct OtaReservationNotification {
    /// Reservation ID.
    pub res_id: String,
    /// Reservation status (Commit, Cancel, Modify).
    pub res_status: String,
    /// Full reservation data.
    pub reservation: Option<BookingReservation>,
}

impl OtaHotelResNotifRQ {
    /// Parse from OTA XML format.
    pub fn from_xml(_xml: &str) -> Result<Self, BookingError> {
        // Stub implementation
        tracing::info!("Parsing OTA_HotelResNotifRQ XML");

        Ok(Self {
            reservations: Vec::new(),
        })
    }
}

/// OTA Hotel Reservation Notification Response.
#[derive(Debug, Clone)]
pub struct OtaHotelResNotifRS {
    /// Whether processing was successful.
    pub success: bool,
    /// Error message if failed.
    pub error: Option<String>,
}

impl OtaHotelResNotifRS {
    /// Create a success response.
    pub fn success() -> Self {
        Self {
            success: true,
            error: None,
        }
    }

    /// Create an error response.
    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            error: Some(message.to_string()),
        }
    }

    /// Convert to OTA XML format.
    pub fn to_xml(&self) -> String {
        if self.success {
            r#"<?xml version="1.0" encoding="UTF-8"?>
<OTA_HotelResNotifRS xmlns="http://www.opentravel.org/OTA/2003/05" Version="1.0">
  <Success/>
</OTA_HotelResNotifRS>"#
                .to_string()
        } else {
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<OTA_HotelResNotifRS xmlns="http://www.opentravel.org/OTA/2003/05" Version="1.0">
  <Errors>
    <Error Type="3" Code="450">{}</Error>
  </Errors>
</OTA_HotelResNotifRS>"#,
                self.error.as_deref().unwrap_or("Unknown error")
            )
        }
    }
}

// ============================================
// Availability Types
// ============================================

/// Availability update for a room type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityUpdate {
    /// Room type ID.
    pub room_type_id: String,
    /// Date for the update.
    pub date: NaiveDate,
    /// Number of available rooms.
    pub available_count: i32,
    /// Stop sell flag.
    pub stop_sell: bool,
    /// Closed to arrival.
    pub cta: bool,
    /// Closed to departure.
    pub ctd: bool,
    /// Minimum length of stay.
    pub min_los: Option<i32>,
    /// Maximum length of stay.
    pub max_los: Option<i32>,
}

/// OTA Hotel Availability Notification Request.
#[derive(Debug, Clone)]
pub struct OtaHotelAvailNotifRQ {
    /// Hotel code.
    pub hotel_code: String,
    /// Availability status messages.
    pub avail_status_messages: Vec<AvailStatusMessage>,
}

/// Availability status message.
#[derive(Debug, Clone)]
pub struct AvailStatusMessage {
    /// Room type code.
    pub room_type_code: String,
    /// Rate plan code.
    pub rate_plan_code: Option<String>,
    /// Start date.
    pub start_date: NaiveDate,
    /// End date.
    pub end_date: NaiveDate,
    /// Booking limit (available rooms).
    pub booking_limit: i32,
    /// Status (Open/Close).
    pub status: String,
    /// Length of stay restrictions.
    pub los_restrictions: Option<LosRestrictions>,
}

/// Length of stay restrictions.
#[derive(Debug, Clone)]
pub struct LosRestrictions {
    /// Minimum stay.
    pub min_los: Option<i32>,
    /// Maximum stay.
    pub max_los: Option<i32>,
}

impl OtaHotelAvailNotifRQ {
    /// Convert to OTA XML format.
    pub fn to_xml(&self) -> String {
        let mut messages = String::new();

        for msg in &self.avail_status_messages {
            messages.push_str(&format!(
                r#"    <AvailStatusMessage BookingLimit="{}" BookingLimitMessageType="SetLimit">
      <StatusApplicationControl Start="{}" End="{}" InvTypeCode="{}" RatePlanCode="{}"/>
      <RestrictionStatus Status="{}" Restriction="Master"/>
    </AvailStatusMessage>
"#,
                msg.booking_limit,
                msg.start_date,
                msg.end_date,
                msg.room_type_code,
                msg.rate_plan_code.as_deref().unwrap_or("STD"),
                msg.status
            ));
        }

        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<OTA_HotelAvailNotifRQ xmlns="http://www.opentravel.org/OTA/2003/05" Version="1.0">
  <AvailStatusMessages HotelCode="{}">
{}  </AvailStatusMessages>
</OTA_HotelAvailNotifRQ>"#,
            self.hotel_code, messages
        )
    }
}

/// OTA Hotel Availability Notification Response.
#[derive(Debug, Clone)]
pub struct OtaHotelAvailNotifRS {
    /// Whether the update was successful.
    pub success: bool,
    /// Error message if failed.
    pub error: Option<String>,
}

impl OtaHotelAvailNotifRS {
    /// Parse from OTA XML format.
    pub fn from_xml(_xml: &str) -> Result<Self, BookingError> {
        // Stub implementation
        tracing::info!("Parsing OTA_HotelAvailNotifRS XML");

        Ok(Self {
            success: true,
            error: None,
        })
    }
}

// ============================================
// Rate Types
// ============================================

/// Rate update for a room type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateUpdate {
    /// Room type ID.
    pub room_type_id: String,
    /// Rate plan code.
    pub rate_plan_code: String,
    /// Date for the rate.
    pub date: NaiveDate,
    /// Base rate amount.
    pub base_rate: Decimal,
    /// Currency code.
    pub currency: String,
    /// Extra person rate.
    pub extra_person_rate: Option<Decimal>,
    /// Extra child rate.
    pub extra_child_rate: Option<Decimal>,
}

// ============================================
// Property Mapping
// ============================================

/// Mapping between internal property and Booking.com property.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyMapping {
    /// Internal property ID.
    pub internal_property_id: Uuid,
    /// Booking.com hotel ID.
    pub external_property_id: String,
    /// External property name for reference.
    pub external_property_name: Option<String>,
    /// Room type mappings.
    pub room_mappings: Vec<RoomTypeMapping>,
    /// Whether sync is enabled.
    pub sync_enabled: bool,
    /// Last sync timestamp.
    pub last_sync_at: Option<DateTime<Utc>>,
}

/// Mapping between internal unit and Booking.com room type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomTypeMapping {
    /// Internal unit ID.
    pub internal_unit_id: Uuid,
    /// Booking.com room type ID.
    pub external_room_type_id: String,
    /// Room type name for reference.
    pub external_room_type_name: Option<String>,
}

// ============================================
// Client Implementation
// ============================================

/// Booking.com API client.
pub struct BookingClient {
    client: reqwest::Client,
    credentials: BookingCredentials,
}

impl BookingClient {
    /// Create a new Booking.com client.
    pub fn new(credentials: BookingCredentials) -> Self {
        Self {
            client: reqwest::Client::new(),
            credentials,
        }
    }

    /// Create a new client with simple API key (for basic usage).
    pub fn with_api_key(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            credentials: BookingCredentials {
                hotel_id: String::new(),
                username: api_key.clone(),
                password: api_key,
                api_url: "https://supply-xml.booking.com/hotels/xml".to_string(),
            },
        }
    }

    /// Generate the authorization header.
    fn generate_auth_header(&self) -> String {
        use base64::{engine::general_purpose::STANDARD, Engine};
        let credentials = format!(
            "{}:{}",
            self.credentials.username, self.credentials.password
        );
        format!("Basic {}", STANDARD.encode(credentials.as_bytes()))
    }

    // ==================== Property Operations ====================

    /// Fetch all properties for the account.
    ///
    /// # Returns
    /// List of properties.
    pub async fn fetch_properties(&self) -> Result<Vec<BookingProperty>, BookingError> {
        tracing::info!("Fetching Booking.com properties");

        // Stub implementation
        Ok(Vec::new())
    }

    /// Fetch a single property by ID.
    ///
    /// # Arguments
    /// * `hotel_id` - Booking.com hotel ID
    ///
    /// # Returns
    /// Property details.
    pub async fn fetch_property(&self, hotel_id: &str) -> Result<BookingProperty, BookingError> {
        tracing::info!("Fetching Booking.com property: {}", hotel_id);

        Err(BookingError::Api("Property not found".to_string()))
    }

    // ==================== Reservation Operations ====================

    /// Fetch reservations for a property.
    ///
    /// # Arguments
    /// * `hotel_id` - Booking.com hotel ID
    ///
    /// # Returns
    /// List of reservations.
    pub async fn fetch_reservations(
        &self,
        hotel_id: &str,
    ) -> Result<Vec<BookingReservation>, BookingError> {
        let request = OtaReadRQ {
            hotel_code: hotel_id.to_string(),
            start_date: Utc::now().date_naive(),
            end_date: Utc::now().date_naive() + Duration::days(365),
            status_filter: None,
        };

        let response = self
            .client
            .post(&self.credentials.api_url)
            .header("Authorization", self.generate_auth_header())
            .header("Content-Type", "application/xml")
            .body(request.to_xml())
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(BookingError::Api(error));
        }

        let body = response.text().await?;
        let parsed = OtaReadRS::from_xml(&body)?;

        if parsed.success {
            Ok(parsed.reservations)
        } else {
            Err(BookingError::Api(
                parsed.error.unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }

    /// Sync reservations from Booking.com.
    pub async fn sync_reservations(&self, _property_id: &str) -> Result<(), BookingError> {
        tracing::info!("Syncing Booking.com reservations");
        Ok(())
    }

    // ==================== Push Operations ====================

    /// Push availability updates to Booking.com.
    ///
    /// # Arguments
    /// * `mapping` - Property mapping
    /// * `updates` - List of availability updates
    ///
    /// # Returns
    /// Ok if successful.
    pub async fn push_availability(
        &self,
        mapping: &PropertyMapping,
        updates: Vec<AvailabilityUpdate>,
    ) -> Result<(), BookingError> {
        let request = OtaHotelAvailNotifRQ {
            hotel_code: mapping.external_property_id.clone(),
            avail_status_messages: updates
                .iter()
                .map(|u| AvailStatusMessage {
                    room_type_code: u.room_type_id.clone(),
                    rate_plan_code: None,
                    start_date: u.date,
                    end_date: u.date,
                    booking_limit: u.available_count,
                    status: if u.stop_sell || u.available_count == 0 {
                        "Close".to_string()
                    } else {
                        "Open".to_string()
                    },
                    los_restrictions: u.min_los.map(|min| LosRestrictions {
                        min_los: Some(min),
                        max_los: u.max_los,
                    }),
                })
                .collect(),
        };

        let response = self
            .client
            .post(&self.credentials.api_url)
            .header("Authorization", self.generate_auth_header())
            .header("Content-Type", "application/xml")
            .body(request.to_xml())
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(BookingError::Api(error));
        }

        let body = response.text().await?;
        let parsed = OtaHotelAvailNotifRS::from_xml(&body)?;

        if parsed.success {
            tracing::info!(
                "Pushed {} availability updates to Booking.com",
                updates.len()
            );
            Ok(())
        } else {
            Err(BookingError::PushFailed(
                parsed.error.unwrap_or_else(|| "Unknown error".to_string()),
            ))
        }
    }

    /// Push rate updates to Booking.com.
    ///
    /// # Arguments
    /// * `hotel_id` - Booking.com hotel ID
    /// * `updates` - List of rate updates
    ///
    /// # Returns
    /// Ok if successful.
    pub async fn push_rates(
        &self,
        _hotel_id: &str,
        _updates: Vec<RateUpdate>,
    ) -> Result<(), BookingError> {
        tracing::info!("Pushing rates to Booking.com");

        // Stub implementation
        Ok(())
    }

    // ==================== Webhook/Push Notification Handling ====================

    /// Handle an incoming push notification from Booking.com.
    ///
    /// # Arguments
    /// * `xml` - The raw XML body
    ///
    /// # Returns
    /// Parsed notification request.
    pub fn parse_push_notification(xml: &str) -> Result<OtaHotelResNotifRQ, BookingError> {
        OtaHotelResNotifRQ::from_xml(xml)
    }

    /// Generate a response for a push notification.
    ///
    /// # Arguments
    /// * `success` - Whether processing was successful
    /// * `error` - Error message if failed
    ///
    /// # Returns
    /// XML response string.
    pub fn generate_push_response(success: bool, error: Option<&str>) -> String {
        if success {
            OtaHotelResNotifRS::success().to_xml()
        } else {
            OtaHotelResNotifRS::error(error.unwrap_or("Unknown error")).to_xml()
        }
    }
}

// ============================================
// Mapping Functions
// ============================================

/// Map Booking.com reservation status to internal status string.
pub fn map_reservation_status(status: &BookingReservationStatus) -> String {
    match status {
        BookingReservationStatus::New => "pending".to_string(),
        BookingReservationStatus::Confirmed => "confirmed".to_string(),
        BookingReservationStatus::Modified => "modified".to_string(),
        BookingReservationStatus::Cancelled => "cancelled".to_string(),
        BookingReservationStatus::NoShow => "no_show".to_string(),
    }
}

// ============================================
// Tests
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ota_read_rq_xml() {
        let request = OtaReadRQ {
            hotel_code: "12345".to_string(),
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            status_filter: None,
        };

        let xml = request.to_xml();
        assert!(xml.contains("HotelCode=\"12345\""));
        assert!(xml.contains("Start=\"2024-01-01\""));
        assert!(xml.contains("End=\"2024-12-31\""));
    }

    #[test]
    fn test_ota_hotel_res_notif_rs_success() {
        let response = OtaHotelResNotifRS::success();
        let xml = response.to_xml();

        assert!(xml.contains("<Success/>"));
        assert!(!xml.contains("<Errors>"));
    }

    #[test]
    fn test_ota_hotel_res_notif_rs_error() {
        let response = OtaHotelResNotifRS::error("Test error");
        let xml = response.to_xml();

        assert!(xml.contains("<Errors>"));
        assert!(xml.contains("Test error"));
        assert!(!xml.contains("<Success/>"));
    }

    #[test]
    fn test_ota_hotel_avail_notif_rq_xml() {
        let request = OtaHotelAvailNotifRQ {
            hotel_code: "12345".to_string(),
            avail_status_messages: vec![AvailStatusMessage {
                room_type_code: "DBL".to_string(),
                rate_plan_code: Some("STD".to_string()),
                start_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
                end_date: NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
                booking_limit: 5,
                status: "Open".to_string(),
                los_restrictions: None,
            }],
        };

        let xml = request.to_xml();
        assert!(xml.contains("HotelCode=\"12345\""));
        assert!(xml.contains("BookingLimit=\"5\""));
        assert!(xml.contains("InvTypeCode=\"DBL\""));
        assert!(xml.contains("Status=\"Open\""));
    }

    #[test]
    fn test_map_reservation_status() {
        assert_eq!(
            map_reservation_status(&BookingReservationStatus::New),
            "pending"
        );
        assert_eq!(
            map_reservation_status(&BookingReservationStatus::Confirmed),
            "confirmed"
        );
        assert_eq!(
            map_reservation_status(&BookingReservationStatus::Cancelled),
            "cancelled"
        );
        assert_eq!(
            map_reservation_status(&BookingReservationStatus::NoShow),
            "no_show"
        );
    }

    #[test]
    fn test_credentials_new() {
        let creds =
            BookingCredentials::new("12345".to_string(), "user".to_string(), "pass".to_string());

        assert_eq!(creds.hotel_id, "12345");
        assert!(creds.api_url.contains("booking.com"));
    }

    #[test]
    fn test_reservation_status_serialization() {
        let status = BookingReservationStatus::Confirmed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"confirmed\"");

        let parsed: BookingReservationStatus = serde_json::from_str("\"cancelled\"").unwrap();
        assert_eq!(parsed, BookingReservationStatus::Cancelled);
    }
}
