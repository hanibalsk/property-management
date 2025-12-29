# Story 83.2: Booking.com OAuth and Sync

Status: pending

## Story

As a **property manager**,
I want to **connect my Booking.com account and sync reservations**,
So that **I can manage hotel and vacation rental bookings centrally**.

## Acceptance Criteria

1. **AC-1: Booking.com OAuth Connection**
   - Given I want to connect Booking.com
   - When I initiate the connection
   - Then I am redirected to Booking.com OAuth
   - And after approval I am redirected back
   - And my account is connected

2. **AC-2: Property Sync**
   - Given I have connected Booking.com
   - When sync is triggered
   - Then my Booking.com properties are imported
   - And room types are mapped to units
   - And sync status is visible

3. **AC-3: Reservation Import**
   - Given properties are synced
   - When reservations are fetched
   - Then all reservations are imported
   - And guest details are available
   - And pricing information is complete

4. **AC-4: Real-time Updates via Push**
   - Given I have configured push notifications
   - When a reservation is created or modified
   - Then the change is received via push
   - And the system updates immediately

5. **AC-5: Rate and Availability Push**
   - Given I manage rates in PPT
   - When I update rates or availability
   - Then changes can be pushed to Booking.com
   - And confirmation is received

## Tasks / Subtasks

- [ ] Task 1: Implement OAuth Flow (AC: 1)
  - [ ] 1.1 Update `/backend/crates/integrations/src/booking.rs`
  - [ ] 1.2 Implement Booking.com Connectivity API auth
  - [ ] 1.3 Create authorization flow
  - [ ] 1.4 Store encrypted credentials
  - [ ] 1.5 Implement credential refresh

- [ ] Task 2: Implement Property Sync (AC: 2)
  - [ ] 2.1 Create Booking.com API client
  - [ ] 2.2 Fetch hotel/property list
  - [ ] 2.3 Fetch room types and rates
  - [ ] 2.4 Map to internal property/unit model
  - [ ] 2.5 Store mapping relationships

- [ ] Task 3: Implement Reservation Sync (AC: 3)
  - [ ] 3.1 Fetch reservations via API
  - [ ] 3.2 Map to internal reservation model
  - [ ] 3.3 Handle modifications and cancellations
  - [ ] 3.4 Import guest information
  - [ ] 3.5 Track reservation source

- [ ] Task 4: Implement Push Notifications (AC: 4)
  - [ ] 4.1 Create push notification endpoint
  - [ ] 4.2 Handle OTA_HotelResNotifRQ messages
  - [ ] 4.3 Parse reservation notifications
  - [ ] 4.4 Update system in real-time
  - [ ] 4.5 Send acknowledgment responses

- [ ] Task 5: Implement Rate/Availability Push (AC: 5)
  - [ ] 5.1 Create outbound rate update function
  - [ ] 5.2 Format OTA_HotelAvailNotifRQ messages
  - [ ] 5.3 Push availability changes
  - [ ] 5.4 Handle push responses and errors
  - [ ] 5.5 Create retry mechanism

- [ ] Task 6: Create Frontend Integration UI (AC: 1, 2, 3, 5)
  - [ ] 6.1 Create Booking.com integration card
  - [ ] 6.2 Add connection wizard
  - [ ] 6.3 Show property mapping interface
  - [ ] 6.4 Display sync status
  - [ ] 6.5 Add rate push controls

## Dev Notes

### Architecture Requirements
- Booking.com Connectivity API integration
- OTA/XML message handling
- Push notification endpoint
- Bidirectional sync capability

### Technical Specifications
- API: Booking.com Connectivity API
- Message format: OTA XML
- Push endpoint: HTTPS with IP whitelisting
- Sync interval: 30 minutes (supplementary to push)

### Existing TODO Reference
```rust
// backend/crates/integrations/src/booking.rs:30
// TODO: Implement Booking.com OAuth flow
// - Connectivity API authentication
// - Property and reservation sync
// - Push notification handling
```

