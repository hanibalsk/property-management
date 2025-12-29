# Story 85.2: Build Configuration by Environment

Status: pending

## Story

As a **mobile developer**,
I want to **have distinct build configurations for each environment**,
So that **I can easily build and deploy to different environments with proper settings**.

## Acceptance Criteria

1. **AC-1: Android Build Flavors**
   - Given I want to build for Android
   - When I select a build flavor
   - Then the correct configuration is applied
   - And the APK has the correct app ID suffix
   - And debug/release variants work correctly

2. **AC-2: iOS Build Schemes**
   - Given I want to build for iOS
   - When I select a build scheme
   - Then the correct configuration is applied
   - And the IPA has the correct bundle ID
   - And signing is configured correctly

3. **AC-3: KMP Build Variants**
   - Given I want to build the KMP module
   - When building for different environments
   - Then the shared code uses correct config
   - And platform-specific code compiles correctly

4. **AC-4: App Distinguishability**
   - Given I have multiple app versions installed
   - When viewing the app icon or name
   - Then I can distinguish development from staging
   - And staging from production

5. **AC-5: Automated Build Scripts**
   - Given I want to build from command line
   - When I run the build script
   - Then the correct environment is built
   - And all dependencies are properly linked

## Tasks / Subtasks

- [ ] Task 1: Configure Android Build System (AC: 1, 4)
  - [ ] 1.1 Set up product flavors (dev, staging, prod)
  - [ ] 1.2 Configure application ID suffixes
  - [ ] 1.3 Create app icons per flavor
  - [ ] 1.4 Configure app names per flavor
  - [ ] 1.5 Set up signing configs

- [ ] Task 2: Configure iOS Build System (AC: 2, 4)
  - [ ] 2.1 Create build schemes (Development, Staging, Production)
  - [ ] 2.2 Configure bundle ID per scheme
  - [ ] 2.3 Create app icons per scheme
  - [ ] 2.4 Configure display names per scheme
  - [ ] 2.5 Set up provisioning profiles

- [ ] Task 3: Configure KMP Build Variants (AC: 3)
  - [ ] 3.1 Set up Gradle build types
  - [ ] 3.2 Configure shared module variants
  - [ ] 3.3 Set up Android library variants
  - [ ] 3.4 Configure iOS framework variants

- [ ] Task 4: Create App Icons and Assets (AC: 4)
  - [ ] 4.1 Create development app icon (with "DEV" badge)
  - [ ] 4.2 Create staging app icon (with "STG" badge)
  - [ ] 4.3 Create production app icon
  - [ ] 4.4 Generate all required sizes

- [ ] Task 5: Create Build Scripts (AC: 5)
  - [ ] 5.1 Create Android build script
  - [ ] 5.2 Create iOS build script
  - [ ] 5.3 Create KMP build script
  - [ ] 5.4 Create unified build script
  - [ ] 5.5 Document build commands

## Dev Notes

### Architecture Requirements
- Separate app identifiers per environment
- Visual distinction for non-production builds
- Automated build process
- Consistent configuration across platforms

### Technical Specifications
- Android: Gradle product flavors with build types
- iOS: Xcode schemes with configurations
- KMP: Gradle multi-variant support

### Android Gradle Configuration
```groovy
// mobile-native/androidApp/build.gradle.kts
android {
    namespace = "three.two.bit.ppt.reality"

    defaultConfig {
        applicationId = "three.two.bit.ppt.reality"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "1.0.0"
    }

    signingConfigs {
        create("release") {
            storeFile = file(System.getenv("KEYSTORE_FILE") ?: "../keystore/release.jks")
            storePassword = System.getenv("KEYSTORE_PASSWORD") ?: ""
            keyAlias = System.getenv("KEY_ALIAS") ?: "release"
            keyPassword = System.getenv("KEY_PASSWORD") ?: ""
        }
    }

    flavorDimensions += "environment"
    productFlavors {
        create("development") {
            dimension = "environment"
            applicationIdSuffix = ".dev"
            versionNameSuffix = "-dev"
            resValue("string", "app_name", "Reality (Dev)")
            buildConfigField("String", "API_BASE_URL", "\"http://10.0.2.2:8081\"")
            buildConfigField("Boolean", "ENABLE_LOGGING", "true")
        }
        create("staging") {
            dimension = "environment"
            applicationIdSuffix = ".staging"
            versionNameSuffix = "-staging"
            resValue("string", "app_name", "Reality (Staging)")
            buildConfigField("String", "API_BASE_URL", "\"https://staging-reality.ppt.example.com\"")
            buildConfigField("Boolean", "ENABLE_LOGGING", "true")
        }
        create("production") {
            dimension = "environment"
            resValue("string", "app_name", "Reality Portal")
            buildConfigField("String", "API_BASE_URL", "\"https://reality.ppt.example.com\"")
            buildConfigField("Boolean", "ENABLE_LOGGING", "false")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
            signingConfig = signingConfigs.getByName("release")
        }
        debug {
            isDebuggable = true
            applicationIdSuffix = ".debug"
        }
    }
}
```

