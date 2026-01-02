# Epic 111: Multi-Language Support Across All Platforms

## Overview

Implement comprehensive internationalization (i18n) support across all platforms in the Property Management System. This epic expands language support from the current partial implementation (reality-web only with 4 languages) to full multi-language support across all frontend applications, mobile apps, and backend services.

## Business Value

- **Market Expansion**: Enable the platform to serve users in multiple European markets (SK, CZ, DE, AT, PL, HU)
- **User Experience**: Allow users to interact with the system in their preferred language
- **Competitive Advantage**: Multi-language support is essential for real estate platforms serving diverse populations
- **Accessibility**: Improve accessibility for non-English speaking users

## Target Languages

| Code | Language | Priority | Markets |
|------|----------|----------|---------|
| en | English | P0 | Global (fallback) |
| sk | Slovak | P0 | Slovakia |
| cs | Czech | P0 | Czech Republic |
| de | German | P0 | Germany, Austria |
| pl | Polish | P1 | Poland |
| hu | Hungarian | P1 | Hungary |

## Current State

| Platform | Library | Languages | Status |
|----------|---------|-----------|--------|
| reality-web | next-intl 3.4.0 | en, sk, cs, de | Files exist, integration incomplete |
| ppt-web | None | en only | Not implemented |
| mobile | None | en only | Not implemented |
| mobile-native | None | en only | Not implemented |
| backend | None | en only | Not implemented |

## Technical Approach

### Frontend (TypeScript)
- **reality-web**: Complete next-intl integration, add locale routing middleware
- **ppt-web**: Add i18next with react-i18next for React SPA
- **mobile**: Add react-i18next with expo-localization

### Mobile Native (Kotlin)
- **mobile-native**: Use Compose Resources for multiplatform string localization

### Backend (Rust)
- Add Accept-Language header parsing
- Implement error message localization
- Support locale parameter in API responses

## Stories

---

### Story 111.1: Complete reality-web i18n Integration

**As a** Reality Portal user
**I want** to browse listings in my preferred language
**So that** I can understand property details and navigate the site easily

**Acceptance Criteria:**
1. Middleware detects user's preferred language from browser settings
2. URL-based locale routing works (e.g., /sk/listings, /de/listings)
3. All existing components use useTranslations() hook
4. Language switcher allows users to change language
5. Selected language persists in cookie/localStorage
6. All 4 existing languages (en, sk, cs, de) work correctly

**Technical Tasks:**
- [ ] Create middleware.ts for locale detection and routing
- [ ] Create i18n.ts configuration file
- [ ] Update next.config.js with i18n settings
- [ ] Wrap RootLayout with NextIntlClientProvider
- [ ] Update all page components to use getTranslations()
- [ ] Update all client components to use useTranslations()
- [ ] Add language switcher component to header
- [ ] Add locale persistence (cookie)
- [ ] Update all hardcoded strings in components

**Story Points:** 8

---

### Story 111.2: Add Polish and Hungarian to reality-web

**As a** user from Poland or Hungary
**I want** to browse Reality Portal in my native language
**So that** I can fully understand property listings

**Acceptance Criteria:**
1. Polish (pl) translations added for all strings
2. Hungarian (hu) translations added for all strings
3. Language switcher includes new languages
4. Locale routing supports /pl and /hu paths

**Technical Tasks:**
- [ ] Create messages/pl.json with Polish translations
- [ ] Create messages/hu.json with Hungarian translations
- [ ] Update i18n configuration to include new locales
- [ ] Add language flags/labels for switcher
- [ ] Test all pages in new languages

**Story Points:** 3

---

### Story 111.3: Implement i18n in ppt-web (Property Management)

**As a** property manager
**I want** to use the management dashboard in my preferred language
**So that** I can manage properties without language barriers

**Acceptance Criteria:**
1. i18next configured and working
2. All UI strings externalized to translation files
3. Support for en, sk, cs, de languages
4. Language detection from browser
5. Language switcher in settings/header
6. Date/number formatting respects locale

**Technical Tasks:**
- [ ] Install i18next, react-i18next dependencies
- [ ] Create i18n configuration (i18n.ts)
- [ ] Create locales directory structure
- [ ] Create en.json base translation file
- [ ] Externalize all hardcoded strings from components
- [ ] Create sk.json, cs.json, de.json translations
- [ ] Add I18nextProvider to App.tsx
- [ ] Implement useTranslation() in all components
- [ ] Add language switcher component
- [ ] Configure date-fns locale support

