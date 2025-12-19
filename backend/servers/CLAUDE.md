# Backend Servers - CLAUDE.md

> **Parent:** See `backend/CLAUDE.md` for workspace overview.

## Servers

### api-server (Port 8080)

**Purpose:** Property Management API

**Handles:**
- Authentication (UC-14)
- Organizations (UC-27)
- Buildings & Units (UC-15)
- Faults (UC-03)
- Voting (UC-04)
- Rentals (UC-29, UC-30)
- Listings management (UC-31)
- Integrations (UC-22, UC-32)

**Consumers:** ppt-web (React SPA), mobile (React Native)

**Endpoints:**
- `/health` - Health check
- `/api/v1/auth/*` - Authentication
- `/api/v1/organizations/*` - Organizations
- `/api/v1/buildings/*` - Buildings
- `/api/v1/faults/*` - Faults
- `/api/v1/voting/*` - Voting
- `/api/v1/rentals/*` - Short-term rentals
- `/api/v1/listings/*` - Listing management
- `/api/v1/integrations/*` - External integrations
- `/swagger-ui` - API documentation

---

### reality-server (Port 8081)

**Purpose:** Reality Portal public API

**Handles:**
- Public listing search and view
- Portal user accounts (separate from PM)
- Favorites
- Inquiries (contact, viewing requests)

**Consumers:** reality-web (Next.js), mobile-native (KMP)

**Endpoints:**
- `/health` - Health check (includes region)
- `/api/v1/listings/*` - Public listing search/view
- `/api/v1/users/*` - Portal user accounts
- `/api/v1/favorites/*` - Saved listings
- `/api/v1/inquiries/*` - Contact/viewing requests
- `/swagger-ui` - API documentation

**Multi-Region:**
- `REGION` env var controls deployment region
- Supports: sk, cz, eu, local
