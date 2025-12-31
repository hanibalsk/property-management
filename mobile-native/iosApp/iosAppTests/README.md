# Reality Portal iOS Tests

Epic 80 - Story 80.6: Mobile Unit Tests (iOS)

## Overview

This directory contains XCTest scaffolding for the Reality Portal iOS application.

## Test Structure

```
iosAppTests/
├── README.md                    # This file
├── Info.plist                   # Test target configuration
└── RealityPortalTests.swift     # Main test file with test classes
```

## Test Classes

| Class | Purpose |
|-------|---------|
| `RealityPortalTests` | Main test class with placeholder tests |
| `ConfigurationTests` | Tests for app configuration |
| `NavigationTests` | Tests for navigation coordinator |
| `AuthenticationTests` | Tests for authentication manager |
| `ViewTests` | Tests for SwiftUI views |
| `DependencyInjectionTests` | Tests for DI container |
| `ModelTests` | Tests for data models |
| `PerformanceTests` | Performance measurements |

## Setup Instructions

### Adding Test Target to Xcode Project

Since this is a Kotlin Multiplatform project with iOS support, you need to add the test target to your Xcode project manually:

1. Open the Xcode project (or create one if using SPM/CocoaPods)
2. Go to File > New > Target
3. Select "Unit Testing Bundle"
4. Name it "iosAppTests"
5. Add the test files to the target
6. Configure the test target to depend on the main iosApp target

### Running Tests

**From Xcode:**
```
Cmd + U  # Run all tests
```

**From Command Line:**
```bash
xcodebuild test \
  -project iosApp.xcodeproj \
  -scheme iosApp \
  -destination 'platform=iOS Simulator,name=iPhone 15'
```

## Implementing Tests

The current tests are placeholder tests that verify the test infrastructure is working.
To implement actual tests:

1. Remove the placeholder assertions
2. Import the actual components being tested
3. Write meaningful assertions

### Example: Testing Configuration

```swift
func testConfigurationExists() throws {
    let config = Configuration.shared
    XCTAssertNotNil(config)
    XCTAssertFalse(config.apiBaseUrl.isEmpty)
}
```

### Example: Testing Navigation

```swift
func testDeepLinkHandling() throws {
    let coordinator = NavigationCoordinator()
    let url = URL(string: "realityportal://listing/123")!

    coordinator.handleDeepLink(url)

    XCTAssertEqual(coordinator.selectedTab, .search)
}
```

## Dependencies

Tests use:
- XCTest framework (built into Xcode)
- `@testable import iosApp` for internal access

## Notes

- Tests marked with `async` require Swift 5.5+ concurrency support
- Performance tests should be run in Release configuration for accurate measurements
- Mock objects may be needed for testing network-dependent code
