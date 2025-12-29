# Story 83.3: Real Estate Portal Webhooks

Status: pending

## Story

As a **property manager**,
I want to **receive inquiries from real estate portals automatically**,
So that **I can respond to potential buyers/renters quickly**.

## Acceptance Criteria

1. **AC-1: Webhook Registration**
   - Given I want to receive portal inquiries
   - When I configure a portal integration
   - Then a webhook URL is generated for that portal
   - And authentication credentials are provided
   - And the portal can send notifications

2. **AC-2: Inquiry Reception**
   - Given a webhook is configured
   - When a portal sends an inquiry notification
   - Then the inquiry is parsed and stored
   - And it appears in the inquiries list
   - And the property manager is notified

3. **AC-3: Multiple Portal Support**
   - Given I list on multiple portals
   - When configuring integrations
   - Then I can add webhooks for each portal
   - And inquiries are tagged with source
   - And all inquiries appear in unified view

4. **AC-4: Webhook Security**
   - Given a webhook endpoint exists
   - When a request is received
   - Then the signature is verified
   - And invalid requests are rejected
   - And attempts are logged

5. **AC-5: Inquiry Data Mapping**
   - Given an inquiry is received
   - When processing the webhook
   - Then all inquiry fields are mapped correctly
   - And the associated listing is identified
   - And contact details are extracted

## Tasks / Subtasks

- [ ] Task 1: Create Webhook Infrastructure (AC: 1, 4)
  - [ ] 1.1 Update `/backend/crates/integrations/src/portals.rs`
  - [ ] 1.2 Create webhook endpoint `/api/v1/webhooks/portal/{portal_id}`
  - [ ] 1.3 Generate unique webhook URLs per connection
  - [ ] 1.4 Implement signature verification
  - [ ] 1.5 Log all webhook attempts

- [ ] Task 2: Implement Inquiry Parser (AC: 2, 5)
  - [ ] 2.1 Create portal-specific parsers
  - [ ] 2.2 Parse Sreality.cz format
  - [ ] 2.3 Parse Bezrealitky.cz format
  - [ ] 2.4 Parse Immowelt.de format
  - [ ] 2.5 Map to unified inquiry model

- [ ] Task 3: Create Inquiry Storage (AC: 2, 3)
  - [ ] 3.1 Create portal_inquiries table
  - [ ] 3.2 Store raw webhook payload
  - [ ] 3.3 Store parsed inquiry data
  - [ ] 3.4 Link to property listing
  - [ ] 3.5 Track inquiry source

- [ ] Task 4: Implement Notification (AC: 2)
  - [ ] 4.1 Send push notification to mobile
  - [ ] 4.2 Send email notification
  - [ ] 4.3 Update unread inquiry count
  - [ ] 4.4 Create in-app notification

- [ ] Task 5: Create Portal Management UI (AC: 1, 3)
  - [ ] 5.1 Create portal integrations list
  - [ ] 5.2 Add portal connection wizard
  - [ ] 5.3 Display webhook URL and credentials
  - [ ] 5.4 Show connection status and test button
  - [ ] 5.5 Display inquiry statistics per portal

- [ ] Task 6: Create Inquiry List View (AC: 2, 3)
  - [ ] 6.1 Create unified inquiries page
  - [ ] 6.2 Filter by portal source
  - [ ] 6.3 Show inquiry details
  - [ ] 6.4 Quick reply functionality
  - [ ] 6.5 Mark as handled/archived

## Dev Notes

### Architecture Requirements
- Portal-agnostic webhook handling
- Extensible parser architecture for new portals
- Secure webhook verification
- Real-time notification delivery

### Technical Specifications
- Webhook URL format: `https://api.ppt.example.com/webhooks/portal/{connection_id}`
- Authentication: HMAC-SHA256 signature or Basic Auth (portal-dependent)
- Supported portals: Sreality.cz, Bezrealitky.cz, Immowelt.de
- Inquiry retention: 2 years

### Existing TODO Reference
```rust
// backend/crates/integrations/src/portals.rs:32
// TODO: Implement portal webhook handling
// - Webhook endpoint registration
// - Signature verification
// - Inquiry parsing and storage
```