### Booking.com API Integration
```rust
impl BookingIntegration {
    pub async fn fetch_reservations(&self, hotel_id: &str) -> Result<Vec<BookingReservation>, IntegrationError> {
        let request = OtaReadRQ {
            hotel_code: hotel_id.to_string(),
            start_date: Utc::now().date_naive(),
            end_date: Utc::now().date_naive() + Duration::days(365),
        };

        let response = self.client
            .post(&self.api_url)
            .header("Authorization", self.generate_auth_header())
            .body(request.to_xml())
            .send()
            .await?;

        OtaReadRS::from_xml(&response.text().await?)
            .map(|rs| rs.reservations)
    }
}
```

### Push Notification Handler
```rust
async fn handle_booking_push(
    State(state): State<AppState>,
    body: String,
) -> Result<impl IntoResponse, ApiError> {
    let notification = OtaHotelResNotifRQ::from_xml(&body)?;

    for reservation in notification.reservations {
        match reservation.res_status {
            "Commit" => create_or_update_reservation(&state, reservation).await?,
            "Cancel" => cancel_reservation(&state, &reservation.res_id).await?,
            "Modify" => modify_reservation(&state, reservation).await?,
            _ => {}
        }
    }

    Ok(OtaHotelResNotifRS::success().to_xml())
}
```

### Property Mapping Model
```sql
CREATE TABLE integration_property_mappings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    connection_id UUID NOT NULL REFERENCES integration_connections(id),
    internal_property_id UUID NOT NULL REFERENCES properties(id),
    external_property_id VARCHAR(255) NOT NULL,
    external_property_name VARCHAR(255),
    sync_enabled BOOLEAN NOT NULL DEFAULT true,
    last_sync_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(connection_id, external_property_id)
);

CREATE TABLE integration_unit_mappings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    property_mapping_id UUID NOT NULL REFERENCES integration_property_mappings(id),
    internal_unit_id UUID NOT NULL REFERENCES units(id),
    external_room_type_id VARCHAR(255) NOT NULL,
    external_room_type_name VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(property_mapping_id, external_room_type_id)
);
```

### File List (to create/modify)

**Modify:**
- `/backend/crates/integrations/src/booking.rs` - Complete implementation
- `/backend/crates/integrations/src/lib.rs` - Export module

**Create:**
- `/backend/servers/api-server/src/routes/integrations/booking.rs` - API handlers
- `/backend/crates/integrations/src/ota/mod.rs` - OTA XML handling
- `/backend/crates/integrations/src/ota/messages.rs` - OTA message types
- `/backend/crates/db/migrations/NNNN_create_property_mappings.sql` - Schema

**Frontend:**
- `/frontend/apps/ppt-web/src/features/settings/integrations/BookingIntegration.tsx`
- `/frontend/apps/ppt-web/src/features/settings/integrations/PropertyMappingWizard.tsx`
- `/frontend/packages/api-client/src/integrations/booking.ts`

### Rate Push Example
```rust
pub async fn push_availability(
    &self,
    mapping: &PropertyMapping,
    updates: Vec<AvailabilityUpdate>,
) -> Result<(), IntegrationError> {
    let request = OtaHotelAvailNotifRQ {
        hotel_code: mapping.external_property_id.clone(),
        avail_status_messages: updates.iter().map(|u| AvailStatusMessage {
            room_type_code: u.room_type_id.clone(),
            start_date: u.date,
            end_date: u.date,
            booking_limit: u.available_count,
            status: if u.available_count > 0 { "Open" } else { "Close" },
        }).collect(),
    };

    let response = self.client
        .post(&self.api_url)
        .body(request.to_xml())
        .send()
        .await?;

    OtaHotelAvailNotifRS::from_xml(&response.text().await?)
        .and_then(|rs| if rs.success { Ok(()) } else { Err(IntegrationError::PushFailed) })
}
```

### Dependencies
- Story 83.1 (Airbnb Integration) - Shared integration infrastructure
- Epic 84 (Backend Infrastructure) - Encryption and token storage

### References
- [Source: backend/crates/integrations/src/booking.rs:30]
- [UC-44: External Platform Integrations]
- [Booking.com Connectivity API Documentation]