**Story Points:** 13

---

### Story 111.4: Implement i18n in mobile app (React Native)

**As a** mobile app user
**I want** to use the Property Management app in my language
**So that** I can report faults and manage my unit easily

**Acceptance Criteria:**
1. react-i18next configured for React Native
2. Device locale detection works
3. Support for en, sk, cs, de languages
4. In-app language switcher
5. All screens use translated strings
6. Push notifications respect user language

**Technical Tasks:**
- [ ] Install i18next, react-i18next, expo-localization
- [ ] Create i18n configuration
- [ ] Create translation files for each language
- [ ] Wrap app with I18nextProvider
- [ ] Add useTranslation() to all screens
- [ ] Implement language settings screen
- [ ] Store language preference in AsyncStorage
- [ ] Update notification service for localized messages

**Story Points:** 8

---

### Story 111.5: Implement i18n in mobile-native (Kotlin Multiplatform)

**As a** Reality Portal mobile app user
**I want** to browse listings in my native language
**So that** I can understand property details on mobile

**Acceptance Criteria:**
1. Compose Resources configured for string localization
2. Support for en, sk, cs, de, pl, hu languages
3. Device locale detection works
4. Android strings.xml resources created
5. iOS Localizable.strings created
6. All Compose UI uses stringResource()

**Technical Tasks:**
- [ ] Add Compose Resources plugin to build.gradle.kts
- [ ] Create commonMain/composeResources/values/strings.xml
- [ ] Create language-specific resource directories
- [ ] Update all Compose @Composable functions to use stringResource()
- [ ] Implement locale detection for Android
- [ ] Implement locale detection for iOS
- [ ] Add language picker in settings

**Story Points:** 8

---

### Story 111.6: Backend Localization Support

**As a** developer
**I want** the backend to support localized responses
**So that** error messages and API responses match user's language

**Acceptance Criteria:**
1. Accept-Language header is parsed on all requests
2. Error messages are localized
3. Validation messages support localization
4. API can return locale-specific content
5. Default fallback to English

**Technical Tasks:**
- [ ] Create locale extraction middleware
- [ ] Create localized error message catalog
- [ ] Implement error message lookup by locale
- [ ] Add locale context to request extensions
- [ ] Update all error responses to use localization
- [ ] Create translation files for error messages (en, sk, cs, de)
- [ ] Document API locale behavior

**Story Points:** 5

---

### Story 111.7: Add Polish and Hungarian to All Platforms

**As a** user from Poland or Hungary
**I want** to use all applications in my native language
**So that** I have a consistent experience across platforms

**Acceptance Criteria:**
1. ppt-web supports pl, hu languages
2. mobile (React Native) supports pl, hu
3. mobile-native (KMP) supports pl, hu
4. backend supports pl, hu error messages

**Technical Tasks:**
- [ ] Create ppt-web pl.json and hu.json
- [ ] Create mobile pl.json and hu.json
- [ ] Create mobile-native pl and hu string resources
- [ ] Create backend pl and hu error message catalogs
- [ ] Update all language switchers

**Story Points:** 5

---

## Dependencies

- None (self-contained epic)

## Risks

| Risk | Mitigation |
|------|------------|
| Missing translations | Use English as fallback, prioritize user-facing strings |
| Translation quality | Use native speakers for review before release |
| Performance impact | Use lazy loading for translation files |
| Maintenance overhead | Establish translation update workflow |

## Definition of Done

- [ ] All 6 platforms support en, sk, cs, de (minimum)
- [ ] reality-web and mobile-native also support pl, hu
- [ ] Language detection works automatically
- [ ] Users can switch language manually
- [ ] All user-facing strings are externalized
- [ ] Translation files are complete (no missing keys)
- [ ] Unit tests pass for locale handling
- [ ] E2E tests verify language switching
- [ ] Documentation updated

## Estimated Total Story Points: 50

## Priority Order

1. Story 111.1 - Complete reality-web (highest user traffic)
2. Story 111.3 - Implement ppt-web (manager critical path)
3. Story 111.5 - Implement mobile-native (Reality Portal mobile)
4. Story 111.4 - Implement mobile (PM mobile)
5. Story 111.6 - Backend localization
6. Story 111.2 - Add pl/hu to reality-web
7. Story 111.7 - Add pl/hu to all platforms