### Webhook Handler
```rust
async fn handle_portal_webhook(
    State(state): State<AppState>,
    Path(connection_id): Path<Uuid>,
    headers: HeaderMap,
    body: String,
) -> Result<impl IntoResponse, ApiError> {
    // 1. Verify connection exists
    let connection = state.portal_repo.get_connection(connection_id).await?
        .ok_or(ApiError::NotFound)?;

    // 2. Verify signature
    verify_webhook_signature(&connection, &headers, &body)?;

    // 3. Parse inquiry based on portal type
    let inquiry = match connection.portal_type.as_str() {
        "sreality" => SrealityParser::parse(&body)?,
        "bezrealitky" => BezrealitkyParser::parse(&body)?,
        "immowelt" => ImmoweltParser::parse(&body)?,
        _ => return Err(ApiError::BadRequest("Unknown portal type".into())),
    };

    // 4. Store inquiry
    let stored = state.inquiry_repo.create(PortalInquiry {
        connection_id,
        portal_type: connection.portal_type.clone(),
        external_id: inquiry.external_id,
        property_external_id: inquiry.property_id,
        contact_name: inquiry.name,
        contact_email: inquiry.email,
        contact_phone: inquiry.phone,
        message: inquiry.message,
        raw_payload: body,
        received_at: Utc::now(),
        ..Default::default()
    }).await?;

    // 5. Send notification
    state.notification_service.send_inquiry_notification(&stored).await?;

    Ok(StatusCode::OK)
}
```

### Portal Parser Trait
```rust
trait PortalParser {
    fn parse(body: &str) -> Result<ParsedInquiry, ParseError>;
}

struct ParsedInquiry {
    external_id: String,
    property_id: String,
    name: String,
    email: String,
    phone: Option<String>,
    message: String,
}

struct SrealityParser;
impl PortalParser for SrealityParser {
    fn parse(body: &str) -> Result<ParsedInquiry, ParseError> {
        let data: SrealityWebhook = serde_json::from_str(body)?;
        Ok(ParsedInquiry {
            external_id: data.inquiry_id,
            property_id: data.estate_id.to_string(),
            name: data.contact.name,
            email: data.contact.email,
            phone: data.contact.phone,
            message: data.message,
        })
    }
}
```

### Database Schema
```sql
CREATE TABLE portal_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    portal_type VARCHAR(50) NOT NULL, -- 'sreality', 'bezrealitky', 'immowelt'
    name VARCHAR(255) NOT NULL,
    webhook_secret VARCHAR(255) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_webhook_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE portal_inquiries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    connection_id UUID NOT NULL REFERENCES portal_connections(id),
    portal_type VARCHAR(50) NOT NULL,
    external_id VARCHAR(255),
    property_id UUID REFERENCES properties(id),
    property_external_id VARCHAR(255),
    contact_name VARCHAR(255) NOT NULL,
    contact_email VARCHAR(255) NOT NULL,
    contact_phone VARCHAR(50),
    message TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'new', -- 'new', 'read', 'replied', 'archived'
    raw_payload JSONB NOT NULL,
    received_at TIMESTAMPTZ NOT NULL,
    read_at TIMESTAMPTZ,
    replied_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_portal_inquiries_connection ON portal_inquiries(connection_id);
CREATE INDEX idx_portal_inquiries_status ON portal_inquiries(status);
CREATE INDEX idx_portal_inquiries_property ON portal_inquiries(property_id);
```

### File List (to create/modify)

**Modify:**
- `/backend/crates/integrations/src/portals.rs` - Complete implementation

**Create:**
- `/backend/servers/api-server/src/routes/webhooks/portal.rs` - Webhook handler
- `/backend/servers/api-server/src/routes/webhooks/mod.rs` - Module
- `/backend/crates/integrations/src/parsers/mod.rs` - Parser module
- `/backend/crates/integrations/src/parsers/sreality.rs` - Sreality parser
- `/backend/crates/integrations/src/parsers/bezrealitky.rs` - Bezrealitky parser
- `/backend/crates/integrations/src/parsers/immowelt.rs` - Immowelt parser
- `/backend/crates/db/src/repositories/portal_inquiry.rs` - Repository
- `/backend/crates/db/migrations/NNNN_create_portal_webhooks.sql` - Schema

**Frontend:**
- `/frontend/apps/ppt-web/src/features/settings/integrations/PortalIntegrations.tsx`
- `/frontend/apps/ppt-web/src/features/inquiries/PortalInquiriesPage.tsx`
- `/frontend/packages/api-client/src/integrations/portals.ts`

### Signature Verification
```rust
fn verify_webhook_signature(
    connection: &PortalConnection,
    headers: &HeaderMap,
    body: &str,
) -> Result<(), ApiError> {
    let signature = headers
        .get("X-Webhook-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    let expected = compute_hmac_sha256(&connection.webhook_secret, body);

    if !constant_time_compare(signature, &expected) {
        return Err(ApiError::Unauthorized);
    }

    Ok(())
}
```

### Dependencies
- Story 83.1 (Airbnb Integration) - Shared integration infrastructure
- Epic 2B (Notifications) - Notification delivery

### References
- [Source: backend/crates/integrations/src/portals.rs:32]
- [UC-44: External Platform Integrations]
- [Sreality API Documentation]
