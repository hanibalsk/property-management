# Technical Design

This document provides the complete technical design for the Property Management System (PPT) and Reality Portal, including API endpoints, DTOs, validation rules, and state machines.

## Table of Contents

1. [Common Patterns](#common-patterns)
2. [API Endpoints](#api-endpoints)
3. [DTOs (Data Transfer Objects)](#dtos-data-transfer-objects)
4. [Validation Rules](#validation-rules)
5. [State Machines](#state-machines)

---

## Common Patterns

### Base URL

| Server | Base URL | Purpose |
|--------|----------|---------|
| api-server | `/api/v1` | Property Management |
| reality-server | `/api/v1` | Reality Portal |

### Request Headers

```http
Authorization: Bearer <jwt_access_token>
X-Tenant-Id: <organization_uuid>
X-Request-Id: <uuid>
Accept-Language: sk-SK | en-US | cs-CZ | de-DE
Content-Type: application/json
```

### Response Format

**Success Response:**
```json
{
  "data": { ... },
  "meta": {
    "requestId": "550e8400-e29b-41d4-a716-446655440000",
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

**Paginated Response:**
```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 150,
    "totalPages": 8,
    "hasMore": true
  },
  "meta": { ... }
}
```

**Error Response:**
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Validation failed",
    "details": [
      {
        "field": "email",
        "code": "INVALID_FORMAT",
        "message": "Invalid email format"
      }
    ],
    "requestId": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

### HTTP Status Codes

| Status | Meaning | Usage |
|--------|---------|-------|
| 200 | OK | Successful GET, PUT, PATCH |
| 201 | Created | Successful POST |
| 204 | No Content | Successful DELETE |
| 400 | Bad Request | Validation error |
| 401 | Unauthorized | Missing/invalid token |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Duplicate resource |
| 422 | Unprocessable Entity | Business rule violation |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |

### Rate Limiting

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1705312200
```

---

## API Endpoints

### 1. Authentication (auth)

#### POST /api/v1/auth/register

Register a new user account.

**Authorization:** None (public with invitation code)

**Request Body:** `RegisterRequest`

**Response:** `201 Created` → `AuthResponse`

**Errors:**
- `400` - Validation error
- `409` - Email already exists
- `422` - Invalid/expired invitation code

---

#### POST /api/v1/auth/login

Authenticate user with email and password.

**Authorization:** None (public)

**Request Body:** `LoginRequest`

**Response:** `200 OK` → `AuthResponse` or `MfaRequiredResponse`

**Errors:**
- `400` - Validation error
- `401` - Invalid credentials
- `423` - Account locked

---

#### POST /api/v1/auth/logout

Logout current session.

**Authorization:** Bearer token

**Request Body:** None

**Response:** `204 No Content`

---

#### POST /api/v1/auth/refresh

Refresh access token.

**Authorization:** None (uses refresh token)

**Request Body:** `RefreshTokenRequest`

**Response:** `200 OK` → `TokenResponse`

**Errors:**
- `401` - Invalid/expired refresh token

---

#### POST /api/v1/auth/mfa/verify

Verify MFA code during login.

**Authorization:** MFA token

**Request Body:** `MfaVerifyRequest`

**Response:** `200 OK` → `AuthResponse`

**Errors:**
- `401` - Invalid MFA code
- `429` - Too many attempts

---

#### POST /api/v1/auth/mfa/setup

Setup MFA for user account.

**Authorization:** Bearer token

**Request Body:** `MfaSetupRequest`

**Response:** `200 OK` → `MfaSetupResponse`

---

#### DELETE /api/v1/auth/mfa

Disable MFA for user account.

**Authorization:** Bearer token

**Request Body:** `MfaDisableRequest`

**Response:** `204 No Content`

---

#### POST /api/v1/auth/password/forgot

Request password reset email.

**Authorization:** None (public)

**Request Body:** `ForgotPasswordRequest`

**Response:** `200 OK` (always, for security)

---

#### POST /api/v1/auth/password/reset

Reset password with token.

**Authorization:** None (uses reset token)

**Request Body:** `ResetPasswordRequest`

**Response:** `200 OK`

**Errors:**
- `400` - Validation error
- `422` - Invalid/expired token

---

#### PUT /api/v1/auth/password

Change current password.

**Authorization:** Bearer token

**Request Body:** `ChangePasswordRequest`

**Response:** `200 OK`

**Errors:**
- `400` - Validation error
- `401` - Current password incorrect

---

#### GET /api/v1/auth/oauth/{provider}

Initiate OAuth flow.

**Authorization:** None (public)

**Path Parameters:**
- `provider`: `google` | `apple` | `facebook`

**Response:** `302 Redirect` to OAuth provider

---

#### GET /api/v1/auth/oauth/{provider}/callback

OAuth callback handler.

**Authorization:** None (public)

**Query Parameters:**
- `code`: Authorization code
- `state`: CSRF state

**Response:** `302 Redirect` to app with tokens

---

### 2. Organizations

#### GET /api/v1/organizations

List organizations (Super Admin only).

**Authorization:** Super Admin

**Query Parameters:** `PaginationParams`

**Response:** `200 OK` → `PaginatedResponse<OrganizationResponse>`

---

#### POST /api/v1/organizations

Create new organization.

**Authorization:** Super Admin

**Request Body:** `CreateOrganizationRequest`

**Response:** `201 Created` → `OrganizationResponse`

---

#### GET /api/v1/organizations/{id}

Get organization details.

**Authorization:** Organization Admin, Super Admin

**Response:** `200 OK` → `OrganizationResponse`

---

#### PUT /api/v1/organizations/{id}

Update organization.

**Authorization:** Organization Admin, Super Admin

**Request Body:** `UpdateOrganizationRequest`

**Response:** `200 OK` → `OrganizationResponse`

---

#### DELETE /api/v1/organizations/{id}

Delete organization.

**Authorization:** Super Admin

**Response:** `204 No Content`

---

#### PUT /api/v1/organizations/{id}/branding

Update organization branding.

**Authorization:** Organization Admin

**Request Body:** `OrganizationBrandingRequest`

**Response:** `200 OK` → `OrganizationResponse`

---

#### GET /api/v1/organizations/{id}/statistics

Get organization statistics.

**Authorization:** Organization Admin, Manager

**Response:** `200 OK` → `OrganizationStatisticsResponse`

---

### 3. Buildings

#### GET /api/v1/buildings

List buildings in organization.

**Authorization:** Owner, Tenant, Manager

**Query Parameters:** `PaginationParams`, `BuildingFilterParams`

**Response:** `200 OK` → `PaginatedResponse<BuildingListItem>`

---

#### POST /api/v1/buildings

Create new building.

**Authorization:** Manager, System Admin

**Request Body:** `CreateBuildingRequest`

**Response:** `201 Created` → `BuildingResponse`

---

#### GET /api/v1/buildings/{id}

Get building details.

**Authorization:** Owner, Tenant, Manager (in building)

**Response:** `200 OK` → `BuildingResponse`

---

#### PUT /api/v1/buildings/{id}

Update building.

**Authorization:** Manager, System Admin

**Request Body:** `UpdateBuildingRequest`

**Response:** `200 OK` → `BuildingResponse`

---

#### DELETE /api/v1/buildings/{id}

Archive building.

**Authorization:** System Admin

**Response:** `204 No Content`

---

#### GET /api/v1/buildings/{id}/statistics

Get building statistics.

**Authorization:** Manager

**Response:** `200 OK` → `BuildingStatisticsResponse`

---

#### GET /api/v1/buildings/{id}/units

List units in building.

**Authorization:** Owner, Tenant, Manager (in building)

**Query Parameters:** `PaginationParams`, `UnitFilterParams`

**Response:** `200 OK` → `PaginatedResponse<UnitListItem>`

---

#### POST /api/v1/buildings/{id}/units

Create new unit.

**Authorization:** Manager, System Admin

**Request Body:** `CreateUnitRequest`

**Response:** `201 Created` → `UnitResponse`

---

#### GET /api/v1/buildings/{buildingId}/units/{unitId}

Get unit details.

**Authorization:** Owner, Tenant (of unit), Manager

**Response:** `200 OK` → `UnitResponse`

---

#### PUT /api/v1/buildings/{buildingId}/units/{unitId}

Update unit.

**Authorization:** Manager, System Admin

**Request Body:** `UpdateUnitRequest`

**Response:** `200 OK` → `UnitResponse`

---

#### POST /api/v1/buildings/{buildingId}/units/{unitId}/owners

Assign owner to unit.

**Authorization:** Manager, System Admin

**Request Body:** `AssignOwnerRequest`

**Response:** `201 Created` → `UnitOwnershipResponse`

---

#### DELETE /api/v1/buildings/{buildingId}/units/{unitId}/owners/{ownerId}

Remove owner from unit.

**Authorization:** Manager, System Admin

**Response:** `204 No Content`

---

#### POST /api/v1/buildings/{buildingId}/units/{unitId}/tenants

Add tenant to unit.

**Authorization:** Manager, Owner (of unit)

**Request Body:** `AddTenantRequest`

**Response:** `201 Created` → `UnitOccupancyResponse`

---

#### DELETE /api/v1/buildings/{buildingId}/units/{unitId}/tenants/{tenantId}

Remove tenant from unit.

**Authorization:** Manager, Owner (of unit)

**Response:** `204 No Content`

---

### 4. Faults

#### GET /api/v1/faults

List faults.

**Authorization:** Owner, Tenant, Manager

**Query Parameters:** `PaginationParams`, `FaultFilterParams`

**Response:** `200 OK` → `PaginatedResponse<FaultListItem>`

---

#### POST /api/v1/faults

Report new fault.

**Authorization:** Owner, Tenant, Manager

**Request Body:** `CreateFaultRequest`

**Response:** `201 Created` → `FaultResponse`

---

#### GET /api/v1/faults/{id}

Get fault details.

**Authorization:** Owner, Tenant, Manager (in building)

**Response:** `200 OK` → `FaultResponse`

---

#### PUT /api/v1/faults/{id}

Update fault.

**Authorization:** Reporter, Manager, Technical Manager

**Request Body:** `UpdateFaultRequest`

**Response:** `200 OK` → `FaultResponse`

---

#### PATCH /api/v1/faults/{id}/status

Update fault status.

**Authorization:** Manager, Technical Manager

**Request Body:** `UpdateFaultStatusRequest`

**Response:** `200 OK` → `FaultResponse`

---

#### POST /api/v1/faults/{id}/assign

Assign fault to technician.

**Authorization:** Manager

**Request Body:** `AssignFaultRequest`

**Response:** `200 OK` → `FaultResponse`

---

#### POST /api/v1/faults/{id}/resolve

Resolve fault.

**Authorization:** Manager, Technical Manager

**Request Body:** `ResolveFaultRequest`

**Response:** `200 OK` → `FaultResponse`

---

#### POST /api/v1/faults/{id}/reopen

Reopen fault.

**Authorization:** Reporter, Manager

**Request Body:** `ReopenFaultRequest`

**Response:** `200 OK` → `FaultResponse`

---

#### POST /api/v1/faults/{id}/close

Close fault.

**Authorization:** Manager

**Response:** `200 OK` → `FaultResponse`

---

#### POST /api/v1/faults/{id}/escalate

Escalate fault priority.

**Authorization:** Manager, Technical Manager

**Request Body:** `EscalateFaultRequest`

**Response:** `200 OK` → `FaultResponse`

---

#### GET /api/v1/faults/{id}/communications

List fault communications.

**Authorization:** Owner, Tenant, Manager (in building)

**Query Parameters:** `PaginationParams`

**Response:** `200 OK` → `PaginatedResponse<FaultCommunication>`

---

#### POST /api/v1/faults/{id}/communications

Add communication to fault.

**Authorization:** Reporter, Manager, Technical Manager

**Request Body:** `CreateFaultCommunicationRequest`

**Response:** `201 Created` → `FaultCommunication`

---

#### POST /api/v1/faults/{id}/photos

Add photos to fault.

**Authorization:** Reporter, Manager, Technical Manager

**Request Body:** `multipart/form-data` with photos

**Response:** `201 Created` → `FaultPhotoResponse[]`

---

### 5. Voting

#### GET /api/v1/votes

List votes.

**Authorization:** Owner, Manager

**Query Parameters:** `PaginationParams`, `VoteFilterParams`

**Response:** `200 OK` → `PaginatedResponse<VoteListItem>`

---

#### POST /api/v1/votes

Create new vote.

**Authorization:** Manager

**Request Body:** `CreateVoteRequest`

**Response:** `201 Created` → `VoteResponse`

---

#### GET /api/v1/votes/{id}

Get vote details.

**Authorization:** Owner, Manager

**Response:** `200 OK` → `VoteResponse`

---

#### PUT /api/v1/votes/{id}

Update vote (draft only).

**Authorization:** Manager (creator)

**Request Body:** `UpdateVoteRequest`

**Response:** `200 OK` → `VoteResponse`

---

#### POST /api/v1/votes/{id}/publish

Publish vote (start voting).

**Authorization:** Manager (creator)

**Response:** `200 OK` → `VoteResponse`

---

#### POST /api/v1/votes/{id}/cancel

Cancel vote.

**Authorization:** Manager (creator)

**Response:** `200 OK` → `VoteResponse`

---

#### POST /api/v1/votes/{id}/extend

Extend voting deadline.

**Authorization:** Manager (creator)

**Request Body:** `ExtendVoteRequest`

**Response:** `200 OK` → `VoteResponse`

---

#### POST /api/v1/votes/{id}/ballots

Cast ballot.

**Authorization:** Owner (eligible)

**Request Body:** `CastBallotRequest`

**Response:** `201 Created` → `BallotResponse`

---

#### PUT /api/v1/votes/{id}/ballots/{ballotId}

Change ballot (if allowed).

**Authorization:** Owner (ballot owner)

**Request Body:** `CastBallotRequest`

**Response:** `200 OK` → `BallotResponse`

---

#### GET /api/v1/votes/{id}/results

Get vote results.

**Authorization:** Owner, Manager

**Response:** `200 OK` → `VoteResultsResponse`

---

#### GET /api/v1/votes/{id}/results/export

Export vote results.

**Authorization:** Manager

**Query Parameters:** `format`: `pdf` | `xlsx`

**Response:** `200 OK` → File download

---

#### POST /api/v1/votes/{id}/delegate

Delegate voting rights.

**Authorization:** Owner

**Request Body:** `DelegateVoteRequest`

**Response:** `200 OK` → `VoteDelegationResponse`

---

#### POST /api/v1/votes/{id}/remind

Send voting reminder.

**Authorization:** Manager

**Response:** `200 OK` → `ReminderSentResponse`

---

#### GET /api/v1/votes/{id}/comments

List vote comments.

**Authorization:** Owner, Manager

**Query Parameters:** `PaginationParams`

**Response:** `200 OK` → `PaginatedResponse<Comment>`

---

#### POST /api/v1/votes/{id}/comments

Add comment to vote.

**Authorization:** Owner, Manager

**Request Body:** `CreateCommentRequest`

**Response:** `201 Created` → `Comment`

---

### 6. Documents

#### GET /api/v1/documents

List documents.

**Authorization:** Owner, Tenant, Manager

**Query Parameters:** `PaginationParams`, `DocumentFilterParams`

**Response:** `200 OK` → `PaginatedResponse<DocumentListItem>`

---

#### POST /api/v1/documents

Upload document.

**Authorization:** Manager

**Request Body:** `multipart/form-data`

**Response:** `201 Created` → `DocumentResponse`

---

#### GET /api/v1/documents/{id}

Get document details.

**Authorization:** Owner, Tenant, Manager (with access)

**Response:** `200 OK` → `DocumentResponse`

---

#### GET /api/v1/documents/{id}/download

Download document file.

**Authorization:** Owner, Tenant, Manager (with access)

**Response:** `200 OK` → File download

---

#### PUT /api/v1/documents/{id}

Update document metadata.

**Authorization:** Manager

**Request Body:** `UpdateDocumentRequest`

**Response:** `200 OK` → `DocumentResponse`

---

#### DELETE /api/v1/documents/{id}

Delete document.

**Authorization:** Manager

**Response:** `204 No Content`

---

#### POST /api/v1/documents/{id}/versions

Upload new version.

**Authorization:** Manager

**Request Body:** `multipart/form-data`

**Response:** `201 Created` → `DocumentVersionResponse`

---

#### GET /api/v1/documents/{id}/versions

List document versions.

**Authorization:** Owner, Tenant, Manager (with access)

**Response:** `200 OK` → `DocumentVersionResponse[]`

---

#### PUT /api/v1/documents/{id}/access

Update document access permissions.

**Authorization:** Manager

**Request Body:** `UpdateDocumentAccessRequest`

**Response:** `200 OK` → `DocumentResponse`

---

#### POST /api/v1/documents/{id}/share

Share document with specific users.

**Authorization:** Manager

**Request Body:** `ShareDocumentRequest`

**Response:** `200 OK` → `DocumentResponse`

---

#### GET /api/v1/folders

List folders.

**Authorization:** Owner, Tenant, Manager

**Query Parameters:** `parentId` (optional)

**Response:** `200 OK` → `FolderResponse[]`

---

#### POST /api/v1/folders

Create folder.

**Authorization:** Manager

**Request Body:** `CreateFolderRequest`

**Response:** `201 Created` → `FolderResponse`

---

#### PUT /api/v1/folders/{id}

Update folder.

**Authorization:** Manager

**Request Body:** `UpdateFolderRequest`

**Response:** `200 OK` → `FolderResponse`

---

#### DELETE /api/v1/folders/{id}

Delete folder.

**Authorization:** Manager

**Response:** `204 No Content`

---

### 7. Messages

#### GET /api/v1/conversations

List conversations.

**Authorization:** Owner, Tenant, Manager

**Query Parameters:** `PaginationParams`

**Response:** `200 OK` → `PaginatedResponse<ConversationListItem>`

---

#### POST /api/v1/conversations

Create new conversation.

**Authorization:** Owner, Tenant, Manager

**Request Body:** `CreateConversationRequest`

**Response:** `201 Created` → `ConversationResponse`

---

#### GET /api/v1/conversations/{id}

Get conversation details.

**Authorization:** Participant

**Response:** `200 OK` → `ConversationResponse`

---

#### GET /api/v1/conversations/{id}/messages

List messages in conversation.

**Authorization:** Participant

**Query Parameters:** `PaginationParams`, `before` (cursor)

**Response:** `200 OK` → `PaginatedResponse<MessageResponse>`

---

#### POST /api/v1/conversations/{id}/messages

Send message.

**Authorization:** Participant

**Request Body:** `SendMessageRequest`

**Response:** `201 Created` → `MessageResponse`

---

#### DELETE /api/v1/conversations/{id}/messages/{messageId}

Delete message.

**Authorization:** Sender

**Response:** `204 No Content`

---

#### POST /api/v1/conversations/{id}/read

Mark conversation as read.

**Authorization:** Participant

**Response:** `200 OK`

---

#### POST /api/v1/conversations/{id}/archive

Archive conversation.

**Authorization:** Participant

**Response:** `200 OK`

---

#### DELETE /api/v1/conversations/{id}

Delete conversation.

**Authorization:** Participant

**Response:** `204 No Content`

---

### 8. Financial

#### GET /api/v1/accounts/{unitId}

Get unit financial account.

**Authorization:** Owner (of unit), Manager

**Response:** `200 OK` → `FinancialAccountResponse`

---

#### GET /api/v1/accounts/{unitId}/transactions

List account transactions.

**Authorization:** Owner (of unit), Manager

**Query Parameters:** `PaginationParams`, `TransactionFilterParams`

**Response:** `200 OK` → `PaginatedResponse<TransactionResponse>`

---

#### GET /api/v1/invoices

List invoices.

**Authorization:** Owner, Manager

**Query Parameters:** `PaginationParams`, `InvoiceFilterParams`

**Response:** `200 OK` → `PaginatedResponse<InvoiceListItem>`

---

#### POST /api/v1/invoices

Create invoice.

**Authorization:** Manager

**Request Body:** `CreateInvoiceRequest`

**Response:** `201 Created` → `InvoiceResponse`

---

#### GET /api/v1/invoices/{id}

Get invoice details.

**Authorization:** Owner (recipient), Manager

**Response:** `200 OK` → `InvoiceResponse`

---

#### GET /api/v1/invoices/{id}/pdf

Download invoice PDF.

**Authorization:** Owner (recipient), Manager

**Response:** `200 OK` → File download

---

#### POST /api/v1/invoices/{id}/issue

Issue invoice.

**Authorization:** Manager

**Response:** `200 OK` → `InvoiceResponse`

---

#### POST /api/v1/invoices/{id}/cancel

Cancel invoice.

**Authorization:** Manager

**Response:** `200 OK` → `InvoiceResponse`

---

#### POST /api/v1/payments/intent

Create payment intent.

**Authorization:** Owner

**Request Body:** `CreatePaymentIntentRequest`

**Response:** `200 OK` → `PaymentIntentResponse`

---

#### POST /api/v1/payments/webhook

Stripe webhook handler.

**Authorization:** Stripe signature

**Request Body:** Stripe event

**Response:** `200 OK`

---

#### GET /api/v1/payments

List payments.

**Authorization:** Owner, Manager

**Query Parameters:** `PaginationParams`, `PaymentFilterParams`

**Response:** `200 OK` → `PaginatedResponse<PaymentResponse>`

---

#### POST /api/v1/payments/{id}/refund

Process refund.

**Authorization:** Manager

**Request Body:** `RefundRequest`

**Response:** `200 OK` → `PaymentResponse`

---

#### GET /api/v1/budgets

List budgets.

**Authorization:** Owner, Manager

**Query Parameters:** `year`, `buildingId`

**Response:** `200 OK` → `BudgetResponse[]`

---

#### POST /api/v1/budgets

Create budget.

**Authorization:** Manager, Organization Admin

**Request Body:** `CreateBudgetRequest`

**Response:** `201 Created` → `BudgetResponse`

---

#### GET /api/v1/budgets/{id}

Get budget details.

**Authorization:** Owner, Manager

**Response:** `200 OK` → `BudgetResponse`

---

#### PUT /api/v1/budgets/{id}

Update budget.

**Authorization:** Manager, Organization Admin

**Request Body:** `UpdateBudgetRequest`

**Response:** `200 OK` → `BudgetResponse`

---

### 9. Meters

#### GET /api/v1/meters

List meters.

**Authorization:** Owner, Manager

**Query Parameters:** `unitId`, `buildingId`

**Response:** `200 OK` → `MeterResponse[]`

---

#### POST /api/v1/meters

Create meter.

**Authorization:** Manager

**Request Body:** `CreateMeterRequest`

**Response:** `201 Created` → `MeterResponse`

---

#### GET /api/v1/meter-readings

List meter readings.

**Authorization:** Owner, Manager

**Query Parameters:** `PaginationParams`, `MeterReadingFilterParams`

**Response:** `200 OK` → `PaginatedResponse<MeterReadingResponse>`

---

#### POST /api/v1/meter-readings

Submit meter reading.

**Authorization:** Owner

**Request Body:** `SubmitMeterReadingRequest`

**Response:** `201 Created` → `MeterReadingResponse`

---

#### GET /api/v1/meter-readings/{id}

Get meter reading details.

**Authorization:** Owner (submitter), Manager

**Response:** `200 OK` → `MeterReadingResponse`

---

#### PATCH /api/v1/meter-readings/{id}

Update meter reading.

**Authorization:** Owner (submitter, before verification), Manager

**Request Body:** `UpdateMeterReadingRequest`

**Response:** `200 OK` → `MeterReadingResponse`

---

#### POST /api/v1/meter-readings/{id}/verify

Verify meter reading.

**Authorization:** Manager

**Response:** `200 OK` → `MeterReadingResponse`

---

#### POST /api/v1/meter-readings/{id}/reject

Reject meter reading.

**Authorization:** Manager

**Request Body:** `RejectMeterReadingRequest`

**Response:** `200 OK` → `MeterReadingResponse`

---

#### GET /api/v1/meter-readings/export

Export meter readings.

**Authorization:** Manager

**Query Parameters:** `buildingId`, `period`, `format`

**Response:** `200 OK` → File download

---

### 10. Rentals (Short-term)

#### GET /api/v1/reservations

List reservations.

**Authorization:** Property Manager, Owner

**Query Parameters:** `PaginationParams`, `ReservationFilterParams`

**Response:** `200 OK` → `PaginatedResponse<ReservationListItem>`

---

#### POST /api/v1/reservations

Create reservation (manual).

**Authorization:** Property Manager, Owner

**Request Body:** `CreateReservationRequest`

**Response:** `201 Created` → `ReservationResponse`

---

#### GET /api/v1/reservations/{id}

Get reservation details.

**Authorization:** Property Manager, Owner

**Response:** `200 OK` → `ReservationResponse`

---

#### PUT /api/v1/reservations/{id}

Update reservation.

**Authorization:** Property Manager, Owner

**Request Body:** `UpdateReservationRequest`

**Response:** `200 OK` → `ReservationResponse`

---

#### POST /api/v1/reservations/{id}/check-in

Process guest check-in.

**Authorization:** Property Manager, Guest (self)

**Request Body:** `CheckInRequest`

**Response:** `200 OK` → `ReservationResponse`

---

#### POST /api/v1/reservations/{id}/check-out

Process guest check-out.

**Authorization:** Property Manager, Guest (self)

**Request Body:** `CheckOutRequest`

**Response:** `200 OK` → `ReservationResponse`

---

#### POST /api/v1/reservations/{id}/cancel

Cancel reservation.

**Authorization:** Property Manager, Owner

**Request Body:** `CancelReservationRequest`

**Response:** `200 OK` → `ReservationResponse`

---

#### GET /api/v1/reservations/{id}/access-code

Get access code for reservation.

**Authorization:** Property Manager, Guest

**Response:** `200 OK` → `AccessCodeResponse`

---

#### POST /api/v1/reservations/{id}/access-code/regenerate

Regenerate access code.

**Authorization:** Property Manager

**Response:** `200 OK` → `AccessCodeResponse`

---

#### GET /api/v1/reservations/{id}/police-registration

Get police registration.

**Authorization:** Property Manager

**Response:** `200 OK` → `PoliceRegistrationResponse`

---

#### POST /api/v1/reservations/{id}/police-registration/submit

Submit police registration.

**Authorization:** Property Manager

**Response:** `200 OK` → `PoliceRegistrationResponse`

---

#### POST /api/v1/reservations/{id}/rate

Rate guest.

**Authorization:** Property Manager

**Request Body:** `RateGuestRequest`

**Response:** `200 OK`

---

#### GET /api/v1/guests

List guests.

**Authorization:** Property Manager

**Query Parameters:** `PaginationParams`

**Response:** `200 OK` → `PaginatedResponse<GuestListItem>`

---

#### POST /api/v1/guests/{id}/block

Block guest.

**Authorization:** Property Manager

**Request Body:** `BlockGuestRequest`

**Response:** `200 OK`

---

#### GET /api/v1/rental-platforms

List connected platforms.

**Authorization:** Property Manager, Owner

**Response:** `200 OK` → `RentalPlatformConnection[]`

---

#### POST /api/v1/rental-platforms/airbnb/connect

Connect Airbnb account.

**Authorization:** Property Manager, Owner

**Request Body:** `ConnectAirbnbRequest`

**Response:** `200 OK` → `RentalPlatformConnection`

---

#### POST /api/v1/rental-platforms/booking/connect

Connect Booking.com account.

**Authorization:** Property Manager, Owner

**Request Body:** `ConnectBookingRequest`

**Response:** `200 OK` → `RentalPlatformConnection`

---

#### POST /api/v1/rental-platforms/{id}/sync

Trigger platform sync.

**Authorization:** Property Manager

**Response:** `200 OK` → `SyncStatusResponse`

---

#### DELETE /api/v1/rental-platforms/{id}

Disconnect platform.

**Authorization:** Property Manager, Owner

**Response:** `204 No Content`

---

### 11. Listings (Reality Portal)

#### GET /api/v1/listings

Search listings (public).

**Authorization:** None (public)

**Query Parameters:** `ListingSearchParams`

**Response:** `200 OK` → `PaginatedResponse<ListingListItem>`

---

#### GET /api/v1/listings/{id}

Get listing details (public).

**Authorization:** None (public)

**Response:** `200 OK` → `ListingDetailResponse`

---

#### POST /api/v1/listings

Create listing.

**Authorization:** Realtor

**Request Body:** `CreateListingRequest`

**Response:** `201 Created` → `ListingResponse`

---

#### PUT /api/v1/listings/{id}

Update listing.

**Authorization:** Realtor (owner)

**Request Body:** `UpdateListingRequest`

**Response:** `200 OK` → `ListingResponse`

---

#### DELETE /api/v1/listings/{id}

Delete listing.

**Authorization:** Realtor (owner), Agency Manager

**Response:** `204 No Content`

---

#### PATCH /api/v1/listings/{id}/status

Update listing status.

**Authorization:** Realtor (owner)

**Request Body:** `UpdateListingStatusRequest`

**Response:** `200 OK` → `ListingResponse`

---

#### POST /api/v1/listings/{id}/publish

Publish listing.

**Authorization:** Realtor (owner)

**Response:** `200 OK` → `ListingResponse`

---

#### POST /api/v1/listings/{id}/feature

Feature listing.

**Authorization:** Realtor (owner)

**Request Body:** `FeatureListingRequest`

**Response:** `200 OK` → `ListingResponse`

---

#### POST /api/v1/listings/{id}/photos

Upload listing photos.

**Authorization:** Realtor (owner)

**Request Body:** `multipart/form-data`

**Response:** `201 Created` → `ListingPhotoResponse[]`

---

#### PUT /api/v1/listings/{id}/photos/order

Reorder listing photos.

**Authorization:** Realtor (owner)

**Request Body:** `ReorderPhotosRequest`

**Response:** `200 OK`

---

#### DELETE /api/v1/listings/{id}/photos/{photoId}

Delete listing photo.

**Authorization:** Realtor (owner)

**Response:** `204 No Content`

---

#### GET /api/v1/listings/{id}/analytics

Get listing analytics.

**Authorization:** Realtor (owner)

**Response:** `200 OK` → `ListingAnalyticsResponse`

---

### 12. Agencies (Reality Portal)

#### GET /api/v1/agencies

List agencies.

**Authorization:** None (public)

**Query Parameters:** `PaginationParams`

**Response:** `200 OK` → `PaginatedResponse<AgencyListItem>`

---

#### POST /api/v1/agencies

Create agency.

**Authorization:** Authenticated user

**Request Body:** `CreateAgencyRequest`

**Response:** `201 Created` → `AgencyResponse`

---

#### GET /api/v1/agencies/{id}

Get agency details.

**Authorization:** None (public)

**Response:** `200 OK` → `AgencyDetailResponse`

---

#### PUT /api/v1/agencies/{id}

Update agency.

**Authorization:** Agency Owner

**Request Body:** `UpdateAgencyRequest`

**Response:** `200 OK` → `AgencyResponse`

---

#### PUT /api/v1/agencies/{id}/branding

Update agency branding.

**Authorization:** Agency Owner

**Request Body:** `AgencyBrandingRequest`

**Response:** `200 OK` → `AgencyResponse`

---

#### GET /api/v1/agencies/{id}/realtors

List agency realtors.

**Authorization:** Agency Owner, Agency Manager

**Response:** `200 OK` → `RealtorListItem[]`

---

#### POST /api/v1/agencies/{id}/realtors/invite

Invite realtor to agency.

**Authorization:** Agency Owner, Agency Manager

**Request Body:** `InviteRealtorRequest`

**Response:** `200 OK`

---

#### POST /api/v1/agencies/invitations/{token}/accept

Accept agency invitation.

**Authorization:** Realtor

**Response:** `200 OK` → `AgencyMembershipResponse`

---

#### DELETE /api/v1/agencies/{id}/realtors/{realtorId}

Remove realtor from agency.

**Authorization:** Agency Owner

**Response:** `204 No Content`

---

#### POST /api/v1/agencies/{id}/realtors/{realtorId}/suspend

Suspend realtor.

**Authorization:** Agency Owner

**Response:** `200 OK`

---

#### GET /api/v1/agencies/{id}/statistics

Get agency statistics.

**Authorization:** Agency Owner, Agency Manager

**Response:** `200 OK` → `AgencyStatisticsResponse`

---

### 13. Portal Users

#### GET /api/v1/portal/me

Get current portal user.

**Authorization:** Portal User

**Response:** `200 OK` → `PortalUserResponse`

---

#### PUT /api/v1/portal/me

Update portal user profile.

**Authorization:** Portal User

**Request Body:** `UpdatePortalUserRequest`

**Response:** `200 OK` → `PortalUserResponse`

---

#### GET /api/v1/portal/favorites

List favorites.

**Authorization:** Portal User

**Query Parameters:** `PaginationParams`

**Response:** `200 OK` → `PaginatedResponse<ListingListItem>`

---

#### POST /api/v1/portal/favorites/{listingId}

Add to favorites.

**Authorization:** Portal User

**Response:** `201 Created`

---

#### DELETE /api/v1/portal/favorites/{listingId}

Remove from favorites.

**Authorization:** Portal User

**Response:** `204 No Content`

---

#### GET /api/v1/portal/saved-searches

List saved searches.

**Authorization:** Portal User

**Response:** `200 OK` → `SavedSearchResponse[]`

---

#### POST /api/v1/portal/saved-searches

Create saved search.

**Authorization:** Portal User

**Request Body:** `CreateSavedSearchRequest`

**Response:** `201 Created` → `SavedSearchResponse`

---

#### PUT /api/v1/portal/saved-searches/{id}

Update saved search.

**Authorization:** Portal User

**Request Body:** `UpdateSavedSearchRequest`

**Response:** `200 OK` → `SavedSearchResponse`

---

#### DELETE /api/v1/portal/saved-searches/{id}

Delete saved search.

**Authorization:** Portal User

**Response:** `204 No Content`

---

#### GET /api/v1/portal/inquiries

List inquiries.

**Authorization:** Portal User

**Query Parameters:** `PaginationParams`

**Response:** `200 OK` → `PaginatedResponse<InquiryListItem>`

---

#### POST /api/v1/inquiries

Send inquiry.

**Authorization:** Portal User

**Request Body:** `CreateInquiryRequest`

**Response:** `201 Created` → `InquiryResponse`

---

#### GET /api/v1/inquiries/{id}

Get inquiry details.

**Authorization:** Portal User (sender), Realtor (recipient)

**Response:** `200 OK` → `InquiryResponse`

---

#### POST /api/v1/inquiries/{id}/respond

Respond to inquiry.

**Authorization:** Realtor (recipient)

**Request Body:** `RespondToInquiryRequest`

**Response:** `200 OK` → `InquiryResponse`

---

#### POST /api/v1/inquiries/{id}/schedule-viewing

Schedule viewing.

**Authorization:** Realtor (recipient)

**Request Body:** `ScheduleViewingRequest`

**Response:** `200 OK` → `ViewingResponse`

---

### 14. Notifications

#### GET /api/v1/notifications

List notifications.

**Authorization:** Authenticated user

**Query Parameters:** `PaginationParams`

**Response:** `200 OK` → `PaginatedResponse<NotificationResponse>`

---

#### POST /api/v1/notifications/{id}/read

Mark notification as read.

**Authorization:** Recipient

**Response:** `200 OK`

---

#### POST /api/v1/notifications/read-all

Mark all notifications as read.

**Authorization:** Authenticated user

**Response:** `200 OK`

---

#### GET /api/v1/notifications/preferences

Get notification preferences.

**Authorization:** Authenticated user

**Response:** `200 OK` → `NotificationPreferencesResponse`

---

#### PUT /api/v1/notifications/preferences

Update notification preferences.

**Authorization:** Authenticated user

**Request Body:** `UpdateNotificationPreferencesRequest`

**Response:** `200 OK` → `NotificationPreferencesResponse`

---

#### POST /api/v1/devices

Register device for push notifications.

**Authorization:** Authenticated user

**Request Body:** `RegisterDeviceRequest`

**Response:** `201 Created` → `DeviceResponse`

---

#### DELETE /api/v1/devices/{id}

Unregister device.

**Authorization:** Device owner

**Response:** `204 No Content`

---

---

## DTOs (Data Transfer Objects)

### Common DTOs

#### PaginationParams

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| page | integer | No | 1 | Page number (1-indexed) |
| limit | integer | No | 20 | Items per page (max 100) |
| sort | string | No | - | Sort field |
| order | string | No | desc | Sort order: asc, desc |

#### PaginatedResponse<T>

| Field | Type | Description |
|-------|------|-------------|
| data | T[] | Array of items |
| pagination.page | integer | Current page |
| pagination.limit | integer | Items per page |
| pagination.total | integer | Total items |
| pagination.totalPages | integer | Total pages |
| pagination.hasMore | boolean | Has more pages |

#### ErrorResponse

| Field | Type | Description |
|-------|------|-------------|
| error.code | string | Error code (e.g., VALIDATION_ERROR) |
| error.message | string | Human-readable message |
| error.details | ValidationError[] | Field-level errors |
| error.requestId | string | Request ID for debugging |

#### ValidationError

| Field | Type | Description |
|-------|------|-------------|
| field | string | Field path (e.g., "items[0].name") |
| code | string | Error code (e.g., REQUIRED) |
| message | string | Human-readable message |

#### Address

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| street | string | Yes | Street name |
| streetNumber | string | Yes | Street number |
| city | string | Yes | City |
| postalCode | string | Yes | Postal code |
| country | string | Yes | ISO 3166-1 alpha-2 |
| coordinates | Coordinates | No | GPS coordinates |

#### Coordinates

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| latitude | number | Yes | Latitude (-90 to 90) |
| longitude | number | Yes | Longitude (-180 to 180) |

#### Money

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| amount | decimal | Yes | Amount (2 decimal places) |
| currency | string | Yes | ISO 4217 currency code |

#### DateRange

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| start | date | Yes | Start date (ISO 8601) |
| end | date | Yes | End date (ISO 8601) |

---

### Auth DTOs

#### RegisterRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| email | string | Yes | Valid email, max 255 | Email address |
| password | string | Yes | 8-100 chars, complexity | Password |
| firstName | string | Yes | 1-100 chars | First name |
| lastName | string | Yes | 1-100 chars | Last name |
| invitationCode | string | Yes | Valid code | Invitation code |
| language | string | No | sk, en, cs, de | Preferred language |

#### LoginRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| email | string | Yes | Valid email | Email address |
| password | string | Yes | - | Password |
| rememberMe | boolean | No | Default: false | Extended session |

#### AuthResponse

| Field | Type | Description |
|-------|------|-------------|
| accessToken | string | JWT access token |
| refreshToken | string | JWT refresh token |
| expiresIn | integer | Access token TTL (seconds) |
| user | UserResponse | User details |

#### MfaRequiredResponse

| Field | Type | Description |
|-------|------|-------------|
| mfaRequired | boolean | Always true |
| mfaToken | string | Temporary token for MFA verification |
| mfaMethod | string | TOTP, SMS |

#### MfaVerifyRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| mfaToken | string | Yes | Valid token | MFA token from login |
| code | string | Yes | 6 digits | MFA code |
| trustDevice | boolean | No | Default: false | Trust this device |

#### RefreshTokenRequest

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| refreshToken | string | Yes | Refresh token |

#### TokenResponse

| Field | Type | Description |
|-------|------|-------------|
| accessToken | string | JWT access token |
| refreshToken | string | JWT refresh token |
| expiresIn | integer | Access token TTL (seconds) |

#### MfaSetupRequest

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| method | string | Yes | TOTP, SMS |
| phoneNumber | string | Cond. | Required for SMS |

#### MfaSetupResponse

| Field | Type | Description |
|-------|------|-------------|
| secret | string | TOTP secret (for TOTP) |
| qrCodeUrl | string | QR code data URL |
| backupCodes | string[] | One-time backup codes |

#### ForgotPasswordRequest

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| email | string | Yes | Email address |

#### ResetPasswordRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| token | string | Yes | Valid token | Reset token from email |
| password | string | Yes | 8-100 chars, complexity | New password |

#### ChangePasswordRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| currentPassword | string | Yes | - | Current password |
| newPassword | string | Yes | 8-100 chars, complexity | New password |

---

### Building DTOs

#### CreateBuildingRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| name | string | No | Max 255 | Building name |
| address | Address | Yes | Valid address | Building address |
| yearBuilt | integer | No | 1800-current | Year built |
| totalFloors | integer | No | 1-200 | Number of floors |
| amenities | string[] | No | Valid amenity codes | Available amenities |

#### BuildingResponse

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Building ID |
| organizationId | UUID | Organization ID |
| name | string | Building name |
| address | Address | Full address |
| yearBuilt | integer | Year built |
| totalFloors | integer | Number of floors |
| totalUnits | integer | Total units count |
| amenities | string[] | Available amenities |
| status | string | active, archived |
| createdAt | datetime | Creation timestamp |
| updatedAt | datetime | Last update timestamp |

#### CreateUnitRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| designation | string | Yes | 1-50 chars | Unit number/name |
| entranceId | UUID | No | Valid entrance | Entrance ID |
| floor | integer | Yes | 0-200 | Floor number |
| sizeSqm | decimal | No | > 0 | Size in m² |
| rooms | integer | No | 0-50 | Number of rooms |
| type | string | Yes | apartment, office, etc. | Unit type |
| ownershipShare | decimal | No | 0-100, default 0 | Ownership share % |

#### UnitResponse

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Unit ID |
| buildingId | UUID | Building ID |
| entranceId | UUID | Entrance ID |
| designation | string | Unit number |
| floor | integer | Floor number |
| sizeSqm | decimal | Size in m² |
| rooms | integer | Number of rooms |
| type | string | Unit type |
| ownershipShare | decimal | Ownership share % |
| owners | UnitOwnershipResponse[] | Current owners |
| occupants | UnitOccupancyResponse[] | Current occupants |
| status | string | active, archived |

#### AssignOwnerRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| userId | UUID | No | Existing user | User ID (if existing) |
| email | string | Cond. | Valid email | Email (if inviting) |
| ownershipShare | decimal | Yes | 0-100 | Ownership percentage |
| effectiveFrom | date | No | Default: today | Start date |

---

### Fault DTOs

#### CreateFaultRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| buildingId | UUID | Yes | Valid building | Building ID |
| unitId | UUID | No | Valid unit | Unit ID (if unit-specific) |
| title | string | Yes | 5-200 chars | Fault title |
| description | string | Yes | 10-5000 chars | Detailed description |
| category | string | Yes | Valid category | Fault category |
| location | string | No | Max 255 | Location description |
| priority | string | No | low, medium, high, critical | Suggested priority |

#### FaultResponse

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Fault ID |
| buildingId | UUID | Building ID |
| unitId | UUID | Unit ID |
| reporterId | UUID | Reporter user ID |
| reporter | UserSummary | Reporter details |
| title | string | Fault title |
| description | string | Description |
| category | string | Category |
| location | string | Location |
| priority | string | Priority level |
| status | string | new, in_progress, resolved, closed |
| assigneeId | UUID | Assigned technician ID |
| assignee | UserSummary | Assignee details |
| photos | FaultPhoto[] | Attached photos |
| estimatedResolution | datetime | Estimated resolution time |
| resolvedAt | datetime | Resolution timestamp |
| resolution | string | Resolution description |
| createdAt | datetime | Creation timestamp |
| updatedAt | datetime | Last update |

#### UpdateFaultStatusRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| status | string | Yes | Valid transition | New status |
| note | string | No | Max 1000 | Status change note |

#### AssignFaultRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| assigneeId | UUID | Yes | Valid technician | Technician user ID |
| note | string | No | Max 1000 | Assignment note |
| estimatedHours | integer | No | 1-1000 | Estimated hours |

#### ResolveFaultRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| resolution | string | Yes | 10-5000 chars | Resolution description |
| actualHours | decimal | No | > 0 | Actual hours spent |

#### ReopenFaultRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| reason | string | Yes | 10-1000 chars | Reopen reason |

---

### Voting DTOs

#### CreateVoteRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| buildingId | UUID | Yes | Valid building | Building ID |
| title | string | Yes | 5-255 chars | Vote title |
| description | string | No | Max 5000 | Description |
| options | VoteOptionInput[] | Yes | 2-10 options | Vote options |
| startDate | datetime | Yes | Future date | Start time |
| endDate | datetime | Yes | After startDate | End time |
| settings | VoteSettings | No | - | Vote settings |

#### VoteOptionInput

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| text | string | Yes | 1-500 chars | Option text |
| order | integer | No | 0-99 | Display order |

#### VoteSettings

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| allowChangeVote | boolean | No | false | Allow changing vote |
| requiresQuorum | boolean | No | false | Require quorum |
| quorumPercentage | decimal | Cond. | 50 | Quorum % (if required) |
| weightByOwnership | boolean | No | true | Weight by ownership |
| anonymousResults | boolean | No | false | Hide individual votes |

#### VoteResponse

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Vote ID |
| buildingId | UUID | Building ID |
| creatorId | UUID | Creator user ID |
| title | string | Vote title |
| description | string | Description |
| options | VoteOption[] | Vote options |
| settings | VoteSettings | Vote settings |
| status | string | draft, active, completed, cancelled |
| startDate | datetime | Start time |
| endDate | datetime | End time |
| totalEligible | integer | Eligible voters count |
| totalVoted | integer | Votes cast count |
| participationRate | decimal | Participation % |
| createdAt | datetime | Creation timestamp |

#### CastBallotRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| optionId | UUID | Yes | Valid option | Selected option ID |
| unitId | UUID | Yes | Eligible unit | Unit voting for |

#### VoteResultsResponse

| Field | Type | Description |
|-------|------|-------------|
| voteId | UUID | Vote ID |
| status | string | Vote status |
| totalEligible | integer | Eligible voters |
| totalVoted | integer | Votes cast |
| participationRate | decimal | Participation % |
| quorumReached | boolean | Quorum met |
| options | VoteOptionResult[] | Per-option results |
| calculatedAt | datetime | Calculation timestamp |

#### VoteOptionResult

| Field | Type | Description |
|-------|------|-------------|
| optionId | UUID | Option ID |
| text | string | Option text |
| voteCount | integer | Number of votes |
| weightedCount | decimal | Weighted votes |
| percentage | decimal | Percentage of total |
| isWinner | boolean | Is winning option |

---

### Financial DTOs

#### CreateInvoiceRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| unitId | UUID | Yes | Valid unit | Unit ID |
| items | InvoiceItemInput[] | Yes | 1-50 items | Invoice items |
| dueDate | date | Yes | Future date | Due date |
| note | string | No | Max 1000 | Invoice note |

#### InvoiceItemInput

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| description | string | Yes | 1-255 chars | Item description |
| amount | Money | Yes | > 0 | Item amount |
| quantity | decimal | No | Default: 1 | Quantity |

#### InvoiceResponse

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Invoice ID |
| number | string | Invoice number |
| unitId | UUID | Unit ID |
| unit | UnitSummary | Unit details |
| ownerId | UUID | Owner ID |
| owner | UserSummary | Owner details |
| items | InvoiceItem[] | Invoice items |
| subtotal | Money | Subtotal |
| tax | Money | Tax amount |
| total | Money | Total amount |
| status | string | draft, issued, paid, overdue, cancelled |
| dueDate | date | Due date |
| paidAt | datetime | Payment timestamp |
| issuedAt | datetime | Issue timestamp |
| pdfUrl | string | PDF download URL |

#### CreatePaymentIntentRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| invoiceId | UUID | Yes | Valid invoice | Invoice to pay |
| paymentMethod | string | No | card, bank_transfer | Payment method |

#### PaymentIntentResponse

| Field | Type | Description |
|-------|------|-------------|
| clientSecret | string | Stripe client secret |
| paymentIntentId | string | Stripe payment intent ID |
| amount | Money | Amount to pay |
| status | string | Payment intent status |

---

### Meter DTOs

#### SubmitMeterReadingRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| meterId | UUID | Yes | Valid meter | Meter ID |
| value | decimal | Yes | >= previous reading | Reading value |
| readingDate | date | No | Default: today | Date of reading |
| photo | File | No | Max 5MB, image | Photo of meter |

#### MeterReadingResponse

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Reading ID |
| meterId | UUID | Meter ID |
| meter | MeterSummary | Meter details |
| unitId | UUID | Unit ID |
| submitterId | UUID | Submitter ID |
| value | decimal | Reading value |
| previousValue | decimal | Previous reading |
| consumption | decimal | Calculated consumption |
| readingDate | date | Reading date |
| photoUrl | string | Photo URL |
| ocrValue | decimal | OCR extracted value |
| ocrConfidence | decimal | OCR confidence (0-1) |
| status | string | pending, verified, rejected |
| verifiedBy | UUID | Verifier ID |
| verifiedAt | datetime | Verification timestamp |
| rejectionReason | string | Rejection reason |
| submittedAt | datetime | Submission timestamp |

---

### Rental DTOs

#### CreateReservationRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| unitId | UUID | Yes | Valid rental unit | Unit ID |
| guest | GuestInput | Yes | Valid guest | Guest details |
| dates | DateRange | Yes | Future, max 90 days | Stay dates |
| platform | string | No | direct, airbnb, booking | Booking source |
| externalId | string | No | Max 100 | External booking ID |
| totalAmount | Money | No | > 0 | Total amount |
| notes | string | No | Max 1000 | Notes |

#### GuestInput

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| firstName | string | Yes | 1-100 chars | First name |
| lastName | string | Yes | 1-100 chars | Last name |
| email | string | Yes | Valid email | Email |
| phone | string | No | Valid phone | Phone number |
| nationality | string | Yes | ISO 3166-1 alpha-2 | Nationality |
| dateOfBirth | date | No | Past date | Date of birth |
| idType | string | No | passport, id_card | ID document type |
| idNumber | string | No | Max 50 | ID document number |

#### ReservationResponse

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Reservation ID |
| unitId | UUID | Unit ID |
| unit | UnitSummary | Unit details |
| guest | GuestResponse | Guest details |
| dates | DateRange | Stay dates |
| platform | string | Booking source |
| externalId | string | External booking ID |
| status | string | pending, confirmed, checked_in, checked_out, cancelled |
| accessCode | string | Access code (masked) |
| totalAmount | Money | Total amount |
| checkInAt | datetime | Check-in timestamp |
| checkOutAt | datetime | Check-out timestamp |
| policeRegistration | PoliceRegistrationSummary | Registration status |
| rating | GuestRating | Guest rating |
| createdAt | datetime | Creation timestamp |

#### CheckInRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| accessCode | string | Yes | Valid code | Access code |
| idDocument | File | No | Max 5MB, image | ID document scan |

---

### Listing DTOs

#### CreateListingRequest

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| propertyType | string | Yes | apartment, house, etc. | Property type |
| transactionType | string | Yes | sale, rent | Transaction type |
| title | string | Yes | 5-255 chars | Listing title |
| description | string | Yes | 50-10000 chars | Description |
| address | Address | Yes | Valid address | Property address |
| price | Money | Yes | > 0 | Price |
| sizeSqm | decimal | Yes | > 0 | Size in m² |
| rooms | integer | No | 0-50 | Number of rooms |
| bathrooms | integer | No | 0-20 | Number of bathrooms |
| floor | integer | No | 0-200 | Floor number |
| totalFloors | integer | No | 1-200 | Total floors |
| yearBuilt | integer | No | 1800-current | Year built |
| features | string[] | No | Valid features | Property features |

#### ListingSearchParams

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| q | string | No | Full-text search |
| propertyType | string[] | No | Property types |
| transactionType | string | No | sale, rent |
| city | string | No | City filter |
| district | string | No | District filter |
| priceMin | decimal | No | Minimum price |
| priceMax | decimal | No | Maximum price |
| sizeMin | decimal | No | Minimum size |
| sizeMax | decimal | No | Maximum size |
| roomsMin | integer | No | Minimum rooms |
| roomsMax | integer | No | Maximum rooms |
| features | string[] | No | Required features |
| nearPoint | Coordinates | No | Location center |
| radiusKm | decimal | No | Search radius |
| sort | string | No | price, size, date |
| page | integer | No | Page number |
| limit | integer | No | Items per page |

#### ListingDetailResponse

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Listing ID |
| realtorId | UUID | Realtor ID |
| realtor | RealtorSummary | Realtor details |
| agencyId | UUID | Agency ID |
| agency | AgencySummary | Agency details |
| propertyType | string | Property type |
| transactionType | string | Transaction type |
| title | string | Listing title |
| description | string | Description |
| address | Address | Property address |
| price | Money | Current price |
| pricePerSqm | Money | Price per m² |
| previousPrice | Money | Previous price (if changed) |
| sizeSqm | decimal | Size in m² |
| rooms | integer | Number of rooms |
| bathrooms | integer | Number of bathrooms |
| floor | integer | Floor number |
| totalFloors | integer | Total floors |
| yearBuilt | integer | Year built |
| features | Feature[] | Property features |
| photos | ListingPhoto[] | Photos |
| virtualTourUrl | string | Virtual tour URL |
| status | string | draft, active, pending, sold, etc. |
| isFeatured | boolean | Is featured |
| views | integer | View count |
| favorites | integer | Favorites count |
| publishedAt | datetime | Publication date |
| createdAt | datetime | Creation timestamp |
| updatedAt | datetime | Last update |

---

## Validation Rules

### Field Constraints

#### String Validations

| Constraint | Usage | Example |
|------------|-------|---------|
| minLength | Minimum characters | `minLength: 5` |
| maxLength | Maximum characters | `maxLength: 255` |
| pattern | Regex pattern | `pattern: ^[a-zA-Z]+$` |
| email | Valid email format | RFC 5322 |
| url | Valid URL format | RFC 3986 |
| uuid | Valid UUID v4 | RFC 4122 |

#### Number Validations

| Constraint | Usage | Example |
|------------|-------|---------|
| minimum | Minimum value | `minimum: 0` |
| maximum | Maximum value | `maximum: 100` |
| exclusiveMinimum | Greater than | `exclusiveMinimum: 0` |
| multipleOf | Divisibility | `multipleOf: 0.01` |

#### Date Validations

| Constraint | Usage | Example |
|------------|-------|---------|
| past | Must be in past | Birth date |
| future | Must be in future | Due date |
| pastOrPresent | Past or today | Reading date |
| futureOrPresent | Today or future | Start date |

#### Array Validations

| Constraint | Usage | Example |
|------------|-------|---------|
| minItems | Minimum items | `minItems: 1` |
| maxItems | Maximum items | `maxItems: 10` |
| uniqueItems | No duplicates | `uniqueItems: true` |

### Cross-Field Validations

| Rule | Fields | Description |
|------|--------|-------------|
| DateRange | start, end | end >= start |
| PasswordConfirm | password, confirmPassword | Must match |
| ConditionalRequired | field, condition | Required if condition met |
| SumEquals | shares[] | Sum must equal 100% |
| AtLeastOne | field1, field2, ... | At least one required |

### Business Rules

| Domain | Rule | Validation |
|--------|------|------------|
| Auth | Password complexity | Min 8 chars, 1 upper, 1 lower, 1 digit |
| Auth | Account lockout | Lock after 5 failed attempts |
| Building | Unique address | No duplicate addresses in org |
| Unit | Ownership sum | Shares must sum to 100% |
| Fault | Status transition | Only valid transitions allowed |
| Vote | Eligible voter | Must own unit in building |
| Vote | One vote per unit | Cannot vote twice for same unit |
| Invoice | Positive amount | Total must be > 0 |
| Meter | Increasing value | New value >= previous value |
| Reservation | No overlap | Cannot overlap with existing |
| Listing | Active limit | Max 50 active listings per realtor |

### Error Codes

| Code | HTTP | Description |
|------|------|-------------|
| VALIDATION_ERROR | 400 | Field validation failed |
| REQUIRED | 400 | Required field missing |
| INVALID_FORMAT | 400 | Wrong format (email, UUID, etc.) |
| MIN_LENGTH | 400 | Below minimum length |
| MAX_LENGTH | 400 | Exceeds maximum length |
| MIN_VALUE | 400 | Below minimum value |
| MAX_VALUE | 400 | Exceeds maximum value |
| INVALID_ENUM | 400 | Not a valid enum value |
| INVALID_REFERENCE | 400 | Referenced entity not found |
| DUPLICATE | 409 | Duplicate value |
| INVALID_STATE | 422 | Invalid state transition |
| BUSINESS_RULE | 422 | Business rule violation |
| UNAUTHORIZED | 401 | Not authenticated |
| FORBIDDEN | 403 | Insufficient permissions |
| NOT_FOUND | 404 | Resource not found |
| RATE_LIMITED | 429 | Too many requests |

---

## State Machines

### 1. Fault State Machine

```
                                    ┌──────────────────────────────────────┐
                                    │                                      │
                                    ▼                                      │
                               ┌─────────┐                                 │
                        ┌─────►│   New   │◄────────────────────┐           │
                        │      └────┬────┘                     │           │
                        │           │                          │           │
                        │           │ assign                   │           │
                        │           ▼                          │           │
                        │      ┌─────────────┐                 │           │
                        │      │ In Progress │─────────────────┤           │
                        │      └──────┬──────┘                 │           │
                        │             │                        │           │
                        │             │ resolve                │ reopen    │
                        │             ▼                        │           │
                        │      ┌──────────┐                    │           │
        close (direct)  │      │ Resolved │────────────────────┘           │
                        │      └────┬─────┘                                │
                        │           │                                      │
                        │           │ close                                │
                        │           ▼                                      │
                        │      ┌────────┐                                  │
                        └──────│ Closed │──────────────────────────────────┘
                               └────────┘                         reopen (admin)
```

**States:**

| State | Description |
|-------|-------------|
| New | Fault reported, awaiting assignment |
| InProgress | Assigned to technician, being worked on |
| Resolved | Work completed, awaiting confirmation |
| Closed | Fault fully resolved and closed |

**Transitions:**

| From | To | Trigger | Guard | Side Effects |
|------|----|---------| ------|--------------|
| New | InProgress | `assign` | assigneeId provided | Notify assignee, set estimatedResolution |
| New | Closed | `close` | - | Notify reporter |
| InProgress | Resolved | `resolve` | resolution provided | Notify reporter, set resolvedAt |
| InProgress | InProgress | `reassign` | new assigneeId | Notify both assignees |
| InProgress | Closed | `close` | - | Notify reporter |
| Resolved | Closed | `close` | - | Archive fault |
| Resolved | InProgress | `reopen` | reason provided | Notify assignee, clear resolvedAt |
| Closed | InProgress | `reopen` | admin role, reason | Audit log, notify original assignee |

**Escalation Rules:**

| Condition | Action |
|-----------|--------|
| New > 24h | Notify manager |
| InProgress > SLA | Escalate priority |
| Critical priority | Immediate notification |

---

### 2. Vote State Machine

```
                    ┌─────────┐
                    │  Draft  │
                    └────┬────┘
                         │
                         │ publish
                         ▼
                    ┌─────────┐
           ┌────────│ Active  │────────┐
           │        └────┬────┘        │
           │             │             │
           │ cancel      │ end         │ cancel
           │             ▼             │
           │       ┌───────────┐       │
           │       │ Completed │       │
           │       └───────────┘       │
           │                           │
           └──────────►┌───────────┐◄──┘
                       │ Cancelled │
                       └───────────┘
```

**States:**

| State | Description |
|-------|-------------|
| Draft | Vote created, not yet published |
| Active | Voting in progress |
| Completed | Voting period ended, results available |
| Cancelled | Vote cancelled before completion |

**Transitions:**

| From | To | Trigger | Guard | Side Effects |
|------|----|---------|-------|--------------|
| Draft | Active | `publish` | startDate reached or manual | Notify eligible voters |
| Draft | Draft | `update` | - | - |
| Active | Completed | `end` | endDate reached | Calculate results, notify all |
| Active | Active | `extend` | new endDate > current | Notify voters of extension |
| Active | Cancelled | `cancel` | manager role | Notify all voters |
| Draft | Cancelled | `cancel` | - | - |

**Time-Based Triggers:**

| Condition | Action |
|-----------|--------|
| startDate reached | Auto-transition to Active |
| endDate reached | Auto-transition to Completed |
| 24h before end | Send reminder to non-voters |

---

### 3. Announcement State Machine

```
              ┌─────────┐
              │  Draft  │
              └────┬────┘
                   │
         ┌─────────┼─────────┐
         │         │         │
         │ publish │ schedule│
         │         │         │
         ▼         │         ▼
    ┌───────────┐  │    ┌───────────┐
    │ Published │  │    │ Scheduled │
    └─────┬─────┘  │    └─────┬─────┘
          │        │          │
          │        │          │ scheduledAt reached
          │        │          │
          │        └──────────┘
          │              │
          │ archive      │ publish
          ▼              ▼
    ┌──────────┐    ┌───────────┐
    │ Archived │    │ Published │
    └──────────┘    └───────────┘
```

**States:**

| State | Description |
|-------|-------------|
| Draft | Created, not visible to users |
| Scheduled | Set to publish at future time |
| Published | Visible to target audience |
| Archived | Hidden but preserved for history |

**Transitions:**

| From | To | Trigger | Guard | Side Effects |
|------|----|---------|-------|--------------|
| Draft | Published | `publish` | - | Notify target users, send push |
| Draft | Scheduled | `schedule` | scheduledAt in future | - |
| Scheduled | Published | `publish` | scheduledAt reached | Notify target users |
| Scheduled | Draft | `unschedule` | - | - |
| Published | Archived | `archive` | - | Remove from active list |
| Archived | Published | `unarchive` | - | Add back to active list |

---

### 4. Reservation State Machine

```
                      ┌──────────┐
                      │ Pending  │
                      └────┬─────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
              │ confirm    │            │ cancel
              │            │            │
              ▼            │            ▼
         ┌───────────┐     │       ┌───────────┐
         │ Confirmed │     │       │ Cancelled │
         └─────┬─────┘     │       └───────────┘
               │           │
               │ check_in  │
               │           │
               ▼           │
         ┌───────────┐     │
         │ CheckedIn │     │
         └─────┬─────┘     │
               │           │
               │ check_out │
               │           │
               ▼           │
        ┌────────────┐     │
        │ CheckedOut │◄────┘ (auto if no check-in by end+1)
        └────────────┘
```

**States:**

| State | Description |
|-------|-------------|
| Pending | Reservation created, awaiting confirmation |
| Confirmed | Confirmed, access code generated |
| CheckedIn | Guest has checked in |
| CheckedOut | Guest has checked out |
| Cancelled | Reservation cancelled |

**Transitions:**

| From | To | Trigger | Guard | Side Effects |
|------|----|---------|-------|--------------|
| Pending | Confirmed | `confirm` | - | Generate access code, notify guest |
| Pending | Cancelled | `cancel` | - | Notify guest if email provided |
| Confirmed | CheckedIn | `check_in` | valid access code | Notify manager, log entry |
| Confirmed | Cancelled | `cancel` | before check-in time | Deactivate access code |
| CheckedIn | CheckedOut | `check_out` | - | Deactivate access code, rate prompt |
| Confirmed | CheckedOut | auto | end date + 24h passed | Mark as no-show |

---

### 5. Delegation State Machine

```
              ┌─────────┐
              │ Pending │
              └────┬────┘
                   │
         ┌─────────┼─────────┐
         │         │         │
         │ accept  │ decline │
         │         │         │
         ▼         │         ▼
    ┌──────────┐   │    ┌──────────┐
    │ Accepted │   │    │ Declined │
    └────┬─────┘   │    └──────────┘
         │         │
         │         │
    ┌────┴─────────┴────┐
    │                   │
    │ revoke     expire │
    │                   │
    ▼                   ▼
┌─────────┐       ┌─────────┐
│ Revoked │       │ Expired │
└─────────┘       └─────────┘
```

**States:**

| State | Description |
|-------|-------------|
| Pending | Invitation sent, awaiting response |
| Accepted | Delegate has accepted rights |
| Declined | Delegate declined invitation |
| Revoked | Owner revoked delegation |
| Expired | Delegation expired (time-based) |

**Transitions:**

| From | To | Trigger | Guard | Side Effects |
|------|----|---------|-------|--------------|
| Pending | Accepted | `accept` | delegate action | Assign rights, notify owner |
| Pending | Declined | `decline` | delegate action | Notify owner |
| Pending | Expired | auto | invitation > 7 days | - |
| Accepted | Revoked | `revoke` | owner action | Remove rights, notify delegate |
| Accepted | Expired | auto | expiresAt reached | Remove rights, notify both |

---

### 6. Invoice State Machine

```
              ┌─────────┐
              │  Draft  │
              └────┬────┘
                   │
                   │ issue
                   │
                   ▼
              ┌────────┐
         ┌────│ Issued │────┐
         │    └────┬───┘    │
         │         │        │
         │ pay     │        │ cancel
         │         │ overdue│
         ▼         ▼        ▼
     ┌──────┐  ┌─────────┐  ┌───────────┐
     │ Paid │  │ Overdue │  │ Cancelled │
     └──────┘  └────┬────┘  └───────────┘
                    │
                    │ pay
                    │
                    ▼
               ┌──────┐
               │ Paid │
               └──────┘
```

**States:**

| State | Description |
|-------|-------------|
| Draft | Invoice created, not yet sent |
| Issued | Invoice sent to owner |
| Overdue | Past due date, unpaid |
| Paid | Payment received |
| Cancelled | Invoice cancelled |

**Transitions:**

| From | To | Trigger | Guard | Side Effects |
|------|----|---------|-------|--------------|
| Draft | Issued | `issue` | items.length > 0 | Send email, generate PDF |
| Draft | Cancelled | `cancel` | - | - |
| Issued | Paid | `pay` | payment received | Update account balance |
| Issued | Overdue | auto | dueDate passed | Send reminder |
| Issued | Cancelled | `cancel` | - | - |
| Overdue | Paid | `pay` | payment received | Update account, clear late fee |
| Overdue | Overdue | auto | every 7 days | Send reminder, apply late fee |

---

### 7. Listing State Machine

```
              ┌─────────┐
              │  Draft  │
              └────┬────┘
                   │
                   │ publish
                   │
                   ▼
              ┌────────┐
         ┌────│ Active │────────────────────┐
         │    └────┬───┘                    │
         │         │                        │
         │ withdraw│                        │ expire
         │         │                        │
         │    ┌────┴─────┬──────────┐       │
         │    │          │          │       │
         │    ▼          ▼          ▼       │
         │ ┌─────────┐ ┌────────┐ ┌──────┐  │
         │ │ Pending │ │  Sold  │ │Rented│  │
         │ └────┬────┘ └────────┘ └──────┘  │
         │      │                           │
         │      │ finalize                  │
         │      │                           │
         │      ▼                           │
         │ ┌────────┐   ┌─────────┐         │
         └►│Withdrawn│   │ Expired │◄────────┘
           └────────┘   └─────────┘
```

**States:**

| State | Description |
|-------|-------------|
| Draft | Listing created, not public |
| Active | Publicly visible |
| Pending | Offer accepted, sale in progress |
| Sold | Property sold (for sale listings) |
| Rented | Property rented (for rent listings) |
| Withdrawn | Listing withdrawn by owner |
| Expired | Listing expired |

**Transitions:**

| From | To | Trigger | Guard | Side Effects |
|------|----|---------|-------|--------------|
| Draft | Active | `publish` | photos.length >= 1 | Index for search |
| Active | Pending | `mark_pending` | - | Update search index |
| Active | Sold | `mark_sold` | transactionType = sale | Remove from search, archive |
| Active | Rented | `mark_rented` | transactionType = rent | Remove from search, archive |
| Active | Withdrawn | `withdraw` | - | Remove from search |
| Active | Expired | auto | expiresAt reached | Notify realtor |
| Pending | Active | `reactivate` | - | Re-index for search |
| Pending | Sold/Rented | `finalize` | - | Archive |
| Withdrawn | Active | `republish` | - | Re-index |
| Expired | Active | `renew` | - | Re-index, set new expiresAt |

---

### 8. Import Job State Machine

```
              ┌─────────┐
              │ Pending │
              └────┬────┘
                   │
                   │ start
                   │
                   ▼
            ┌────────────┐
            │ InProgress │
            └──────┬─────┘
                   │
         ┌─────────┼─────────┐
         │         │         │
         │ complete│  fail   │
         │         │         │
         ▼         │         ▼
    ┌───────────┐  │    ┌────────┐
    │ Completed │  │    │ Failed │
    └───────────┘  │    └────────┘
                   │
                   │ cancel
                   │
                   ▼
              ┌───────────┐
              │ Cancelled │
              └───────────┘
```

**States:**

| State | Description |
|-------|-------------|
| Pending | Import queued |
| InProgress | Import running |
| Completed | Import finished successfully |
| Failed | Import failed with errors |
| Cancelled | Import cancelled by user |

**Transitions:**

| From | To | Trigger | Guard | Side Effects |
|------|----|---------|-------|--------------|
| Pending | InProgress | `start` | worker available | Log start time |
| InProgress | Completed | `complete` | all items processed | Log completion, notify user |
| InProgress | Failed | `fail` | unrecoverable error | Log error, notify user |
| InProgress | Cancelled | `cancel` | user action | Stop processing, log cancellation |
| Pending | Cancelled | `cancel` | user action | Remove from queue |

---

## Summary

| Category | Count |
|----------|-------|
| **API Endpoints** | ~145 |
| **DTOs** | ~127 |
| **Validation Rules** | ~50 |
| **State Machines** | 8 |

This technical design document serves as the implementation contract for backend development, providing complete specifications for all API endpoints, data structures, validation rules, and entity lifecycles.
