# Story 83.1: Airbnb OAuth and Sync

Status: pending

## Story

As a **property manager**,
I want to **connect my Airbnb account and sync reservations**,
So that **I can manage short-term rentals from a single platform**.

## Acceptance Criteria

1. **AC-1: Airbnb OAuth Connection**
   - Given I want to connect Airbnb
   - When I initiate the connection
   - Then I am redirected to Airbnb OAuth
   - And after approval I am redirected back
   - And my account is connected

2. **AC-2: Property Listing Sync**
   - Given I have connected Airbnb
   - When sync is triggered
   - Then my Airbnb listings are imported
   - And they are mapped to properties in the system
   - And sync status is visible

3. **AC-3: Reservation Import**
   - Given listings are synced
   - When reservations are fetched
   - Then past and upcoming reservations are imported
   - And guest information is available
   - And booking details are complete

4. **AC-4: Availability Sync**
   - Given reservations are imported
   - When a new reservation is made on Airbnb
   - Then it is synced within the polling interval
   - And the property calendar is updated
   - And conflicts are flagged

5. **AC-5: Connection Management**
   - Given I have a connected Airbnb account
   - When I view connection settings
   - Then I can see connection status and last sync
   - And I can manually trigger sync
   - And I can disconnect the account

## Tasks / Subtasks

- [ ] Task 1: Implement OAuth Flow (AC: 1)
  - [ ] 1.1 Update `/backend/crates/integrations/src/airbnb.rs`
  - [ ] 1.2 Create OAuth authorization URL generator
  - [ ] 1.3 Implement token exchange endpoint
  - [ ] 1.4 Store encrypted OAuth tokens
  - [ ] 1.5 Handle token refresh

- [ ] Task 2: Implement Listing Sync (AC: 2)
  - [ ] 2.1 Create Airbnb API client
  - [ ] 2.2 Fetch listings from Airbnb
  - [ ] 2.3 Map Airbnb listing to internal property model
  - [ ] 2.4 Handle listing updates and deletions
  - [ ] 2.5 Store sync metadata (last sync, error state)

- [ ] Task 3: Implement Reservation Sync (AC: 3, 4)
  - [ ] 3.1 Fetch reservations from Airbnb
  - [ ] 3.2 Map to internal reservation model
  - [ ] 3.3 Import guest information
  - [ ] 3.4 Handle reservation updates and cancellations
  - [ ] 3.5 Create background sync job

- [ ] Task 4: Create Webhook Handler (AC: 4)
  - [ ] 4.1 Create webhook endpoint for Airbnb notifications
  - [ ] 4.2 Handle reservation created events
  - [ ] 4.3 Handle reservation updated events
  - [ ] 4.4 Handle reservation cancelled events
  - [ ] 4.5 Verify webhook signatures

- [ ] Task 5: Create Connection Management API (AC: 5)
  - [ ] 5.1 Create GET `/api/v1/integrations/airbnb/status`
  - [ ] 5.2 Create POST `/api/v1/integrations/airbnb/connect`
  - [ ] 5.3 Create POST `/api/v1/integrations/airbnb/sync`
  - [ ] 5.4 Create DELETE `/api/v1/integrations/airbnb`
  - [ ] 5.5 Handle connection errors gracefully

- [ ] Task 6: Create Frontend Integration UI (AC: 1, 5)
  - [ ] 6.1 Create Airbnb integration card in settings
  - [ ] 6.2 Add connect button with OAuth redirect
  - [ ] 6.3 Show sync status and last sync time
  - [ ] 6.4 Add manual sync trigger
  - [ ] 6.5 Add disconnect confirmation

## Dev Notes

### Architecture Requirements
- OAuth 2.0 with PKCE for security
- Encrypted token storage
- Background sync with configurable interval
- Webhook support for real-time updates

### Technical Specifications
- Airbnb API: Partner API v1
- OAuth: Authorization code flow with PKCE
- Sync interval: 15 minutes (configurable)
- Token encryption: AES-256-GCM

### Existing TODO Reference
```rust
// backend/crates/integrations/src/airbnb.rs:30
// TODO: Implement Airbnb OAuth flow
// - Authorization URL generation
// - Token exchange
// - Token storage and refresh
```

### OAuth Flow
```rust
impl AirbnbIntegration {
    pub fn generate_auth_url(&self, state: &str) -> String {
        format!(
            "https://www.airbnb.com/oauth2/auth?client_id={}&redirect_uri={}&response_type=code&state={}&scope={}",
            self.client_id,
            urlencoding::encode(&self.redirect_uri),
            state,
            "listings:read reservations:read"
        )
    }

    pub async fn exchange_code(&self, code: &str) -> Result<OAuthTokens, IntegrationError> {
        let response = self.client
            .post("https://api.airbnb.com/v1/oauth2/token")
            .form(&[
                ("grant_type", "authorization_code"),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("code", code),
                ("redirect_uri", &self.redirect_uri),
            ])
            .send()
            .await?;

        // Parse and return tokens
    }
}
```

### Reservation Model Mapping
```rust
struct AirbnbReservation {
    confirmation_code: String,
    listing_id: String,
    guest: AirbnbGuest,
    check_in: NaiveDate,
    check_out: NaiveDate,
    status: String,
    total_price: Decimal,
    // ...
}

fn map_to_internal(airbnb: AirbnbReservation, property_id: Uuid) -> Reservation {
    Reservation {
        id: Uuid::new_v4(),
        property_id,
        external_id: Some(airbnb.confirmation_code),
        external_source: Some("airbnb".to_string()),
        guest_name: airbnb.guest.full_name,
        check_in: airbnb.check_in,
        check_out: airbnb.check_out,
        status: map_status(airbnb.status),
        // ...
    }
}
```

### File List (to create/modify)

**Modify:**
- `/backend/crates/integrations/src/airbnb.rs` - Complete implementation
- `/backend/crates/integrations/src/lib.rs` - Export module

**Create:**
- `/backend/servers/api-server/src/routes/integrations/airbnb.rs` - API handlers
- `/backend/servers/api-server/src/routes/integrations/mod.rs` - Module
- `/backend/crates/db/src/repositories/integration.rs` - Token storage
- `/backend/crates/db/migrations/NNNN_create_integrations.sql` - Schema

**Frontend:**
- `/frontend/apps/ppt-web/src/features/settings/integrations/AirbnbIntegration.tsx`
- `/frontend/packages/api-client/src/integrations/airbnb.ts`

### Database Schema
```sql
CREATE TABLE integration_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id),
    provider VARCHAR(50) NOT NULL, -- 'airbnb', 'booking', etc.
    status VARCHAR(20) NOT NULL DEFAULT 'connected',
    access_token_encrypted BYTEA NOT NULL,
    refresh_token_encrypted BYTEA,
    token_expires_at TIMESTAMPTZ,
    external_account_id VARCHAR(255),
    last_sync_at TIMESTAMPTZ,
    sync_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(organization_id, provider)
);

CREATE INDEX idx_integration_connections_org ON integration_connections(organization_id);
```

### Dependencies
- Epic 84 (Backend Infrastructure) - Token encryption utilities
- Epic 1 (Authentication) - Tenant context

### References
- [Source: backend/crates/integrations/src/airbnb.rs:30]
- [UC-44: External Platform Integrations]
- [Airbnb Partner API Documentation]
