namespace PropertyManagement.Shared;

@doc("Unique identifier (UUID v4)")
scalar uuid extends string;

@doc("ISO 8601 date-time string")
scalar dateTime extends string;

@doc("ISO 8601 date string (YYYY-MM-DD)")
scalar date extends string;

@doc("Email address")
@format("email")
scalar email extends string;

@doc("Phone number in E.164 format")
scalar phoneNumber extends string;

@doc("URL string")
@format("uri")
scalar url extends string;

@doc("Physical address")
model Address {
  @doc("Street address line 1")
  street1: string;

  @doc("Street address line 2 (optional)")
  street2?: string;

  @doc("City name")
  city: string;

  @doc("State or province")
  state?: string;

  @doc("Postal/ZIP code")
  postalCode: string;

  @doc("ISO 3166-1 alpha-2 country code")
  @minLength(2)
  @maxLength(2)
  country: string;
}

@doc("Monetary amount with currency")
model Money {
  @doc("Amount in smallest currency unit (e.g., cents)")
  amount: int64;

  @doc("ISO 4217 currency code")
  @minLength(3)
  @maxLength(3)
  currency: string;
}

@doc("GPS coordinates")
model GeoLocation {
  @doc("Latitude (-90 to 90)")
  @minValue(-90)
  @maxValue(90)
  latitude: float64;

  @doc("Longitude (-180 to 180)")
  @minValue(-180)
  @maxValue(180)
  longitude: float64;
}

@doc("File attachment metadata")
model Attachment {
  @doc("Unique identifier")
  id: uuid;

  @doc("Original filename")
  filename: string;

  @doc("MIME type")
  mimeType: string;

  @doc("File size in bytes")
  sizeBytes: int64;

  @doc("Download URL")
  url: url;

  @doc("Upload timestamp")
  uploadedAt: dateTime;
}

@doc("Base model for all entities with audit fields")
model AuditableEntity {
  @doc("Creation timestamp")
  createdAt: dateTime;

  @doc("Last update timestamp")
  updatedAt: dateTime;

  @doc("ID of user who created this entity")
  createdBy?: uuid;

  @doc("ID of user who last updated this entity")
  updatedBy?: uuid;
}

@doc("Localized content")
model LocalizedString {
  @doc("Default language value")
  default: string;

  @doc("Translations by ISO 639-1 language code")
  translations?: Record<string>;
}
