# Sequence Diagrams & Flows

This document defines step-by-step interactions between system components, identifying synchronous vs. asynchronous behavior and external service calls.

## Table of Contents

1. [Legend](#legend)
2. [Authentication & Authorization Flows](#authentication--authorization-flows)
3. [Property Management Flows](#property-management-flows)
4. [Financial Flows](#financial-flows)
5. [Communication Flows](#communication-flows)
6. [Short-term Rental Flows](#short-term-rental-flows)
7. [Reality Portal Flows](#reality-portal-flows)
8. [Integration Flows](#integration-flows)
9. [AI/ML Flows](#aiml-flows)

---

## Legend

```
──────►  Synchronous call
- - - -► Asynchronous call (event/message queue)
◄──────  Synchronous response
◄- - - - Async response/callback

[S] = Synchronous operation
[A] = Async operation (event-driven)
[E] = External service call
[DB] = Database operation
[C] = Cache operation
```

### Component Abbreviations

| Abbreviation | Component |
|--------------|-----------|
| Client | Web/Mobile Application |
| API | API Gateway / Backend Server |
| Auth | Authentication Service |
| IAM | Identity & Access Management |
| DB | PostgreSQL Database |
| Cache | Redis Cache |
| Queue | Message Queue (RabbitMQ/SQS) |
| Notif | Notification Service |
| Email | Email Service (SendGrid/SES) |
| SMS | SMS Gateway |
| Push | Push Notification Service (FCM/APNs) |
| Storage | Object Storage (S3/MinIO) |
| AI | AI/ML Service |
| IoT | IoT Gateway |
| Ext | External Service (Airbnb, Booking, etc.) |

---

## Authentication & Authorization Flows

### 1. User Registration (UC-14.1)

```
┌────────┐     ┌─────┐     ┌──────┐     ┌────┐     ┌───────┐     ┌───────┐
│ Client │     │ API │     │ Auth │     │ DB │     │ Email │     │ Queue │
└───┬────┘     └──┬──┘     └──┬───┘     └─┬──┘     └───┬───┘     └───┬───┘
    │             │           │           │            │             │
    │ POST /auth/register     │           │            │             │
    │ {email, password,       │           │            │             │
    │  invitationCode}        │           │            │             │
    ├────────────────────────►│           │            │             │
    │             │ [S]       │           │            │             │
    │             │ Validate invitation   │            │             │
    │             ├──────────►│           │            │             │
    │             │           │ [DB]      │            │             │
    │             │           │ Check invitation code  │             │
    │             │           ├──────────►│            │             │
    │             │           │◄──────────┤            │             │
    │             │           │           │            │             │
    │             │           │ [S] Hash password      │             │
    │             │           ├───────────┤            │             │
    │             │           │           │            │             │
    │             │           │ [DB] Create user       │             │
    │             │           ├──────────►│            │             │
    │             │           │◄──────────┤            │             │
    │             │           │           │            │             │
    │             │           │ [DB] Assign role       │             │
    │             │           ├──────────►│            │             │
    │             │           │◄──────────┤            │             │
    │             │◄──────────┤           │            │             │
    │             │           │           │            │             │
    │             │ [A] Publish UserRegistered event   │             │
    │             ├───────────────────────────────────────────────►│
    │             │           │           │            │             │
    │ 201 Created │           │           │            │             │
    │ {userId, token}         │           │            │             │
    │◄────────────────────────┤           │            │             │
    │             │           │           │            │             │
    │             │           │           │    [A] Send welcome email│
    │             │           │           │            │◄────────────┤
    │             │           │           │            │             │

Flow Type: Synchronous with async side effects
External Calls: Email service (async)
Events Published: UserRegistered
```

### 2. User Login with MFA (UC-14.2, UC-14.10)

```
┌────────┐     ┌─────┐     ┌──────┐     ┌────┐     ┌───────┐     ┌─────┐
│ Client │     │ API │     │ Auth │     │ DB │     │ Cache │     │ SMS │
└───┬────┘     └──┬──┘     └──┬───┘     └─┬──┘     └───┬───┘     └──┬──┘
    │             │           │           │            │            │
    │ POST /auth/login        │           │            │            │
    │ {email, password}       │           │            │            │
    ├────────────────────────►│           │            │            │
    │             │ [S]       │           │            │            │
    │             │ Validate credentials  │            │            │
    │             ├──────────►│           │            │            │
    │             │           │ [DB]      │            │            │
    │             │           │ Get user by email      │            │
    │             │           ├──────────►│            │            │
    │             │           │◄──────────┤            │            │
    │             │           │           │            │            │
    │             │           │ [S] Verify password    │            │
    │             │           ├───────────┤            │            │
    │             │           │           │            │            │
    │             │           │ [S] Check MFA enabled  │            │
    │             │           ├───────────┤ (true)     │            │
    │             │◄──────────┤           │            │            │
    │             │           │           │            │            │
    │ 200 OK      │           │           │            │            │
    │ {mfaRequired: true,     │           │            │            │
    │  mfaToken}  │           │            │            │            │
    │◄────────────────────────┤           │            │            │
    │             │           │           │            │            │
    │ POST /auth/mfa/verify   │           │            │            │
    │ {mfaToken, code}        │           │            │            │
    ├────────────────────────►│           │            │            │
    │             │ [S]       │           │            │            │
    │             │ Verify MFA code       │            │            │
    │             ├──────────►│           │            │            │
    │             │           │ [C] Get MFA secret     │            │
    │             │           ├─────────────────────►│            │
    │             │           │◄─────────────────────┤            │
    │             │           │           │            │            │
    │             │           │ [S] Validate TOTP     │            │
    │             │           ├───────────┤            │            │
    │             │           │           │            │            │
    │             │           │ [S] Generate JWT tokens│            │
    │             │           ├───────────┤            │            │
    │             │           │           │            │            │
    │             │           │ [DB] Create session    │            │
    │             │           ├──────────►│            │            │
    │             │           │◄──────────┤            │            │
    │             │           │           │            │            │
    │             │           │ [C] Cache session      │            │
    │             │           ├─────────────────────►│            │
    │             │◄──────────┤           │            │            │
    │             │           │           │            │            │
    │ 200 OK      │           │           │            │            │
    │ {accessToken, refreshToken, user}   │            │            │
    │◄────────────────────────┤           │            │            │

Flow Type: Synchronous
External Calls: None (TOTP validated locally)
Cache Operations: Session caching, MFA secret retrieval
```

### 3. OAuth/SSO Login (UC-14.11, UC-47.3-5)

```
┌────────┐     ┌─────┐     ┌──────┐     ┌────────┐     ┌────┐
│ Client │     │ API │     │ Auth │     │ OAuth  │     │ DB │
│        │     │     │     │      │     │Provider│     │    │
└───┬────┘     └──┬──┘     └──┬───┘     └───┬────┘     └─┬──┘
    │             │           │             │            │
    │ GET /auth/oauth/{provider}            │            │
    ├────────────────────────►│             │            │
    │             │ [S]       │             │            │
    │             │ Build OAuth URL         │            │
    │             ├──────────►│             │            │
    │             │◄──────────┤             │            │
    │             │           │             │            │
    │ 302 Redirect to OAuth provider        │            │
    │◄────────────────────────┤             │            │
    │             │           │             │            │
    │ User authenticates with provider      │            │
    ├───────────────────────────────────────►            │
    │             │           │             │            │
    │ Redirect to callback with code        │            │
    │◄───────────────────────────────────────            │
    │             │           │             │            │
    │ GET /auth/oauth/callback?code=xxx     │            │
    ├────────────────────────►│             │            │
    │             │ [S]       │             │            │
    │             │ Exchange code for tokens│            │
    │             ├──────────►│             │            │
    │             │           │ [E] POST /token         │
    │             │           ├────────────►│            │
    │             │           │◄────────────┤            │
    │             │           │             │            │
    │             │           │ [E] GET /userinfo       │
    │             │           ├────────────►│            │
    │             │           │◄────────────┤            │
    │             │           │             │            │
    │             │           │ [DB] Find or create user│
    │             │           ├─────────────────────────►
    │             │           │◄─────────────────────────
    │             │           │             │            │
    │             │           │ [DB] Link social account│
    │             │           ├─────────────────────────►
    │             │           │◄─────────────────────────
    │             │           │             │            │
    │             │           │ [S] Generate JWT tokens │
    │             │           ├───────────┤             │
    │             │◄──────────┤             │            │
    │             │           │             │            │
    │ 302 Redirect to app with tokens       │            │
    │◄────────────────────────┤             │            │

Flow Type: Synchronous with external OAuth flow
External Calls: OAuth Provider (Google, Apple, Facebook)
```

### 4. Token Refresh

```
┌────────┐     ┌─────┐     ┌──────┐     ┌───────┐     ┌────┐
│ Client │     │ API │     │ Auth │     │ Cache │     │ DB │
└───┬────┘     └──┬──┘     └──┬───┘     └───┬───┘     └─┬──┘
    │             │           │             │           │
    │ POST /auth/refresh      │             │           │
    │ {refreshToken}          │             │           │
    ├────────────────────────►│             │           │
    │             │ [S]       │             │           │
    │             │ Validate refresh token  │           │
    │             ├──────────►│             │           │
    │             │           │ [S] Verify JWT signature│
    │             │           ├───────────┤             │
    │             │           │             │           │
    │             │           │ [C] Check token not revoked
    │             │           ├────────────►│           │
    │             │           │◄────────────┤           │
    │             │           │             │           │
    │             │           │ [DB] Get user & roles   │
    │             │           ├──────────────────────►│
    │             │           │◄──────────────────────┤
    │             │           │             │           │
    │             │           │ [S] Generate new tokens │
    │             │           ├───────────┤             │
    │             │           │             │           │
    │             │           │ [C] Update session      │
    │             │           ├────────────►│           │
    │             │◄──────────┤             │           │
    │             │           │             │           │
    │ 200 OK      │           │             │           │
    │ {accessToken, refreshToken}           │           │
    │◄────────────────────────┤             │           │

Flow Type: Synchronous
Cache Operations: Token revocation check, session update
```

### 5. Delegation Flow (UC-28.1-4)

```
┌───────┐     ┌─────┐     ┌─────┐     ┌────┐     ┌───────┐     ┌───────┐
│ Owner │     │ API │     │ IAM │     │ DB │     │ Queue │     │Delegate│
└───┬───┘     └──┬──┘     └──┬──┘     └─┬──┘     └───┬───┘     └───┬───┘
    │            │           │          │            │             │
    │ POST /delegations      │          │            │             │
    │ {delegateEmail, rights,│          │            │             │
    │  unitId, expiresAt}    │          │            │             │
    ├───────────────────────►│          │            │             │
    │            │ [S]       │          │            │             │
    │            │ Validate owner rights │            │             │
    │            ├──────────►│          │            │             │
    │            │           │ [DB] Check ownership   │             │
    │            │           ├─────────►│            │             │
    │            │           │◄─────────┤            │             │
    │            │◄──────────┤          │            │             │
    │            │           │          │            │             │
    │            │ [DB] Find/invite delegate          │             │
    │            ├──────────────────────►            │             │
    │            │◄──────────────────────            │             │
    │            │           │          │            │             │
    │            │ [DB] Create delegation (pending)  │             │
    │            ├──────────────────────►            │             │
    │            │◄──────────────────────            │             │
    │            │           │          │            │             │
    │            │ [A] Publish DelegationCreated     │             │
    │            ├───────────────────────────────────►             │
    │            │           │          │            │             │
    │ 201 Created│           │          │            │             │
    │ {delegationId}         │          │            │             │
    │◄───────────────────────┤          │            │             │
    │            │           │          │            │             │
    │            │           │          │  [A] Send invitation email│
    │            │           │          │            ├────────────►│
    │            │           │          │            │             │
    │            │           │          │            │   Click link│
    │            │           │          │            │◄────────────┤
    │            │           │          │            │             │
    │            │       POST /delegations/{id}/accept             │
    │            │◄────────────────────────────────────────────────┤
    │            │           │          │            │             │
    │            │ [DB] Update delegation (accepted) │             │
    │            ├──────────────────────►            │             │
    │            │◄──────────────────────            │             │
    │            │           │          │            │             │
    │            │ [A] Publish DelegationAccepted    │             │
    │            ├───────────────────────────────────►             │
    │            │           │          │            │             │
    │            │ 200 OK    │          │            │             │
    │            ├─────────────────────────────────────────────────►
    │            │           │          │            │             │

Flow Type: Synchronous with async notifications
Events Published: DelegationCreated, DelegationAccepted
```

---

## Property Management Flows

### 6. Report Fault (UC-03.1)

```
┌────────┐     ┌─────┐     ┌───────┐     ┌────┐     ┌─────────┐     ┌───────┐     ┌──────┐
│ Client │     │ API │     │Fault  │     │ DB │     │ Storage │     │ Queue │     │ AI   │
│        │     │     │     │Service│     │    │     │         │     │       │     │      │
└───┬────┘     └──┬──┘     └───┬───┘     └─┬──┘     └────┬────┘     └───┬───┘     └──┬───┘
    │             │            │           │             │              │            │
    │ POST /faults             │           │             │              │            │
    │ {title, description,     │           │             │              │            │
    │  category, photos[]}     │           │             │              │            │
    ├────────────────────────►│            │             │              │            │
    │             │ [S]        │           │             │              │            │
    │             │ Create fault           │             │              │            │
    │             ├───────────►│           │             │              │            │
    │             │            │           │             │              │            │
    │             │            │ [E] Upload photos       │              │            │
    │             │            ├──────────────────────►│              │            │
    │             │            │◄──────────────────────┤              │            │
    │             │            │           │             │              │            │
    │             │            │ [DB] Save fault         │              │            │
    │             │            ├──────────►│             │              │            │
    │             │            │◄──────────┤             │              │            │
    │             │◄───────────┤           │             │              │            │
    │             │            │           │             │              │            │
    │             │ [A] Publish FaultReported           │              │            │
    │             ├────────────────────────────────────────────────────►            │
    │             │            │           │             │              │            │
    │ 201 Created │            │           │             │              │            │
    │ {faultId}   │            │           │             │              │            │
    │◄────────────────────────┤            │             │              │            │
    │             │            │           │             │              │            │
    │             │            │           │             │     [A] AI Categorization │
    │             │            │           │             │              ├───────────►│
    │             │            │           │             │              │            │
    │             │            │           │             │    [A] Analyze image      │
    │             │            │           │             │              ├───────────►│
    │             │            │           │             │              │◄───────────┤
    │             │            │           │             │              │            │
    │             │            │           │     [A] Update fault with AI tags       │
    │             │            │◄──────────────────────────────────────┤            │
    │             │            │           │             │              │            │
    │             │            │      [A] Auto-assign to technician    │            │
    │             │            ├──────────────────────────────────────►│            │
    │             │            │           │             │              │            │
    │             │            │           │             │  [A] Notify manager       │
    │             │            │           │             │              ├───────────►│
    │             │            │           │             │              │            │

Flow Type: Synchronous creation, async processing
External Calls: Object Storage (photo upload)
Events Published: FaultReported
Async Processing: AI categorization, image analysis, auto-assignment, notifications
```

### 7. Fault Resolution Workflow (UC-03.6-10)

```
┌─────────┐     ┌─────┐     ┌───────┐     ┌────┐     ┌───────┐     ┌──────┐     ┌────────┐
│ Manager │     │ API │     │Fault  │     │ DB │     │ Queue │     │ Push │     │Reporter│
│         │     │     │     │Service│     │    │     │       │     │      │     │        │
└────┬────┘     └──┬──┘     └───┬───┘     └─┬──┘     └───┬───┘     └──┬───┘     └───┬────┘
     │             │            │           │            │            │             │
     │ PATCH /faults/{id}       │           │            │            │             │
     │ {assigneeId}             │           │            │            │             │
     ├────────────────────────►│            │            │            │             │
     │             │ [S]        │           │            │            │             │
     │             │ Assign fault           │            │            │             │
     │             ├───────────►│           │            │            │             │
     │             │            │ [DB] Update assignment │            │             │
     │             │            ├──────────►│            │            │             │
     │             │            │◄──────────┤            │            │             │
     │             │            │           │            │            │             │
     │             │            │ [A] Publish FaultAssigned           │             │
     │             │            ├───────────────────────►│            │             │
     │             │◄───────────┤           │            │            │             │
     │ 200 OK      │            │           │            │            │             │
     │◄────────────────────────┤            │            │            │             │
     │             │            │           │            │            │             │
     │             │            │           │   [A] Notify technician │             │
     │             │            │           │            ├───────────►│             │
     │             │            │           │            │            │             │
     │             │            │           │            │            │             │
     │ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ (Technician works on issue) ─ ─ ─ ─ ─ ─ ─ ─             │
     │             │            │           │            │            │             │
     │ PATCH /faults/{id}       │           │            │            │             │
     │ {status: "resolved",     │           │            │            │             │
     │  resolution: "Fixed..."}│            │            │            │             │
     ├────────────────────────►│            │            │            │             │
     │             │ [S]        │           │            │            │             │
     │             │ Resolve fault          │            │            │             │
     │             ├───────────►│           │            │            │             │
     │             │            │ [DB] Update status     │            │             │
     │             │            ├──────────►│            │            │             │
     │             │            │◄──────────┤            │            │             │
     │             │            │           │            │            │             │
     │             │            │ [DB] Add status history│            │             │
     │             │            ├──────────►│            │            │             │
     │             │            │◄──────────┤            │            │             │
     │             │            │           │            │            │             │
     │             │            │ [A] Publish FaultResolved           │             │
     │             │            ├───────────────────────►│            │             │
     │             │◄───────────┤           │            │            │             │
     │ 200 OK      │            │           │            │            │             │
     │◄────────────────────────┤            │            │            │             │
     │             │            │           │            │            │             │
     │             │            │           │  [A] Notify reporter    │             │
     │             │            │           │            ├────────────────────────►│
     │             │            │           │            │            │             │

Flow Type: Synchronous updates with async notifications
Events Published: FaultAssigned, FaultResolved
Async Processing: Push notifications to technician and reporter
```

### 8. Create and Cast Vote (UC-04.4, UC-04.7)

```
┌─────────┐     ┌─────┐     ┌───────┐     ┌────┐     ┌───────┐     ┌───────┐     ┌───────┐
│ Manager │     │ API │     │ Vote  │     │ DB │     │ Queue │     │ Push  │     │ Owner │
│         │     │     │     │Service│     │    │     │       │     │       │     │       │
└────┬────┘     └──┬──┘     └───┬───┘     └─┬──┘     └───┬───┘     └───┬───┘     └───┬───┘
     │             │            │           │            │             │             │
     │ POST /votes │            │           │            │             │             │
     │ {title, options[],       │           │            │             │             │
     │  settings, endDate}      │           │            │             │             │
     ├────────────────────────►│            │           │            │             │
     │             │ [S]        │           │            │             │             │
     │             │ Create vote            │            │             │             │
     │             ├───────────►│           │            │             │             │
     │             │            │ [DB] Create vote       │             │             │
     │             │            ├──────────►│            │             │             │
     │             │            │◄──────────┤            │             │             │
     │             │            │           │            │             │             │
     │             │            │ [DB] Get eligible voters             │             │
     │             │            ├──────────►│            │             │             │
     │             │            │◄──────────┤            │             │             │
     │             │            │           │            │             │             │
     │             │            │ [A] Publish VoteCreated│             │             │
     │             │            ├───────────────────────►│             │             │
     │             │◄───────────┤           │            │             │             │
     │ 201 Created │            │           │            │             │             │
     │ {voteId}    │            │           │            │             │             │
     │◄────────────────────────┤            │            │             │             │
     │             │            │           │            │             │             │
     │             │            │           │   [A] Notify all owners  │             │
     │             │            │           │            ├────────────►│             │
     │             │            │           │            │             ├────────────►│
     │             │            │           │            │             │             │
     │             │            │           │            │             │             │
     │ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ (Owner casts vote) ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│
     │             │            │           │            │             │             │
     │             │            │           │            │             │ POST /votes/{id}/ballots
     │             │            │           │            │             │ {optionId}  │
     │             │◄───────────────────────────────────────────────────────────────┤
     │             │ [S]        │           │            │             │             │
     │             │ Cast ballot            │            │             │             │
     │             ├───────────►│           │            │             │             │
     │             │            │ [S] Verify eligibility │             │             │
     │             │            ├──────────►│            │             │             │
     │             │            │◄──────────┤            │             │             │
     │             │            │           │            │             │             │
     │             │            │ [S] Check not already voted          │             │
     │             │            ├──────────►│            │             │             │
     │             │            │◄──────────┤            │             │             │
     │             │            │           │            │             │             │
     │             │            │ [S] Calculate vote weight            │             │
     │             │            ├──────────►│ (ownership share)        │             │
     │             │            │◄──────────┤            │             │             │
     │             │            │           │            │             │             │
     │             │            │ [DB] Create ballot     │             │             │
     │             │            ├──────────►│            │             │             │
     │             │            │◄──────────┤            │             │             │
     │             │            │           │            │             │             │
     │             │            │ [A] Publish BallotCast │             │             │
     │             │            ├───────────────────────►│             │             │
     │             │◄───────────┤           │            │             │             │
     │             │            │           │            │             │             │
     │             │ 201 Created│           │            │             │             │
     │             ├─────────────────────────────────────────────────────────────────►
     │             │            │           │            │             │             │
     │             │            │           │  [A] Broadcast live results (WebSocket)│
     │             │            │           │            ├─────────────────────────►│

Flow Type: Synchronous with async notifications and real-time updates
Events Published: VoteCreated, BallotCast
Real-time: WebSocket broadcast for live voting results
```

### 9. Submit Meter Reading with OCR (UC-11.1, UC-20.11)

```
┌────────┐     ┌─────┐     ┌───────┐     ┌─────────┐     ┌────┐     ┌──────┐     ┌───────┐
│ Owner  │     │ API │     │Meter  │     │ Storage │     │ DB │     │ AI   │     │ Queue │
│        │     │     │     │Service│     │         │     │    │     │      │     │       │
└───┬────┘     └──┬──┘     └───┬───┘     └────┬────┘     └─┬──┘     └──┬───┘     └───┬───┘
    │             │            │              │            │           │             │
    │ POST /meter-readings     │              │            │           │             │
    │ {meterId, value,         │              │            │           │             │
    │  photo: base64}          │              │            │           │             │
    ├────────────────────────►│              │            │           │             │
    │             │ [S]        │              │            │           │             │
    │             │ Submit reading           │            │           │             │
    │             ├───────────►│              │            │           │             │
    │             │            │              │            │           │             │
    │             │            │ [E] Upload photo         │           │             │
    │             │            ├─────────────►│            │           │             │
    │             │            │◄─────────────┤            │           │             │
    │             │            │              │            │           │             │
    │             │            │ [DB] Save reading (pending)           │             │
    │             │            ├──────────────────────────►│           │             │
    │             │            │◄──────────────────────────┤           │             │
    │             │            │              │            │           │             │
    │             │            │ [A] Request OCR          │           │             │
    │             │            ├──────────────────────────────────────────────────►│
    │             │◄───────────┤              │            │           │             │
    │             │            │              │            │           │             │
    │ 201 Created │            │              │            │           │             │
    │ {readingId, │            │              │            │           │             │
    │  status: "pending"}      │              │            │           │             │
    │◄────────────────────────┤              │            │           │             │
    │             │            │              │            │           │             │
    │             │            │              │            │   [A] OCR processing   │
    │             │            │              │            │           │◄────────────┤
    │             │            │              │            │           │             │
    │             │            │              │     [E] Download image │             │
    │             │            │              │◄──────────────────────┤             │
    │             │            │              ├──────────────────────►│             │
    │             │            │              │            │           │             │
    │             │            │              │            │   [S] Extract value    │
    │             │            │              │            │           ├───────────┤│
    │             │            │              │            │           │             │
    │             │            │      [A] Update with OCR value        │             │
    │             │            │◄──────────────────────────────────────┤             │
    │             │            │              │            │           │             │
    │             │            │ [S] Compare OCR vs submitted value    │             │
    │             │            ├───────────┤              │            │             │
    │             │            │              │            │           │             │
    │             │            │ [A] Flag if mismatch > threshold      │             │
    │             │            ├──────────────────────────────────────────────────►│
    │             │            │              │            │           │             │

Flow Type: Synchronous submission with async OCR processing
External Calls: Object Storage, AI Service (OCR)
Async Processing: OCR extraction, value comparison, anomaly flagging
```

---

## Financial Flows

### 10. Generate and Pay Invoice (UC-16.3-4)

```
┌─────────┐     ┌─────┐     ┌─────────┐     ┌────┐     ┌─────────┐     ┌───────┐     ┌───────┐
│ Manager │     │ API │     │Financial│     │ DB │     │ Payment │     │ Queue │     │ Owner │
│         │     │     │     │ Service │     │    │     │ Gateway │     │       │     │       │
└────┬────┘     └──┬──┘     └────┬────┘     └─┬──┘     └────┬────┘     └───┬───┘     └───┬───┘
     │             │             │            │             │              │             │
     │ POST /invoices            │            │             │              │             │
     │ {unitId, items[],         │            │             │              │             │
     │  dueDate}                 │            │             │              │             │
     ├────────────────────────►│             │            │             │              │
     │             │ [S]         │            │             │              │             │
     │             │ Generate invoice        │             │              │             │
     │             ├────────────►│            │             │              │             │
     │             │             │ [DB] Calculate amounts  │              │             │
     │             │             ├───────────►│             │              │             │
     │             │             │◄───────────┤             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [DB] Create invoice      │              │             │
     │             │             ├───────────►│             │              │             │
     │             │             │◄───────────┤             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [S] Generate PDF         │              │             │
     │             │             ├───────────┤             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [A] Publish InvoiceGenerated           │             │
     │             │             ├─────────────────────────────────────────►             │
     │             │◄────────────┤            │             │              │             │
     │ 201 Created │             │            │             │              │             │
     │ {invoiceId} │             │            │             │              │             │
     │◄────────────────────────┤             │            │             │              │
     │             │             │            │             │              │             │
     │             │             │            │             │  [A] Email invoice        │
     │             │             │            │             │              ├────────────►│
     │             │             │            │             │              │             │
     │             │             │            │             │              │             │
     │ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ (Owner makes payment) ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ │
     │             │             │            │             │              │             │
     │             │             │            │             │              │ POST /payments
     │             │             │            │             │              │ {invoiceId, │
     │             │             │            │             │              │  paymentMethod}
     │             │◄───────────────────────────────────────────────────────────────────┤
     │             │ [S]         │            │             │              │             │
     │             │ Process payment         │             │              │             │
     │             ├────────────►│            │             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [E] Create payment intent│             │              │
     │             │             ├────────────────────────►│              │             │
     │             │             │◄────────────────────────┤              │             │
     │             │             │            │             │              │             │
     │             │ 200 OK      │            │             │              │             │
     │             │ {clientSecret}           │             │              │             │
     │             ├─────────────────────────────────────────────────────────────────────►
     │             │             │            │             │              │             │
     │             │             │            │             │              │ Confirm payment
     │             │             │            │             │              │ (client-side)
     │             │             │            │             │              │◄────────────┤
     │             │             │            │             │              │             │
     │             │             │     [E] Webhook: payment.succeeded     │             │
     │             │             │◄────────────────────────┤              │             │
     │             │             │            │             │              │             │
     │             │             │ [DB] Mark invoice paid  │              │             │
     │             │             ├───────────►│             │              │             │
     │             │             │◄───────────┤             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [DB] Update account balance            │             │
     │             │             ├───────────►│             │              │             │
     │             │             │◄───────────┤             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [A] Publish PaymentReceived            │             │
     │             │             ├─────────────────────────────────────────►             │
     │             │             │            │             │              │             │
     │             │             │            │             │  [A] Send receipt         │
     │             │             │            │             │              ├────────────►│

Flow Type: Synchronous with external payment gateway and webhooks
External Calls: Payment Gateway (Stripe, etc.)
Events Published: InvoiceGenerated, PaymentReceived
Webhook Handling: Payment confirmation from gateway
```

### 11. Automated Payment Reminder (UC-26.2)

```
┌──────────┐     ┌───────┐     ┌─────────┐     ┌────┐     ┌───────┐     ┌───────┐
│Scheduler │     │ Queue │     │Financial│     │ DB │     │ Email │     │ Push  │
│ (Cron)   │     │       │     │ Service │     │    │     │       │     │       │
└────┬─────┘     └───┬───┘     └────┬────┘     └─┬──┘     └───┬───┘     └───┬───┘
     │               │              │            │            │             │
     │ [A] Trigger payment reminder job          │            │             │
     ├──────────────►│              │            │            │             │
     │               │              │            │            │             │
     │               │ [A] Process job           │            │            │
     │               ├─────────────►│            │            │             │
     │               │              │            │            │             │
     │               │              │ [DB] Query overdue invoices          │
     │               │              ├───────────►│            │             │
     │               │              │◄───────────┤            │             │
     │               │              │            │            │             │
     │               │              │ For each overdue invoice:            │
     │               │              ├────────────┤            │             │
     │               │              │            │            │             │
     │               │              │ [DB] Get owner preferences           │
     │               │              ├───────────►│            │             │
     │               │              │◄───────────┤            │             │
     │               │              │            │            │             │
     │               │              │ [E] Send email reminder │             │
     │               │              ├────────────────────────►│             │
     │               │              │            │            │             │
     │               │              │ [E] Send push notification           │
     │               │              ├──────────────────────────────────────►│
     │               │              │            │            │             │
     │               │              │ [DB] Log reminder sent │             │
     │               │              ├───────────►│            │             │
     │               │              │◄───────────┤            │             │
     │               │              │            │            │             │
     │               │              │ [A] Publish PaymentOverdue           │
     │               │              ├─────────────────────────►            │
     │               │◄─────────────┤            │            │             │
     │               │              │            │            │             │

Flow Type: Fully asynchronous (scheduled job)
Trigger: Cron scheduler (daily)
External Calls: Email Service, Push Notification Service
Events Published: PaymentOverdue
```

---

## Communication Flows

### 12. Send Message with Real-time Delivery (UC-05.5)

```
┌────────┐     ┌─────┐     ┌─────────┐     ┌────┐     ┌─────────┐     ┌───────────┐     ┌──────────┐
│ Sender │     │ API │     │ Message │     │ DB │     │WebSocket│     │ Recipient │     │   Push   │
│        │     │     │     │ Service │     │    │     │ Server  │     │ (Online)  │     │ (Offline)│
└───┬────┘     └──┬──┘     └────┬────┘     └─┬──┘     └────┬────┘     └─────┬─────┘     └────┬─────┘
    │             │             │            │             │                │                │
    │ POST /conversations/{id}/messages      │             │                │                │
    │ {content, attachments[]}  │            │             │                │                │
    ├────────────────────────►│             │            │             │                │
    │             │ [S]         │            │             │                │                │
    │             │ Send message            │             │                │                │
    │             ├────────────►│            │             │                │                │
    │             │             │            │             │                │                │
    │             │             │ [DB] Save message        │                │                │
    │             │             ├───────────►│             │                │                │
    │             │             │◄───────────┤             │                │                │
    │             │             │            │             │                │                │
    │             │             │ [DB] Update conversation │                │                │
    │             │             ├───────────►│             │                │                │
    │             │             │◄───────────┤             │                │                │
    │             │             │            │             │                │                │
    │             │             │ [S] Check recipient online               │                │
    │             │             ├────────────────────────►│                │                │
    │             │             │◄────────────────────────┤ (online)       │                │
    │             │             │            │             │                │                │
    │             │             │ [S] Push via WebSocket  │                │                │
    │             │             ├────────────────────────►│                │                │
    │             │             │            │             │ New message    │                │
    │             │             │            │             ├───────────────►│                │
    │             │◄────────────┤            │             │                │                │
    │             │             │            │             │                │                │
    │ 201 Created │             │            │             │                │                │
    │ {messageId} │             │            │             │                │                │
    │◄────────────────────────┤             │            │             │                │
    │             │             │            │             │                │                │
    │             │             │            │             │                │                │
    │ ─ ─ ─ ─ ─ ─ (If recipient offline) ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─               │
    │             │             │            │             │                │                │
    │             │             │ [S] Check recipient online               │                │
    │             │             ├────────────────────────►│                │                │
    │             │             │◄────────────────────────┤ (offline)      │                │
    │             │             │            │             │                │                │
    │             │             │ [E] Send push notification               │                │
    │             │             ├─────────────────────────────────────────────────────────►│
    │             │             │            │             │                │                │

Flow Type: Synchronous with real-time WebSocket delivery
Real-time: WebSocket for online recipients
Fallback: Push notifications for offline recipients
```

### 13. Typing Indicator (UC-19.3)

```
┌────────┐     ┌───────────┐     ┌───────────┐     ┌───────────┐
│ Sender │     │ WebSocket │     │  Presence │     │ Recipient │
│        │     │  Server   │     │  Service  │     │           │
└───┬────┘     └─────┬─────┘     └─────┬─────┘     └─────┬─────┘
    │                │                 │                 │
    │ WS: typing_start                 │                 │
    │ {conversationId}                 │                 │
    ├───────────────►│                 │                 │
    │                │                 │                 │
    │                │ [S] Check recipient presence      │
    │                ├────────────────►│                 │
    │                │◄────────────────┤ (online)        │
    │                │                 │                 │
    │                │ WS: typing_indicator              │
    │                │ {userId, conversationId}          │
    │                ├────────────────────────────────►│
    │                │                 │                 │
    │ ─ ─ ─ ─ ─ (User stops typing or 3s timeout) ─ ─ ─│
    │                │                 │                 │
    │ WS: typing_stop│                 │                 │
    │ {conversationId}                 │                 │
    ├───────────────►│                 │                 │
    │                │                 │                 │
    │                │ WS: typing_stopped                │
    │                ├────────────────────────────────►│
    │                │                 │                 │

Flow Type: Real-time WebSocket only (fire-and-forget)
No persistence: Ephemeral indicator, not stored
```

---

## Short-term Rental Flows

### 14. Airbnb Reservation Sync (UC-29.1-5)

```
┌──────────┐     ┌───────┐     ┌─────────┐     ┌────┐     ┌─────────┐     ┌───────┐     ┌──────┐
│Scheduler │     │ Queue │     │ Rental  │     │ DB │     │ Airbnb  │     │ Guest │     │Police│
│          │     │       │     │ Service │     │    │     │   API   │     │ Reg   │     │Portal│
└────┬─────┘     └───┬───┘     └────┬────┘     └─┬──┘     └────┬────┘     └───┬───┘     └──┬───┘
     │               │              │            │             │              │            │
     │ [A] Trigger sync job         │            │             │              │            │
     ├──────────────►│              │            │             │              │            │
     │               │              │            │             │              │            │
     │               │ [A] Process sync         │             │              │            │
     │               ├─────────────►│            │             │              │            │
     │               │              │            │             │              │            │
     │               │              │ [DB] Get connected accounts           │            │
     │               │              ├───────────►│             │              │            │
     │               │              │◄───────────┤             │              │            │
     │               │              │            │             │              │            │
     │               │              │ [E] Fetch reservations  │              │            │
     │               │              ├────────────────────────►│              │            │
     │               │              │◄────────────────────────┤              │            │
     │               │              │            │             │              │            │
     │               │              │ For each new reservation:             │            │
     │               │              ├────────────┤             │              │            │
     │               │              │            │             │              │            │
     │               │              │ [DB] Create reservation │              │            │
     │               │              ├───────────►│             │              │            │
     │               │              │◄───────────┤             │              │            │
     │               │              │            │             │              │            │
     │               │              │ [S] Generate access code│              │            │
     │               │              ├───────────┤             │              │            │
     │               │              │            │             │              │            │
     │               │              │ [A] Create guest registration          │            │
     │               │              ├──────────────────────────────────────►│            │
     │               │              │            │             │              │            │
     │               │              │            │             │   [E] Submit to police   │
     │               │              │            │             │              ├───────────►│
     │               │              │            │             │              │◄───────────┤
     │               │              │            │             │              │            │
     │               │              │ [A] Publish ReservationCreated         │            │
     │               │              ├─────────────────────────►             │            │
     │               │              │            │             │              │            │
     │               │              │            │  [A] Send welcome message to guest     │
     │               │              │            │             ├─────────────►            │
     │               │◄─────────────┤            │             │              │            │

Flow Type: Async scheduled job with external API calls
External Calls: Airbnb API, Police Registration Portal
Trigger: Scheduled (every 15 minutes)
Events Published: ReservationCreated
```

### 15. Guest Check-in Flow (UC-29.9)

```
┌───────┐     ┌─────┐     ┌─────────┐     ┌────┐     ┌──────────┐     ┌───────┐     ┌─────────┐
│ Guest │     │ API │     │ Rental  │     │ DB │     │Smart Lock│     │ Queue │     │ Manager │
│       │     │     │     │ Service │     │    │     │   API    │     │       │     │         │
└───┬───┘     └──┬──┘     └────┬────┘     └─┬──┘     └────┬─────┘     └───┬───┘     └────┬────┘
    │            │             │            │             │               │              │
    │ POST /check-in           │            │             │               │              │
    │ {reservationId,          │            │             │               │              │
    │  accessCode}             │            │             │               │              │
    ├───────────────────────►│             │            │             │               │
    │            │ [S]         │            │             │               │              │
    │            │ Process check-in        │             │               │              │
    │            ├────────────►│            │             │               │              │
    │            │             │            │             │               │              │
    │            │             │ [DB] Validate reservation│               │              │
    │            │             ├───────────►│             │               │              │
    │            │             │◄───────────┤             │               │              │
    │            │             │            │             │               │              │
    │            │             │ [S] Verify access code   │               │              │
    │            │             ├───────────┤             │               │              │
    │            │             │            │             │               │              │
    │            │             │ [E] Activate smart lock  │               │              │
    │            │             ├────────────────────────►│               │              │
    │            │             │◄────────────────────────┤               │              │
    │            │             │            │             │               │              │
    │            │             │ [DB] Update check-in time│               │              │
    │            │             ├───────────►│             │               │              │
    │            │             │◄───────────┤             │               │              │
    │            │             │            │             │               │              │
    │            │             │ [A] Publish GuestCheckedIn               │              │
    │            │             ├──────────────────────────────────────────►              │
    │            │◄────────────┤            │             │               │              │
    │            │             │            │             │               │              │
    │ 200 OK     │             │            │             │               │              │
    │ {lockCode, │             │            │             │               │              │
    │  wifiInfo} │             │            │             │               │              │
    │◄───────────────────────┤             │            │             │               │
    │            │             │            │             │               │              │
    │            │             │            │             │   [A] Notify manager         │
    │            │             │            │             │               ├─────────────►│

Flow Type: Synchronous with external IoT call
External Calls: Smart Lock API (IoT)
Events Published: GuestCheckedIn
```

---

## Reality Portal Flows

### 16. Create Listing with Photos (UC-51.4, UC-51.6)

```
┌─────────┐     ┌─────┐     ┌─────────┐     ┌────┐     ┌─────────┐     ┌───────┐     ┌──────┐
│ Realtor │     │ API │     │ Listing │     │ DB │     │ Storage │     │ Queue │     │ AI   │
│         │     │     │     │ Service │     │    │     │         │     │       │     │      │
└────┬────┘     └──┬──┘     └────┬────┘     └─┬──┘     └────┬────┘     └───┬───┘     └──┬───┘
     │             │             │            │             │              │            │
     │ POST /listings            │            │             │              │            │
     │ {property, description,   │            │             │              │            │
     │  photos[], pricing}       │            │             │              │            │
     ├────────────────────────►│             │            │             │              │
     │             │ [S]         │            │             │              │            │
     │             │ Create listing          │             │              │            │
     │             ├────────────►│            │             │              │            │
     │             │             │            │             │              │            │
     │             │             │ [E] Upload photos       │              │            │
     │             │             ├────────────────────────►│              │            │
     │             │             │◄────────────────────────┤              │            │
     │             │             │            │             │              │            │
     │             │             │ [E] Generate thumbnails │              │            │
     │             │             ├────────────────────────►│              │            │
     │             │             │◄────────────────────────┤              │            │
     │             │             │            │             │              │            │
     │             │             │ [DB] Create listing     │              │            │
     │             │             ├───────────►│             │              │            │
     │             │             │◄───────────┤             │              │            │
     │             │             │            │             │              │            │
     │             │             │ [A] Queue AI processing │              │            │
     │             │             ├─────────────────────────────────────────►            │
     │             │◄────────────┤            │             │              │            │
     │             │             │            │             │              │            │
     │ 201 Created │             │            │             │              │            │
     │ {listingId} │             │            │             │              │            │
     │◄────────────────────────┤             │            │             │              │
     │             │             │            │             │              │            │
     │             │             │            │             │     [A] AI processing     │
     │             │             │            │             │              ├────────────►│
     │             │             │            │             │              │             │
     │             │             │            │             │     [A] Suggest tags      │
     │             │             │            │             │              │◄────────────┤
     │             │             │            │             │              │             │
     │             │             │            │             │     [A] Detect features   │
     │             │             │            │             │              │◄────────────┤
     │             │             │            │             │              │             │
     │             │             │      [A] Update listing with AI data   │             │
     │             │             │◄────────────────────────────────────────┤             │

Flow Type: Synchronous creation with async AI enrichment
External Calls: Object Storage (photos)
Async Processing: AI tag suggestions, feature detection
```

### 17. Search Listings with Alerts (UC-45.1-7)

```
┌────────┐     ┌─────┐     ┌─────────┐     ┌────┐     ┌───────┐     ┌───────┐     ┌───────┐
│ Portal │     │ API │     │ Search  │     │ DB │     │ Cache │     │ Queue │     │ Email │
│ User   │     │     │     │ Service │     │    │     │       │     │       │     │       │
└───┬────┘     └──┬──┘     └────┬────┘     └─┬──┘     └───┬───┘     └───┬───┘     └───┬───┘
    │             │             │            │            │             │             │
    │ GET /listings?location=...&price=...   │            │             │             │
    ├────────────────────────►│             │            │             │             │
    │             │ [S]         │            │             │             │             │
    │             │ Search listings         │             │             │             │
    │             ├────────────►│            │             │             │             │
    │             │             │            │             │             │             │
    │             │             │ [C] Check cache         │             │             │
    │             │             ├────────────────────────►│             │             │
    │             │             │◄────────────────────────┤ (miss)      │             │
    │             │             │            │             │             │             │
    │             │             │ [DB] Execute search      │             │             │
    │             │             ├───────────►│             │             │             │
    │             │             │◄───────────┤             │             │             │
    │             │             │            │             │             │             │
    │             │             │ [C] Cache results        │             │             │
    │             │             ├────────────────────────►│             │             │
    │             │◄────────────┤            │             │             │             │
    │             │             │            │             │             │             │
    │ 200 OK      │             │            │             │             │             │
    │ {listings[]}│             │            │             │             │             │
    │◄────────────────────────┤             │            │             │             │
    │             │             │            │             │             │             │
    │ POST /saved-searches     │             │             │             │             │
    │ {criteria, alertSettings}│             │             │             │             │
    ├────────────────────────►│             │            │             │             │
    │             │ [S]         │            │             │             │             │
    │             │ Save search │            │             │             │             │
    │             ├────────────►│            │             │             │             │
    │             │             │ [DB] Save criteria       │             │             │
    │             │             ├───────────►│             │             │             │
    │             │             │◄───────────┤             │             │             │
    │             │◄────────────┤            │             │             │             │
    │ 201 Created │             │            │             │             │             │
    │◄────────────────────────┤             │            │             │             │
    │             │             │            │             │             │             │
    │             │             │            │             │             │             │
    │ ─ ─ ─ ─ ─ ─ (Later: New listing matches criteria) ─ ─ ─ ─ ─ ─ ─ ─ ─             │
    │             │             │            │             │             │             │
    │             │             │ [A] ListingCreated event│             │             │
    │             │◄─────────────────────────────────────────────────────              │
    │             │             │            │             │             │             │
    │             │ [S] Match against saved searches       │             │             │
    │             ├────────────►│            │             │             │             │
    │             │             │ [DB] Query matching searches           │             │
    │             │             ├───────────►│             │             │             │
    │             │             │◄───────────┤             │             │             │
    │             │             │            │             │             │             │
    │             │             │ [A] Queue alert notifications          │             │
    │             │             ├─────────────────────────────────────────►            │
    │             │             │            │             │             │             │
    │             │             │            │             │  [A] Send alert emails    │
    │             │             │            │             │             ├────────────►│
    │             │             │            │             │             │             │

Flow Type: Synchronous search, async alert matching
Cache: Search result caching
Events: ListingCreated triggers alert matching
```

### 18. Property Inquiry and Response (UC-46.1-6)

```
┌────────┐     ┌─────┐     ┌─────────┐     ┌────┐     ┌───────┐     ┌───────┐     ┌─────────┐
│ Portal │     │ API │     │ Inquiry │     │ DB │     │ Queue │     │ Email │     │ Realtor │
│ User   │     │     │     │ Service │     │    │     │       │     │       │     │         │
└───┬────┘     └──┬──┘     └────┬────┘     └─┬──┘     └───┬───┘     └───┬───┘     └────┬────┘
    │             │             │            │            │             │              │
    │ POST /inquiries           │            │            │             │              │
    │ {listingId, message,      │            │            │             │              │
    │  preferredViewingTimes[]} │            │            │             │              │
    ├────────────────────────►│             │           │            │             │
    │             │ [S]         │            │            │             │              │
    │             │ Create inquiry          │            │             │              │
    │             ├────────────►│            │            │             │              │
    │             │             │            │            │             │              │
    │             │             │ [DB] Get listing & realtor            │              │
    │             │             ├───────────►│            │             │              │
    │             │             │◄───────────┤            │             │              │
    │             │             │            │            │             │              │
    │             │             │ [DB] Create inquiry     │             │              │
    │             │             ├───────────►│            │             │              │
    │             │             │◄───────────┤            │             │              │
    │             │             │            │            │             │              │
    │             │             │ [DB] Update listing analytics         │              │
    │             │             ├───────────►│            │             │              │
    │             │             │◄───────────┤            │             │              │
    │             │             │            │            │             │              │
    │             │             │ [A] Publish InquirySent │             │              │
    │             │             ├────────────────────────►│             │              │
    │             │◄────────────┤            │            │             │              │
    │             │             │            │            │             │              │
    │ 201 Created │             │            │            │             │              │
    │ {inquiryId} │             │            │            │             │              │
    │◄────────────────────────┤             │           │            │             │
    │             │             │            │            │             │              │
    │             │             │            │   [A] Email notification │              │
    │             │             │            │            ├────────────►│              │
    │             │             │            │            │             ├─────────────►│
    │             │             │            │            │             │              │
    │             │             │            │            │             │              │
    │ ─ ─ ─ ─ ─ ─ (Realtor responds) ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─              │
    │             │             │            │            │             │              │
    │             │             │            │            │         POST /inquiries/{id}/responses
    │             │             │            │            │             │ {message}    │
    │             │◄───────────────────────────────────────────────────────────────────┤
    │             │ [S]         │            │            │             │              │
    │             │ Add response            │            │             │              │
    │             ├────────────►│            │            │             │              │
    │             │             │ [DB] Save response      │             │              │
    │             │             ├───────────►│            │             │              │
    │             │             │◄───────────┤            │             │              │
    │             │             │            │            │             │              │
    │             │             │ [DB] Update inquiry status            │              │
    │             │             ├───────────►│            │             │              │
    │             │             │◄───────────┤            │             │              │
    │             │             │            │            │             │              │
    │             │             │ [A] Publish InquiryResponded          │              │
    │             │             ├────────────────────────►│             │              │
    │             │◄────────────┤            │            │             │              │
    │             │             │            │            │             │              │
    │             │ 200 OK      │            │            │             │              │
    │             ├─────────────────────────────────────────────────────────────────────►
    │             │             │            │            │             │              │
    │             │             │            │   [A] Email to user      │              │
    │             │             │            │            ├────────────►│              │
    │◄───────────────────────────────────────────────────────────────────              │

Flow Type: Synchronous with async notifications
Events Published: InquirySent, InquiryResponded
```

---

## Integration Flows

### 19. IoT Smart Meter Reading (UC-21.1)

```
┌───────────┐     ┌─────────┐     ┌─────────┐     ┌────┐     ┌───────┐     ┌───────┐
│ IoT Meter │     │   IoT   │     │ Meter   │     │ DB │     │ Queue │     │ Alert │
│           │     │ Gateway │     │ Service │     │    │     │       │     │Service│
└─────┬─────┘     └────┬────┘     └────┬────┘     └─┬──┘     └───┬───┘     └───┬───┘
      │                │              │            │            │             │
      │ MQTT: reading  │              │            │            │             │
      │ {deviceId, value, timestamp}  │            │            │             │
      ├───────────────►│              │            │            │             │
      │                │              │            │            │             │
      │                │ [A] POST /internal/meter-readings      │             │
      │                ├─────────────►│            │            │             │
      │                │              │            │            │             │
      │                │              │ [DB] Validate device    │             │
      │                │              ├───────────►│            │             │
      │                │              │◄───────────┤            │             │
      │                │              │            │            │             │
      │                │              │ [DB] Save reading       │             │
      │                │              ├───────────►│            │             │
      │                │              │◄───────────┤            │             │
      │                │              │            │            │             │
      │                │              │ [S] Check for anomaly   │             │
      │                │              ├───────────┤            │             │
      │                │              │            │            │             │
      │                │              │ [A] Publish MeterReadingReceived     │
      │                │              ├─────────────────────────►            │
      │                │◄─────────────┤            │            │             │
      │                │              │            │            │             │
      │                │              │            │  [A] Anomaly detected   │
      │                │              │            │            ├────────────►│
      │                │              │            │            │             │
      │                │              │            │            │   [A] Alert │
      │                │              │            │            │◄────────────┤

Flow Type: Async IoT event processing
Protocol: MQTT for device → gateway, HTTP for gateway → service
Events Published: MeterReadingReceived
Anomaly Detection: Real-time threshold checks
```

### 20. Calendar Sync (UC-22.1)

```
┌────────┐     ┌─────┐     ┌──────────┐     ┌────┐     ┌──────────┐     ┌──────────┐
│  User  │     │ API │     │ Calendar │     │ DB │     │  Google  │     │ Outlook  │
│        │     │     │     │ Service  │     │    │     │ Calendar │     │ Calendar │
└───┬────┘     └──┬──┘     └────┬─────┘     └─┬──┘     └────┬─────┘     └────┬─────┘
    │             │             │            │             │                │
    │ POST /calendar/connect    │            │             │                │
    │ {provider: "google"}      │            │             │                │
    ├────────────────────────►│             │            │             │
    │             │ [S]         │            │             │                │
    │             │ Initialize OAuth        │             │                │
    │             ├────────────►│            │             │                │
    │             │◄────────────┤            │             │                │
    │             │             │            │             │                │
    │ 302 Redirect to Google    │            │             │                │
    │◄────────────────────────┤             │            │             │
    │             │             │            │             │                │
    │ (OAuth flow with Google)  │            │             │                │
    ├────────────────────────────────────────────────────►│                │
    │◄────────────────────────────────────────────────────┤                │
    │             │             │            │             │                │
    │ GET /calendar/callback?code=xxx        │             │                │
    ├────────────────────────►│             │            │             │
    │             │ [S]         │            │             │                │
    │             │ Complete OAuth          │             │                │
    │             ├────────────►│            │             │                │
    │             │             │ [E] Exchange code        │                │
    │             │             ├────────────────────────►│                │
    │             │             │◄────────────────────────┤                │
    │             │             │            │             │                │
    │             │             │ [DB] Store tokens        │                │
    │             │             ├───────────►│             │                │
    │             │             │◄───────────┤             │                │
    │             │             │            │             │                │
    │             │             │ [E] Fetch existing events│                │
    │             │             ├────────────────────────►│                │
    │             │             │◄────────────────────────┤                │
    │             │             │            │             │                │
    │             │             │ [DB] Sync events         │                │
    │             │             ├───────────►│             │                │
    │             │◄────────────┤            │             │                │
    │             │             │            │             │                │
    │ 200 OK {connected}        │            │             │                │
    │◄────────────────────────┤             │            │             │
    │             │             │            │             │                │
    │             │             │            │             │                │
    │ ─ ─ ─ ─ ─ ─ (Event created in PPT) ─ ─ ─ ─ ─ ─ ─ ─ ─                 │
    │             │             │            │             │                │
    │             │             │ [A] EventCreated        │                │
    │             │◄────────────────────────┤             │                │
    │             │             │            │             │                │
    │             │             │ [E] Create Google event  │                │
    │             │             ├────────────────────────►│                │
    │             │             │◄────────────────────────┤                │

Flow Type: OAuth flow + bidirectional sync
External Calls: Google Calendar API, Outlook Calendar API
Sync: Bidirectional event synchronization
```

### 21. CRM Property Import (UC-50.1-6)

```
┌─────────┐     ┌─────┐     ┌─────────┐     ┌────┐     ┌─────────┐     ┌───────┐     ┌───────┐
│ Realtor │     │ API │     │ Import  │     │ DB │     │   CRM   │     │ Queue │     │Storage│
│         │     │     │     │ Service │     │    │     │   API   │     │       │     │       │
└────┬────┘     └──┬──┘     └────┬────┘     └─┬──┘     └────┬────┘     └───┬───┘     └───┬───┘
     │             │             │            │             │              │             │
     │ POST /imports            │             │             │              │             │
     │ {crmType, credentials,   │             │             │              │             │
     │  fieldMapping}           │             │             │              │             │
     ├────────────────────────►│             │            │             │              │
     │             │ [S]         │            │             │              │             │
     │             │ Create import config    │             │              │             │
     │             ├────────────►│            │             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [E] Test CRM connection  │              │             │
     │             │             ├────────────────────────►│              │             │
     │             │             │◄────────────────────────┤              │             │
     │             │             │            │             │              │             │
     │             │             │ [DB] Save import config │              │             │
     │             │             ├───────────►│             │              │             │
     │             │             │◄───────────┤             │              │             │
     │             │◄────────────┤            │             │              │             │
     │             │             │            │             │              │             │
     │ 201 Created │             │            │             │              │             │
     │ {importId}  │             │            │             │              │             │
     │◄────────────────────────┤             │            │             │              │
     │             │             │            │             │              │             │
     │ POST /imports/{id}/run   │             │             │              │             │
     ├────────────────────────►│             │            │             │              │
     │             │ [S]         │            │             │              │             │
     │             │ Start import            │             │              │             │
     │             ├────────────►│            │             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [A] Queue import job    │              │             │
     │             │             ├────────────────────────────────────────►            │
     │             │◄────────────┤            │             │              │             │
     │             │             │            │             │              │             │
     │ 202 Accepted│             │            │             │              │             │
     │ {importRunId}             │            │             │              │             │
     │◄────────────────────────┤             │            │             │              │
     │             │             │            │             │              │             │
     │             │             │            │     [A] Process import job │             │
     │             │             │◄───────────────────────────────────────┤             │
     │             │             │            │             │              │             │
     │             │             │ [E] Fetch properties     │              │             │
     │             │             ├────────────────────────►│              │             │
     │             │             │◄────────────────────────┤              │             │
     │             │             │            │             │              │             │
     │             │             │ For each property:      │              │             │
     │             │             ├───────────┤             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [S] Transform data      │              │             │
     │             │             ├───────────┤             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [E] Download photos     │              │             │
     │             │             ├────────────────────────►│              │             │
     │             │             │◄────────────────────────┤              │             │
     │             │             │            │             │              │             │
     │             │             │ [E] Upload to storage   │              │             │
     │             │             ├──────────────────────────────────────────────────────►
     │             │             │◄──────────────────────────────────────────────────────
     │             │             │            │             │              │             │
     │             │             │ [DB] Create/update listing             │             │
     │             │             ├───────────►│             │              │             │
     │             │             │◄───────────┤             │              │             │
     │             │             │            │             │              │             │
     │             │             │ [A] Publish ImportCompleted            │             │
     │             │             ├────────────────────────────────────────►             │

Flow Type: Async batch import with progress tracking
External Calls: CRM API, Object Storage
Events Published: ImportStarted, ImportProgress, ImportCompleted
```

---

## AI/ML Flows

### 22. AI Chatbot Interaction (UC-20.1-2)

```
┌────────┐     ┌─────┐     ┌──────────┐     ┌────┐     ┌────────┐     ┌───────────┐
│  User  │     │ API │     │ Chatbot  │     │ DB │     │   LLM  │     │ Knowledge │
│        │     │     │     │ Service  │     │    │     │  API   │     │   Base    │
└───┬────┘     └──┬──┘     └────┬─────┘     └─┬──┘     └───┬────┘     └─────┬─────┘
    │             │             │            │            │                │
    │ POST /chat  │             │            │            │                │
    │ {message}   │             │            │            │                │
    ├────────────────────────►│             │           │            │
    │             │ [S]         │            │            │                │
    │             │ Process message         │            │                │
    │             ├────────────►│            │            │                │
    │             │             │            │            │                │
    │             │             │ [DB] Get conversation context           │
    │             │             ├───────────►│            │                │
    │             │             │◄───────────┤            │                │
    │             │             │            │            │                │
    │             │             │ [S] Classify intent     │                │
    │             │             ├────────────────────────►│                │
    │             │             │◄────────────────────────┤                │
    │             │             │            │            │                │
    │             │             │ [S] Search knowledge base               │
    │             │             ├─────────────────────────────────────────►│
    │             │             │◄─────────────────────────────────────────┤
    │             │             │            │            │                │
    │             │             │ [S] Generate response   │                │
    │             │             ├────────────────────────►│                │
    │             │             │◄────────────────────────┤                │
    │             │             │            │            │                │
    │             │             │ [DB] Save conversation  │                │
    │             │             ├───────────►│            │                │
    │             │             │◄───────────┤            │                │
    │             │◄────────────┤            │            │                │
    │             │             │            │            │                │
    │ 200 OK      │             │            │            │                │
    │ {response,  │             │            │            │                │
    │  actions[]} │             │            │            │                │
    │◄────────────────────────┤             │           │            │
    │             │             │            │            │                │
    │             │             │            │            │                │
    │ ─ ─ ─ (If action: report_fault) ─ ─ ─ ─ ─ ─ ─ ─ ─ ─                 │
    │             │             │            │            │                │
    │ POST /chat  │             │            │            │                │
    │ {message: "leaky faucet in bathroom"} │            │                │
    ├────────────────────────►│             │           │            │
    │             │ [S]         │            │            │                │
    │             │ Process with context    │            │                │
    │             ├────────────►│            │            │                │
    │             │             │            │            │                │
    │             │             │ [S] Extract fault details│               │
    │             │             ├────────────────────────►│                │
    │             │             │◄────────────────────────┤                │
    │             │             │            │            │                │
    │             │             │ [S] Pre-fill fault form │                │
    │             │             ├───────────┤            │                │
    │             │◄────────────┤            │            │                │
    │             │             │            │            │                │
    │ 200 OK      │             │            │            │                │
    │ {response: "I'll help...",│            │            │                │
    │  prefillData: {...}}      │            │            │                │
    │◄────────────────────────┤             │           │            │

Flow Type: Synchronous with LLM API calls
External Calls: LLM API (OpenAI, Anthropic, etc.)
Features: Intent classification, knowledge retrieval, response generation
```

### 23. Predictive Maintenance (UC-20.6)

```
┌──────────┐     ┌───────┐     ┌───────────┐     ┌────┐     ┌────────┐     ┌───────┐
│Scheduler │     │ Queue │     │Prediction │     │ DB │     │  ML    │     │ Alert │
│          │     │       │     │ Service   │     │    │     │ Model  │     │Service│
└────┬─────┘     └───┬───┘     └─────┬─────┘     └─┬──┘     └───┬────┘     └───┬───┘
     │               │               │            │            │              │
     │ [A] Trigger prediction job    │            │            │              │
     ├──────────────►│               │            │            │              │
     │               │               │            │            │              │
     │               │ [A] Process job            │            │              │
     │               ├──────────────►│            │            │              │
     │               │               │            │            │              │
     │               │               │ [DB] Get equipment data │              │
     │               │               ├───────────►│            │              │
     │               │               │◄───────────┤            │              │
     │               │               │            │            │              │
     │               │               │ [DB] Get fault history  │              │
     │               │               ├───────────►│            │              │
     │               │               │◄───────────┤            │              │
     │               │               │            │            │              │
     │               │               │ [DB] Get maintenance records          │
     │               │               ├───────────►│            │              │
     │               │               │◄───────────┤            │              │
     │               │               │            │            │              │
     │               │               │ [S] Run ML prediction   │              │
     │               │               ├────────────────────────►│              │
     │               │               │◄────────────────────────┤              │
     │               │               │            │            │              │
     │               │               │ For each prediction:    │              │
     │               │               ├───────────┤            │              │
     │               │               │            │            │              │
     │               │               │ [DB] Save prediction    │              │
     │               │               ├───────────►│            │              │
     │               │               │◄───────────┤            │              │
     │               │               │            │            │              │
     │               │               │ If high risk:           │              │
     │               │               │ [A] Create alert        │              │
     │               │               ├────────────────────────────────────────►
     │               │               │            │            │              │
     │               │               │ [A] Publish PredictionGenerated       │
     │               │               ├─────────────────────────►              │
     │               │◄──────────────┤            │            │              │

Flow Type: Scheduled async batch processing
ML Operations: Feature extraction, model inference
Output: Maintenance predictions and risk alerts
Trigger: Daily/weekly cron job
```

---

## Summary

### Flow Categories

| Category | Sync | Async | External Calls | Real-time |
|----------|------|-------|----------------|-----------|
| Authentication | 5 | 3 | OAuth Providers | WebSocket |
| Property Management | 4 | 6 | Storage, AI | WebSocket |
| Financial | 2 | 4 | Payment Gateway | - |
| Communication | 2 | 2 | Push, Email | WebSocket |
| Short-term Rental | 2 | 4 | Airbnb, Booking, Police | IoT |
| Reality Portal | 4 | 4 | Storage | - |
| Integration | 2 | 5 | IoT, Calendar, CRM | MQTT |
| AI/ML | 2 | 3 | LLM APIs | - |

### External Service Dependencies

| Service | Used By | Protocol |
|---------|---------|----------|
| OAuth Providers (Google, Apple, Facebook) | Auth | OAuth 2.0 |
| Payment Gateway (Stripe) | Financial | REST + Webhooks |
| Object Storage (S3/MinIO) | Documents, Photos | REST |
| Email Service (SendGrid/SES) | Notifications | REST |
| SMS Gateway | Notifications | REST |
| Push Notification (FCM/APNs) | Mobile Alerts | REST |
| LLM API (OpenAI/Anthropic) | AI Features | REST |
| Airbnb API | Rentals | REST |
| Booking.com API | Rentals | REST |
| Google/Outlook Calendar | Integrations | REST |
| Smart Lock APIs | IoT | REST |
| IoT Gateway | Smart Meters | MQTT |
| Police Portal | Guest Registration | REST/SOAP |

### Async Processing Patterns

| Pattern | Use Case | Technology |
|---------|----------|------------|
| Event-Driven | Notifications, Alerts | RabbitMQ/SQS |
| Scheduled Jobs | Reports, Sync, Predictions | Cron + Queue |
| Real-time | Messaging, Voting, Status | WebSocket |
| Webhooks | Payments, External Events | HTTP Callbacks |
| IoT Streaming | Meter Readings | MQTT |
