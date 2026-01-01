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
    pub fn from_xml(xml: &str) -> Result<Self, BookingError> {
        tracing::info!("Parsing OTA_ReadRS XML");

        // Check for errors first
        if xml.contains("<Errors>") || xml.contains("<Error") {
            let error_msg = Self::extract_error_message(xml);
            return Ok(Self {
                success: false,
                error: Some(error_msg),
                reservations: Vec::new(),
            });
        }

        // Check for success
        if !xml.contains("<Success") && !xml.contains("<ReservationsList") {
            return Ok(Self {
                success: false,
                error: Some("Unknown response format".to_string()),
                reservations: Vec::new(),
            });
        }

        // Parse reservations from XML
        let reservations = Self::parse_reservations(xml)?;

        Ok(Self {
            success: true,
            error: None,
            reservations,
        })
    }

    /// Extract error message from XML response.
    fn extract_error_message(xml: &str) -> String {
        // Try to find error message between <Error> tags or in ShortText attribute
        if let Some(start) = xml.find("ShortText=\"") {
            let start = start + 11;
            if let Some(end) = xml[start..].find('"') {
                return xml[start..start + end].to_string();
            }
        }
        if let Some(start) = xml.find("<Error") {
            if let Some(msg_start) = xml[start..].find('>') {
                let content_start = start + msg_start + 1;
                if let Some(end) = xml[content_start..].find("</Error>") {
                    let msg = xml[content_start..content_start + end].trim();
                    if !msg.is_empty() {
                        return msg.to_string();
                    }
                }
            }
        }
        "Unknown error from Booking.com".to_string()
    }

    /// Parse reservations from OTA XML.
    fn parse_reservations(xml: &str) -> Result<Vec<BookingReservation>, BookingError> {
        let mut reservations = Vec::new();

        // Find all HotelReservation elements
        let mut search_pos = 0;
        while let Some(start) = xml[search_pos..].find("<HotelReservation") {
            let abs_start = search_pos + start;
            if let Some(end) = xml[abs_start..].find("</HotelReservation>") {
                let res_xml = &xml[abs_start..abs_start + end + 19];
                if let Ok(reservation) = Self::parse_single_reservation(res_xml) {
                    reservations.push(reservation);
                }
                search_pos = abs_start + end + 19;
            } else {
                break;
            }
        }

        Ok(reservations)
    }

    /// Parse a single HotelReservation element.
    fn parse_single_reservation(xml: &str) -> Result<BookingReservation, BookingError> {
        // Extract reservation ID
        let res_id = Self::extract_attr(xml, "ResID_Value")
            .or_else(|| Self::extract_attr(xml, "ID"))
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        // Extract booking number
        let booking_number =
            Self::extract_attr(xml, "ResID_Value").unwrap_or_else(|| res_id.clone());

        // Extract hotel ID
        let hotel_id = Self::extract_attr(xml, "HotelCode").unwrap_or_default();

        // Extract room type ID
        let room_type_id = Self::extract_attr(xml, "InvTypeCode")
            .or_else(|| Self::extract_attr(xml, "RoomTypeCode"))
            .unwrap_or_default();

        // Extract dates
        let check_in = Self::extract_attr(xml, "Start")
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
            .unwrap_or_else(|| Utc::now().date_naive());

        let check_out = Self::extract_attr(xml, "End")
            .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
            .unwrap_or_else(|| check_in + Duration::days(1));

        let nights = (check_out - check_in).num_days() as i32;

        // Extract status
        let status_str =
            Self::extract_attr(xml, "ResStatus").unwrap_or_else(|| "Confirmed".to_string());
        let status = match status_str.to_lowercase().as_str() {
            "commit" | "confirmed" => BookingReservationStatus::Confirmed,
            "cancel" | "cancelled" => BookingReservationStatus::Cancelled,
            "modify" | "modified" => BookingReservationStatus::Modified,
            "noshow" | "no_show" => BookingReservationStatus::NoShow,
            _ => BookingReservationStatus::New,
        };

        // Extract guest info
        let guest = BookingGuest {
            first_name: Self::extract_element(xml, "GivenName")
                .unwrap_or_else(|| "Guest".to_string()),
            last_name: Self::extract_element(xml, "Surname").unwrap_or_default(),
            email: Self::extract_element(xml, "Email"),
            phone: Self::extract_element(xml, "PhoneNumber")
                .or_else(|| Self::extract_attr(xml, "PhoneNumber")),
            country: Self::extract_attr(xml, "CountryCode"),
            address: Self::extract_element(xml, "AddressLine"),
            city: Self::extract_element(xml, "CityName"),
            postal_code: Self::extract_element(xml, "PostalCode"),
            notes: Self::extract_element(xml, "SpecialRequests")
                .or_else(|| Self::extract_element(xml, "Text")),
        };

        // Extract counts
        let rooms: i32 = Self::extract_attr(xml, "NumberOfUnits")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let adults: i32 = Self::extract_attr(xml, "AdultCount")
            .or_else(|| {
                Self::extract_attr(xml, "AgeQualifyingCode")
                    .filter(|s| s == "10")
                    .and(Self::extract_attr(xml, "Count"))
            })
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let children: i32 = Self::extract_attr(xml, "ChildCount")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        // Extract pricing
        let total_price = Self::extract_attr(xml, "AmountAfterTax")
            .or_else(|| Self::extract_attr(xml, "Amount"))
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| Decimal::ZERO);
        let commission = Self::extract_attr(xml, "CommissionAmount")
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| Decimal::ZERO);
        let currency = Self::extract_attr(xml, "CurrencyCode").unwrap_or_else(|| "EUR".to_string());

        // Extract rate and meal plan
        let rate_plan = Self::extract_attr(xml, "RatePlanCode");
        let meal_plan = Self::extract_attr(xml, "MealPlanCode");

        // Extract special requests
        let special_requests = Self::extract_element(xml, "SpecialRequests")
            .or_else(|| Self::extract_element(xml, "Text"));

        // Extract timestamps
        let booked_at = Self::extract_attr(xml, "CreateDateTime")
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let modified_at = Self::extract_attr(xml, "LastModifyDateTime")
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(booked_at);

        // Extract cancellation deadline
        let cancel_deadline = Self::extract_attr(xml, "AbsoluteDeadline")
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Check if non-refundable
        let is_non_refundable =
            xml.contains("NonRefundable=\"true\"") || xml.contains("NonRefundable=\"1\"");

        Ok(BookingReservation {
            reservation_id: res_id,
            booking_number,
            hotel_id,
            room_type_id,
            guest,
            check_in,
            check_out,
            nights,
            status,
            rooms,
            adults,
            children,
            total_price,
            commission,
            currency,
            rate_plan,
            meal_plan,
            special_requests,
            booked_at,
            modified_at,
            cancel_deadline,
            is_non_refundable,
        })
    }

    /// Extract an attribute value from XML.
    fn extract_attr(xml: &str, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(start) = xml.find(&pattern) {
            let start = start + pattern.len();
            if let Some(end) = xml[start..].find('"') {
                return Some(xml[start..start + end].to_string());
            }
        }
        None
    }

    /// Extract element content from XML.
    fn extract_element(xml: &str, element_name: &str) -> Option<String> {
        let open_tag = format!("<{}", element_name);
        if let Some(tag_start) = xml.find(&open_tag) {
            // Find the end of the opening tag
            if let Some(content_start) = xml[tag_start..].find('>') {
                let content_start = tag_start + content_start + 1;
                let close_tag = format!("</{}>", element_name);
                if let Some(end) = xml[content_start..].find(&close_tag) {
                    let content = xml[content_start..content_start + end].trim();
                    if !content.is_empty() {
                        return Some(content.to_string());
                    }
                }
            }
        }
        None
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
    pub fn from_xml(xml: &str) -> Result<Self, BookingError> {
        tracing::info!("Parsing OTA_HotelResNotifRQ XML");

        let mut notifications = Vec::new();

        // Find all HotelReservation elements
        let mut search_pos = 0;
        while let Some(start) = xml[search_pos..].find("<HotelReservation") {
            let abs_start = search_pos + start;
            if let Some(end) = xml[abs_start..].find("</HotelReservation>") {
                let res_xml = &xml[abs_start..abs_start + end + 19];

                // Extract reservation status (Commit, Cancel, Modify)
                let res_status = Self::extract_attr(res_xml, "ResStatus")
                    .unwrap_or_else(|| "Commit".to_string());

                // Extract reservation ID
                let res_id = Self::extract_attr(res_xml, "ResID_Value")
                    .or_else(|| Self::extract_attr(res_xml, "ID"))
                    .unwrap_or_else(|| Uuid::new_v4().to_string());

                // Parse the full reservation if it's not a cancellation
                let reservation = if res_status.to_lowercase() != "cancel" {
                    OtaReadRS::parse_single_reservation(res_xml).ok()
                } else {
                    None
                };

                notifications.push(OtaReservationNotification {
                    res_id,
                    res_status,
                    reservation,
                });

                search_pos = abs_start + end + 19;
            } else {
                break;
            }
        }

        Ok(Self {
            reservations: notifications,
        })
    }

    /// Extract an attribute value from XML.
    fn extract_attr(xml: &str, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(start) = xml.find(&pattern) {
            let start = start + pattern.len();
            if let Some(end) = xml[start..].find('"') {
                return Some(xml[start..start + end].to_string());
            }
        }
        None
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
    pub fn from_xml(xml: &str) -> Result<Self, BookingError> {
        tracing::info!("Parsing OTA_HotelAvailNotifRS XML");

        // Check for errors
        if xml.contains("<Errors>") || xml.contains("<Error") {
            let error_msg = Self::extract_error_message(xml);
            return Ok(Self {
                success: false,
                error: Some(error_msg),
            });
        }

        // Check for success
        if xml.contains("<Success") || xml.contains("<Success/>") {
            return Ok(Self {
                success: true,
                error: None,
            });
        }

        // If no explicit success/error, assume success for now
        Ok(Self {
            success: true,
            error: None,
        })
    }

    /// Extract error message from XML response.
    fn extract_error_message(xml: &str) -> String {
        // Try to find error message in ShortText attribute
        if let Some(start) = xml.find("ShortText=\"") {
            let start = start + 11;
            if let Some(end) = xml[start..].find('"') {
                return xml[start..start + end].to_string();
            }
        }
        // Try to find error code
        if let Some(start) = xml.find("Code=\"") {
            let start = start + 6;
            if let Some(end) = xml[start..].find('"') {
                let code = &xml[start..start + end];
                return format!("Booking.com error code: {}", code);
            }
        }
        "Unknown error from Booking.com".to_string()
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
    /// Note: Booking.com doesn't have a dedicated property list API.
    /// Properties are typically configured via their extranet.
    /// This returns the configured property from credentials.
    ///
    /// # Returns
    /// List of properties (single property from credentials).
    pub async fn fetch_properties(&self) -> Result<Vec<BookingProperty>, BookingError> {
        tracing::info!("Fetching Booking.com properties");

        // If we have a hotel_id configured, return basic info for that property
        if !self.credentials.hotel_id.is_empty() {
            // Try to fetch the property details
            match self.fetch_property(&self.credentials.hotel_id).await {
                Ok(property) => Ok(vec![property]),
                Err(_) => {
                    // Return minimal property info if we can't fetch full details
                    Ok(vec![BookingProperty {
                        hotel_id: self.credentials.hotel_id.clone(),
                        name: format!("Property {}", self.credentials.hotel_id),
                        description: None,
                        star_rating: None,
                        property_type: "hotel".to_string(),
                        address: BookingAddress {
                            street: String::new(),
                            city: String::new(),
                            state: None,
                            postal_code: String::new(),
                            country_code: String::new(),
                            latitude: None,
                            longitude: None,
                        },
                        contact: BookingContact {
                            email: None,
                            phone: None,
                            website: None,
                        },
                        room_types: Vec::new(),
                        facilities: Vec::new(),
                        check_in_time: None,
                        check_out_time: None,
                        synced_at: Some(Utc::now()),
                    }])
                }
            }
        } else {
            Ok(Vec::new())
        }
    }

    /// Fetch a single property by ID.
    ///
    /// Uses OTA_HotelDescriptiveInfoRQ to get property details.
    ///
    /// # Arguments
    /// * `hotel_id` - Booking.com hotel ID
    ///
    /// # Returns
    /// Property details.
    pub async fn fetch_property(&self, hotel_id: &str) -> Result<BookingProperty, BookingError> {
        tracing::info!("Fetching Booking.com property: {}", hotel_id);

        // Build OTA_HotelDescriptiveInfoRQ request
        let request_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<OTA_HotelDescriptiveInfoRQ xmlns="http://www.opentravel.org/OTA/2003/05" Version="1.0">
  <HotelDescriptiveInfos>
    <HotelDescriptiveInfo HotelCode="{}" />
  </HotelDescriptiveInfos>
</OTA_HotelDescriptiveInfoRQ>"#,
            hotel_id
        );

        let response = self
            .client
            .post(&self.credentials.api_url)
            .header("Authorization", self.generate_auth_header())
            .header("Content-Type", "application/xml")
            .body(request_xml)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(BookingError::Api(error));
        }

        let body = response.text().await?;

        // Parse the response
        self.parse_property_response(&body, hotel_id)
    }

    /// Parse property details from OTA_HotelDescriptiveInfoRS.
    fn parse_property_response(
        &self,
        xml: &str,
        hotel_id: &str,
    ) -> Result<BookingProperty, BookingError> {
        // Check for errors
        if xml.contains("<Errors>") || xml.contains("<Error") {
            return Err(BookingError::Api(
                "Failed to fetch property details".to_string(),
            ));
        }

        // Extract property name
        let name = Self::extract_xml_attr(xml, "HotelName")
            .or_else(|| Self::extract_xml_element(xml, "HotelName"))
            .unwrap_or_else(|| format!("Property {}", hotel_id));

        // Extract description
        let description = Self::extract_xml_element(xml, "DescriptiveText");

        // Extract star rating
        let star_rating = Self::extract_xml_attr(xml, "Rating")
            .or_else(|| Self::extract_xml_attr(xml, "StarRating"))
            .and_then(|s| s.parse().ok());

        // Extract property type
        let property_type = Self::extract_xml_attr(xml, "PropertyType")
            .or_else(|| Self::extract_xml_attr(xml, "HotelCategory"))
            .unwrap_or_else(|| "hotel".to_string());

        // Extract address
        let address = BookingAddress {
            street: Self::extract_xml_element(xml, "AddressLine").unwrap_or_default(),
            city: Self::extract_xml_element(xml, "CityName").unwrap_or_default(),
            state: Self::extract_xml_element(xml, "StateProv"),
            postal_code: Self::extract_xml_element(xml, "PostalCode").unwrap_or_default(),
            country_code: Self::extract_xml_attr(xml, "CountryCode").unwrap_or_default(),
            latitude: Self::extract_xml_attr(xml, "Latitude").and_then(|s| s.parse().ok()),
            longitude: Self::extract_xml_attr(xml, "Longitude").and_then(|s| s.parse().ok()),
        };

        // Extract contact
        let contact = BookingContact {
            email: Self::extract_xml_element(xml, "Email"),
            phone: Self::extract_xml_attr(xml, "PhoneNumber")
                .or_else(|| Self::extract_xml_element(xml, "PhoneNumber")),
            website: Self::extract_xml_element(xml, "URL"),
        };

        // Extract room types
        let room_types = self.parse_room_types(xml);

        // Extract facilities
        let facilities = self.parse_facilities(xml);

        // Extract check-in/check-out times
        let check_in_time = Self::extract_xml_attr(xml, "CheckInTime");
        let check_out_time = Self::extract_xml_attr(xml, "CheckOutTime");

        Ok(BookingProperty {
            hotel_id: hotel_id.to_string(),
            name,
            description,
            star_rating,
            property_type,
            address,
            contact,
            room_types,
            facilities,
            check_in_time,
            check_out_time,
            synced_at: Some(Utc::now()),
        })
    }

    /// Parse room types from XML.
    fn parse_room_types(&self, xml: &str) -> Vec<BookingRoomType> {
        let mut room_types = Vec::new();

        let mut search_pos = 0;
        while let Some(start) = xml[search_pos..].find("<GuestRoom") {
            let abs_start = search_pos + start;
            if let Some(end) = xml[abs_start..].find("</GuestRoom>") {
                let room_xml = &xml[abs_start..abs_start + end + 12];

                let id = Self::extract_xml_attr(room_xml, "RoomTypeCode")
                    .or_else(|| Self::extract_xml_attr(room_xml, "InvTypeCode"))
                    .unwrap_or_else(|| Uuid::new_v4().to_string());

                let name = Self::extract_xml_element(room_xml, "RoomTypeName")
                    .or_else(|| Self::extract_xml_attr(room_xml, "RoomTypeName"))
                    .unwrap_or_else(|| "Room".to_string());

                let description = Self::extract_xml_element(room_xml, "DescriptiveText");

                let max_occupancy = Self::extract_xml_attr(room_xml, "MaxOccupancy")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(2);

                let bed_count = Self::extract_xml_attr(room_xml, "NumberOfBeds")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);

                let bed_type = Self::extract_xml_attr(room_xml, "BedType");

                let room_size =
                    Self::extract_xml_attr(room_xml, "RoomSize").and_then(|s| s.parse().ok());

                let total_rooms = Self::extract_xml_attr(room_xml, "Quantity")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);

                room_types.push(BookingRoomType {
                    id,
                    name,
                    description,
                    max_occupancy,
                    bed_count,
                    bed_type,
                    room_size,
                    amenities: Vec::new(),
                    total_rooms,
                });

                search_pos = abs_start + end + 12;
            } else {
                break;
            }
        }

        room_types
    }

    /// Parse facilities from XML.
    fn parse_facilities(&self, xml: &str) -> Vec<String> {
        let mut facilities = Vec::new();

        let mut search_pos = 0;
        while let Some(start) = xml[search_pos..].find("<Service") {
            let abs_start = search_pos + start;
            if let Some(end) = xml[abs_start..].find("/>") {
                let service_xml = &xml[abs_start..abs_start + end + 2];

                if let Some(code) = Self::extract_xml_attr(service_xml, "Code") {
                    // Map common Booking.com facility codes to names
                    let name = match code.as_str() {
                        "1" => "24-hour front desk",
                        "2" => "Bar",
                        "3" => "Restaurant",
                        "4" => "Room service",
                        "5" => "Parking",
                        "6" => "Free WiFi",
                        "7" => "Pool",
                        "8" => "Spa",
                        "9" => "Gym",
                        "10" => "Airport shuttle",
                        _ => &code,
                    };
                    facilities.push(name.to_string());
                }

                search_pos = abs_start + end + 2;
            } else {
                break;
            }
        }

        facilities
    }

    /// Extract an XML attribute value (static method for property parsing).
    fn extract_xml_attr(xml: &str, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(start) = xml.find(&pattern) {
            let start = start + pattern.len();
            if let Some(end) = xml[start..].find('"') {
                return Some(xml[start..start + end].to_string());
            }
        }
        None
    }

    /// Extract XML element content (static method for property parsing).
    fn extract_xml_element(xml: &str, element_name: &str) -> Option<String> {
        let open_tag = format!("<{}", element_name);
        if let Some(tag_start) = xml.find(&open_tag) {
            if let Some(content_start) = xml[tag_start..].find('>') {
                let content_start = tag_start + content_start + 1;
                let close_tag = format!("</{}>", element_name);
                if let Some(end) = xml[content_start..].find(&close_tag) {
                    let content = xml[content_start..content_start + end].trim();
                    if !content.is_empty() {
                        return Some(content.to_string());
                    }
                }
            }
        }
        None
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
    ///
    /// Fetches all reservations for the property and returns them.
    ///
    /// # Arguments
    /// * `hotel_id` - Booking.com hotel ID
    ///
    /// # Returns
    /// List of reservations synced.
    pub async fn sync_reservations(
        &self,
        hotel_id: &str,
    ) -> Result<Vec<BookingReservation>, BookingError> {
        tracing::info!("Syncing Booking.com reservations for hotel: {}", hotel_id);

        let reservations = self.fetch_reservations(hotel_id).await?;

        tracing::info!(
            "Synced {} reservations from Booking.com",
            reservations.len()
        );

        Ok(reservations)
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
    /// Uses OTA_HotelRateAmountNotifRQ to update rates.
    ///
    /// # Arguments
    /// * `hotel_id` - Booking.com hotel ID
    /// * `updates` - List of rate updates
    ///
    /// # Returns
    /// Ok if successful.
    pub async fn push_rates(
        &self,
        hotel_id: &str,
        updates: Vec<RateUpdate>,
    ) -> Result<(), BookingError> {
        if updates.is_empty() {
            return Ok(());
        }

        tracing::info!(
            "Pushing {} rate updates to Booking.com for hotel {}",
            updates.len(),
            hotel_id
        );

        // Build OTA_HotelRateAmountNotifRQ
        let mut rate_plans = String::new();
        for update in &updates {
            rate_plans.push_str(&format!(
                r#"    <RateAmountMessage>
      <StatusApplicationControl Start="{}" End="{}" InvTypeCode="{}" RatePlanCode="{}"/>
      <Rates>
        <Rate>
          <BaseByGuestAmts>
            <BaseByGuestAmt AmountAfterTax="{}" CurrencyCode="{}"/>
          </BaseByGuestAmts>
        </Rate>
      </Rates>
    </RateAmountMessage>
"#,
                update.date,
                update.date,
                update.room_type_id,
                update.rate_plan_code,
                update.base_rate,
                update.currency
            ));
        }

        let request_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<OTA_HotelRateAmountNotifRQ xmlns="http://www.opentravel.org/OTA/2003/05" Version="1.0">
  <RateAmountMessages HotelCode="{}">
{}  </RateAmountMessages>
</OTA_HotelRateAmountNotifRQ>"#,
            hotel_id, rate_plans
        );

        let response = self
            .client
            .post(&self.credentials.api_url)
            .header("Authorization", self.generate_auth_header())
            .header("Content-Type", "application/xml")
            .body(request_xml)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(BookingError::Api(error));
        }

        let body = response.text().await?;

        // Check for errors in response
        if body.contains("<Errors>") || body.contains("<Error") {
            let error_msg = if let Some(start) = body.find("ShortText=\"") {
                let start = start + 11;
                if let Some(end) = body[start..].find('"') {
                    body[start..start + end].to_string()
                } else {
                    "Unknown error".to_string()
                }
            } else {
                "Failed to push rates".to_string()
            };
            return Err(BookingError::PushFailed(error_msg));
        }

        tracing::info!("Successfully pushed {} rates to Booking.com", updates.len());
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
