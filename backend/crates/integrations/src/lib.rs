//! External integrations: Airbnb, Booking, real estate portals, calendar sync, accounting exports.

pub mod airbnb;
pub mod booking;
pub mod portals;

// Epic 61: External Integrations Suite
pub mod accounting;
pub mod calendar;
pub mod crypto;

// Epic 84: S3 Storage Integration
pub mod storage;

// Epic 64: Advanced AI & LLM Capabilities
pub mod llm;

// Re-exports

// Story 83.1: Airbnb Integration
pub use airbnb::{
    map_reservation_status as map_airbnb_status, map_to_internal_reservation, AirbnbClient,
    AirbnbError, AirbnbGuest, AirbnbListing, AirbnbOAuthConfig, AirbnbOAuthTokens, AirbnbPhoto,
    AirbnbReservation, AirbnbReservationStatus, AirbnbWebhookEvent, AirbnbWebhookEventType,
    ListingSyncResult, Reservation, ReservationSyncResult,
};

// Story 83.2: Booking.com Integration
pub use booking::{
    map_reservation_status as map_booking_status, AvailStatusMessage, AvailabilityUpdate,
    BookingAddress, BookingClient, BookingContact, BookingCredentials, BookingError, BookingGuest,
    BookingProperty, BookingReservation, BookingReservationStatus, BookingRoomType,
    LosRestrictions, OtaHotelAvailNotifRQ, OtaHotelAvailNotifRS, OtaHotelResNotifRQ,
    OtaHotelResNotifRS, OtaReadRQ, OtaReadRS, OtaReservationNotification, PropertyMapping,
    RateUpdate, RoomTypeMapping,
};

// Story 83.3: Portal Webhooks
pub use portals::{
    compute_hmac_sha256, get_parser, parse_webhook, verify_webhook_signature,
    BezrealitkyInquiryData, BezrealitkyParser, BezrealitkyWebhook, GenericParser, GenericWebhook,
    ImmoweltContact, ImmoweltParser, ImmoweltWebhook, InquiryStatus, ParseError, ParsedInquiry,
    PortalClient, PortalConnection, PortalError, PortalInquiry, PortalParser, PortalType,
    SrealityContact, SrealityParser, SrealityWebhook,
};

// Story 61.1: Calendar Integration
pub use calendar::{
    AttendeeStatus, CalendarError, CalendarListItem, EventAttendee, ExternalCalendarEvent,
    GoogleCalendarClient, MicrosoftCalendarClient, OAuthConfig, OAuthTokens, SyncResult,
};

// Story 61.2: Accounting System Export
pub use accounting::{
    AccountingError, ExportInvoice, ExportPayment, InvoiceItem, MoneyS3Exporter, Partner,
    PaymentType, PohodaExporter, ValidationResult, VatRate,
};

// Encryption utilities for sensitive integration data
pub use crypto::{
    decrypt_if_available, encrypt_if_available, CryptoError, IntegrationCrypto, ENCRYPTION_KEY_ENV,
};

// Story 64.1-64.4: LLM Integration
pub use llm::{
    ChatCompletionRequest, ChatCompletionResponse, ChatMessage, ContextChunk, EnhancedChatResult,
    LeaseGenerationInput, LeaseGenerationResult, ListingDescriptionInput, ListingDescriptionResult,
    LlmClient, LlmConfig, LlmError, TokenUsage,
};

// Story 84.1: S3 Presigned URLs
pub use storage::{
    generate_callback_token, generate_storage_key, get_content_type, is_allowed_content_type,
    supports_inline_preview, DownloadUrlResponse, PresignedUrl, StorageConfig, StorageError,
    StorageService, UploadUrlResponse, ALLOWED_MIME_TYPES, DEFAULT_DOWNLOAD_EXPIRATION_SECS,
    DEFAULT_UPLOAD_EXPIRATION_SECS, MAX_UPLOAD_SIZE_BYTES,
};
