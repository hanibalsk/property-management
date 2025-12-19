# Mobile Native - CLAUDE.md

> **Parent:** See root `CLAUDE.md` for namespace and architecture.

## Overview

Kotlin Multiplatform project for Reality Portal mobile apps.

**Package ID:** `bit.two.three.ppt.reality`

## Targets

| Platform | App |
|----------|-----|
| Android | Reality Portal (bit.two.three.ppt.reality) |
| iOS | Reality Portal (bit.two.three.ppt.reality) |

## Quick Commands

```bash
# Build shared module
./gradlew :shared:build

# Build Android app
./gradlew :androidApp:assembleDebug

# Build iOS framework
./gradlew :shared:linkDebugFrameworkIosSimulatorArm64

# Run tests
./gradlew :shared:allTests
```

## Project Structure

```
mobile-native/
├── build.gradle.kts     # Root build config
├── settings.gradle.kts  # Project settings
├── shared/              # KMP shared code
│   ├── build.gradle.kts
│   └── src/
│       ├── commonMain/  # Shared Kotlin
│       │   └── kotlin/
│       │       └── api/ # Generated API client
│       ├── androidMain/ # Android-specific
│       └── iosMain/     # iOS-specific
├── androidApp/          # Android application
│   ├── build.gradle.kts
│   └── src/main/
└── iosApp/              # iOS application (Xcode)
    └── iosApp/
```

## API Client Generation

```bash
openapi-generator generate \
  -i docs/api/generated/by-service/reality-server.yaml \
  -g kotlin \
  -o shared/src/commonMain/kotlin/api \
  --additional-properties=library=multiplatform
```

## Dependencies

- Ktor Client - HTTP networking
- Kotlin Serialization - JSON parsing
- Kotlin Coroutines - Async operations