### iOS Project Configuration
```xml
<!-- Info.plist with variable substitution -->
<key>CFBundleIdentifier</key>
<string>$(PRODUCT_BUNDLE_IDENTIFIER)</string>
<key>CFBundleName</key>
<string>$(PRODUCT_NAME)</string>
<key>API_BASE_URL</key>
<string>$(API_BASE_URL)</string>
```

```
// Development.xcconfig
PRODUCT_BUNDLE_IDENTIFIER = three.two.bit.ppt.reality.dev
PRODUCT_NAME = Reality (Dev)
API_BASE_URL = http:/$()/localhost:8081
ASSETCATALOG_COMPILER_APPICON_NAME = AppIcon-Dev

// Staging.xcconfig
PRODUCT_BUNDLE_IDENTIFIER = three.two.bit.ppt.reality.staging
PRODUCT_NAME = Reality (Staging)
API_BASE_URL = https:/$()/staging-reality.ppt.example.com
ASSETCATALOG_COMPILER_APPICON_NAME = AppIcon-Staging

// Release.xcconfig
PRODUCT_BUNDLE_IDENTIFIER = three.two.bit.ppt.reality
PRODUCT_NAME = Reality Portal
API_BASE_URL = https:/$()/reality.ppt.example.com
ASSETCATALOG_COMPILER_APPICON_NAME = AppIcon
```

### App Icon Variants
```
Assets.xcassets/
├── AppIcon.appiconset/          # Production (blue)
│   └── Contents.json
├── AppIcon-Dev.appiconset/      # Development (green with DEV badge)
│   └── Contents.json
└── AppIcon-Staging.appiconset/  # Staging (orange with STG badge)
    └── Contents.json
```

### Build Scripts
```bash
#!/bin/bash
# scripts/build-mobile.sh

ENVIRONMENT=${1:-development}
PLATFORM=${2:-android}
BUILD_TYPE=${3:-debug}

case $PLATFORM in
  android)
    cd mobile-native/androidApp
    ./gradlew assemble${ENVIRONMENT^}${BUILD_TYPE^}
    ;;
  ios)
    cd mobile-native/iosApp
    xcodebuild -workspace iosApp.xcworkspace \
      -scheme "${ENVIRONMENT^}" \
      -configuration "${BUILD_TYPE^}" \
      -derivedDataPath build \
      build
    ;;
  all)
    $0 $ENVIRONMENT android $BUILD_TYPE
    $0 $ENVIRONMENT ios $BUILD_TYPE
    ;;
esac
```

### KMP Shared Module Variants
```kotlin
// shared/build.gradle.kts
kotlin {
    android {
        publishLibraryVariants("release", "debug")
    }

    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach {
        it.binaries.framework {
            baseName = "shared"
            isStatic = true

            // Environment-specific configuration at runtime
        }
    }
}
```

### File List (to create/modify)

**Create:**
- `/mobile-native/androidApp/src/development/res/mipmap-*/ic_launcher.png`
- `/mobile-native/androidApp/src/staging/res/mipmap-*/ic_launcher.png`
- `/mobile-native/iosApp/iosApp/Assets.xcassets/AppIcon-Dev.appiconset/`
- `/mobile-native/iosApp/iosApp/Assets.xcassets/AppIcon-Staging.appiconset/`
- `/mobile-native/iosApp/iosApp/Configuration/Development.xcconfig`
- `/mobile-native/iosApp/iosApp/Configuration/Staging.xcconfig`
- `/mobile-native/iosApp/iosApp/Configuration/Release.xcconfig`
- `/scripts/build-mobile.sh`
- `/scripts/build-android.sh`
- `/scripts/build-ios.sh`

**Modify:**
- `/mobile-native/androidApp/build.gradle.kts` - Add flavors
- `/mobile-native/iosApp/iosApp.xcodeproj/project.pbxproj` - Add schemes
- `/mobile-native/shared/build.gradle.kts` - Configure variants

### CI/CD Matrix Build
```yaml
# .github/workflows/mobile-release.yml
jobs:
  build:
    strategy:
      matrix:
        include:
          - platform: android
            environment: development
            artifact: app-development-debug.apk
          - platform: android
            environment: staging
            artifact: app-staging-release.apk
          - platform: android
            environment: production
            artifact: app-production-release.apk
          - platform: ios
            environment: Development
            artifact: Reality-Dev.ipa
          - platform: ios
            environment: Staging
            artifact: Reality-Staging.ipa
          - platform: ios
            environment: Production
            artifact: Reality.ipa
    steps:
      - uses: actions/checkout@v4
      - name: Build ${{ matrix.platform }} ${{ matrix.environment }}
        run: ./scripts/build-mobile.sh ${{ matrix.environment }} ${{ matrix.platform }} release
      - uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact }}
          path: build/${{ matrix.artifact }}
```

### Dependencies
- Story 85.1 (Environment Variables) - Environment configuration

### References
- [Android Build Variants Documentation]
- [Xcode Build Settings Reference]
- [KMP Multiplatform Configuration]
