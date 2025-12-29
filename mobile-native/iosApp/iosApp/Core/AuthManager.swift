import Foundation
import Observation

/// User information from authentication.
///
/// Epic 82 - Story 82.5: Inquiries and Account
struct User: Identifiable, Equatable {
    let id: String
    let email: String
    let name: String
    let avatarUrl: String?
}

/// Authentication state manager for Reality Portal iOS app.
///
/// Epic 82 - Story 82.5: Inquiries and Account
@Observable
final class AuthManager {
    // MARK: - Published Properties

    /// Current authenticated user, nil if not authenticated.
    private(set) var currentUser: User?

    /// Whether authentication is currently loading.
    private(set) var isLoading: Bool = false

    /// Last authentication error message.
    private(set) var errorMessage: String?

    // MARK: - Computed Properties

    /// Whether the user is currently authenticated.
    var isAuthenticated: Bool {
        accessToken != nil && currentUser != nil
    }

    // MARK: - Private Properties

    private var accessToken: String?
    private var refreshToken: String?
    private let configuration = Configuration.shared

    // MARK: - Initialization

    init() {
        // Attempt to restore session on initialization
        restoreSession()
    }

    // MARK: - Public Methods

    /// Login with email and password.
    /// - Parameters:
    ///   - email: User email address.
    ///   - password: User password.
    func login(email: String, password: String) async throws {
        isLoading = true
        errorMessage = nil

        defer { isLoading = false }

        // TODO: Integrate with KMP AuthUseCase
        // let authUseCase = AuthUseCase()
        // let result = try await authUseCase.login(email: email, password: password)

        // Placeholder implementation
        // In production, this would call the KMP shared module
        throw AuthError.notImplemented
    }

    /// Login via SSO token from Property Management app.
    /// - Parameter token: SSO token received via deep link.
    func loginWithSsoToken(_ token: String) async throws {
        isLoading = true
        errorMessage = nil

        defer { isLoading = false }

        // TODO: Integrate with KMP SsoUseCase to validate token
        throw AuthError.notImplemented
    }

    /// Logout the current user.
    func logout() {
        accessToken = nil
        refreshToken = nil
        currentUser = nil

        // Clear from secure storage
        clearStoredTokens()
    }

    /// Restore session from stored tokens.
    func restoreSession() {
        // TODO: Load tokens from Keychain
        // accessToken = try? keychain.get("accessToken")
        // refreshToken = try? keychain.get("refreshToken")

        if accessToken != nil {
            Task {
                await loadCurrentUser()
            }
        }
    }

    /// Refresh the access token using the refresh token.
    func refreshAccessToken() async throws {
        guard let refreshToken = refreshToken else {
            throw AuthError.noRefreshToken
        }

        // TODO: Integrate with KMP to refresh token
        _ = refreshToken
        throw AuthError.notImplemented
    }

    // MARK: - Private Methods

    private func loadCurrentUser() async {
        // TODO: Load current user from API using stored token
    }

    private func storeTokens(accessToken: String, refreshToken: String) {
        // TODO: Store tokens in Keychain using KeychainAccess
        // try keychain.set(accessToken, key: "accessToken")
        // try keychain.set(refreshToken, key: "refreshToken")
        self.accessToken = accessToken
        self.refreshToken = refreshToken
    }

    private func clearStoredTokens() {
        // TODO: Remove tokens from Keychain
        // try? keychain.remove("accessToken")
        // try? keychain.remove("refreshToken")
    }
}

// MARK: - Auth Errors

/// Authentication-related errors.
enum AuthError: LocalizedError {
    case invalidCredentials
    case networkError
    case noRefreshToken
    case tokenExpired
    case notImplemented

    var errorDescription: String? {
        switch self {
        case .invalidCredentials:
            return "Invalid email or password"
        case .networkError:
            return "Network error. Please check your connection."
        case .noRefreshToken:
            return "Session expired. Please login again."
        case .tokenExpired:
            return "Session expired. Please login again."
        case .notImplemented:
            return "This feature is not yet implemented."
        }
    }
}
