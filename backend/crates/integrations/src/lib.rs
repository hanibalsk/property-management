//! External integrations: Airbnb, Booking, real estate portals, calendar sync, accounting exports.

pub mod airbnb;
pub mod booking;
pub mod portals;

// Epic 61: External Integrations Suite
pub mod accounting;
pub mod calendar;
pub mod crypto;

// Epic 64: Advanced AI & LLM Capabilities
pub mod llm;

// Re-exports
pub use airbnb::AirbnbClient;
pub use booking::BookingClient;
pub use portals::PortalClient;

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
