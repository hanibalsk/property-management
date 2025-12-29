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

    /// Keychain service identifier.
    var keychainService: String {
        bundleIdentifier
    }

    private init() {}
}
