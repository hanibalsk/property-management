//! External integrations: Airbnb, Booking, real estate portals, calendar sync, accounting exports.

pub mod airbnb;
pub mod booking;
pub mod portals;

// Epic 61: External Integrations Suite
pub mod accounting;
pub mod calendar;

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
