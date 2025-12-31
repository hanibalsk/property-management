import Foundation

/// Environment configuration for Reality Portal iOS app.
///
/// Epic 82 - Story 82.1: SwiftUI Project Setup
enum Environment: String {
    case development
    case staging
    case production

    /// API base URL for each environment.
    var apiBaseUrl: String {
        switch self {
        case .development:
            return "http://localhost:8081"
        case .staging:
            return "https://staging-api.reality.example.com"
        case .production:
            return "https://api.reality.example.com"
        }
    }

    /// Deep link URL scheme.
    var urlScheme: String {
        return "realityportal"
    }

    /// Universal link domain.
    var universalLinkDomain: String {
        switch self {
        case .development:
            return "localhost"
        case .staging:
            return "staging.reality.example.com"
        case .production:
            return "reality.example.com"
        }
    }

    /// Web URL base for sharing links (Story 85.3).
    var webBaseUrl: String {
        switch self {
        case .development:
            return "http://localhost:3000"
        case .staging:
            return "https://staging.reality.example.com"
        case .production:
            return "https://reality.example.com"
        }
    }
}

/// App configuration singleton.
final class Configuration {
    static let shared = Configuration()

    /// Current environment - defaults to development, can be changed via build configuration.
    #if DEBUG
    let environment: Environment = .development
    #else
    let environment: Environment = .production
    #endif

    /// Bundle identifier for the app.
    let bundleIdentifier = "three.two.bit.ppt.reality"

    /// App display name.
    let appName = "Reality Portal"

    /// API base URL for current environment.
    var apiBaseUrl: String {
        environment.apiBaseUrl
    }

    /// Web base URL for sharing (Story 85.3).
    var webBaseUrl: String {
        environment.webBaseUrl
    }

    /// Keychain service identifier.
    var keychainService: String {
        bundleIdentifier
    }

    // MARK: - Version Information (Story 85.2)

    /// App version from Info.plist (e.g., "0.2.194")
    var version: String {
        Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "Unknown"
    }

    /// App build number from Info.plist (e.g., "2194")
    var buildNumber: String {
        Bundle.main.infoDictionary?["CFBundleVersion"] as? String ?? "0"
    }

    /// Full version string combining version and build (e.g., "0.2.194 (2194)")
    var fullVersionString: String {
        "\(version) (\(buildNumber))"
    }

    private init() {}
}
