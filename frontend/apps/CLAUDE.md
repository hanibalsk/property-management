# Frontend Apps - CLAUDE.md

> **Parent:** See `frontend/CLAUDE.md` for monorepo overview.

## Applications

### ppt-web (@ppt/web)

**Technology:** React + Vite (SPA)
**Backend:** api-server (Port 8080)
**Purpose:** Property Management web application

Features:
- Manager dashboard
- Building management
- Fault reporting
- Voting system
- User management

```bash
pnpm --filter @ppt/web dev
```

---

### reality-web (@ppt/reality-web)

**Technology:** Next.js 14 (SSR + ISR)
**Backend:** reality-server (Port 8081)
**Purpose:** Reality Portal public website

Features:
- Property listing search
- Listing detail pages
- User favorites
- Contact/viewing requests
- Multi-language (sk, cs, de, en)

```bash
pnpm --filter @ppt/reality-web dev
```

i18n Files: `messages/` directory
- `en.json` - English
- `sk.json` - Slovak
- `cs.json` - Czech
- `de.json` - German

---

### mobile (@ppt/mobile)

**Technology:** React Native (Expo)
**Backend:** api-server (Port 8080)
**Package ID:** `three.two.bit.ppt.management`
**Purpose:** Property Management mobile app

```bash
pnpm --filter @ppt/mobile start    # Start Expo
pnpm --filter @ppt/mobile android  # Run on Android
pnpm --filter @ppt/mobile ios      # Run on iOS
```
