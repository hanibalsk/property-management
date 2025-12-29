import Foundation
import Observation
import Security
import shared

/// User information from authentication.
///
/// Epic 82 - Story 82.5: Inquiries and Account
struct User: Identifiable, Equatable {
    let id: String
    let email: String
    let name: String
    let avatarUrl: String?

    /// Initialize from KMP SsoUserInfo.
    init(from ssoUser: SsoUserInfo) {
        self.id = ssoUser.userId
        self.email = ssoUser.email
        self.name = ssoUser.name
        self.avatarUrl = ssoUser.avatarUrl
    }

    /// Initialize with explicit values.
    init(id: String, email: String, name: String, avatarUrl: String? = nil) {
        self.id = id
        self.email = email
        self.name = name
        self.avatarUrl = avatarUrl
    }
}

/// Authentication state manager for Reality Portal iOS app.
///
/// Epic 82 - Story 82.5: Inquiries and Account
/// Integrates with KMP SsoService for SSO authentication and uses iOS Keychain for token storage.
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
    private let ssoService = SsoService()

    // Keychain keys
    private let accessTokenKey = "reality_portal_access_token"
    private let refreshTokenKey = "reality_portal_refresh_token"

    // MARK: - Initialization

    init() {
        // Session will be restored when restoreSession() is called
    }

    // MARK: - Public Methods

    /// Login with email and password.
    /// - Parameters:
    ///   - email: User email address.
    ///   - password: User password.
    /// - Note: Reality Portal uses SSO from Property Management app. Direct login is not supported.
    func login(email: String, password: String) async throws {
        isLoading = true
        errorMessage = nil

        defer { isLoading = false }

        // Reality Portal uses SSO from Property Management app
        // Direct email/password login is not supported
        throw AuthError.ssoRequired
    }

    /// Login via SSO token from Property Management app.
    /// - Parameter token: SSO token received via deep link.
    func loginWithSsoToken(_ token: String) async throws {
        isLoading = true
        errorMessage = nil

        defer { isLoading = false }

        let result = try await withCheckedThrowingContinuation { continuation in
            Task {
                let validationResult = await ssoService.validateAndLogin(ssoToken: token)
                if let session = validationResult.getOrNull() {
                    continuation.resume(returning: session)
                } else if let error = validationResult.exceptionOrNull() {
                    continuation.resume(throwing: AuthError.ssoValidationFailed(error.message ?? "Unknown error"))
                } else {
                    continuation.resume(throwing: AuthError.ssoValidationFailed("Unknown error"))
                }
            }
        }

        // Store the session token
        let sessionToken = result.sessionToken
        storeTokens(accessToken: sessionToken, refreshToken: "")

        // Set current user from SSO response
        currentUser = User(from: result.user)
        accessToken = sessionToken
    }

    /// Logout the current user.
    func logout() {
        ssoService.logout()
        accessToken = nil
        refreshToken = nil
        currentUser = nil

        // Clear from secure storage
        clearStoredTokens()
    }

    /// Restore session from stored tokens.
    func restoreSession() {
        // Load tokens from Keychain
        guard let storedToken = loadFromKeychain(key: accessTokenKey) else {
            return
        }

        accessToken = storedToken
        refreshToken = loadFromKeychain(key: refreshTokenKey)

        // Validate and restore the session using KMP SsoService
        Task {
            await loadCurrentUser()
        }
    }

    /// Refresh the access token using the refresh token.
    func refreshAccessToken() async throws {
        guard accessToken != nil else {
            throw AuthError.noRefreshToken
        }

        isLoading = true
        defer { isLoading = false }

        let result = try await withCheckedThrowingContinuation { continuation in
            Task {
                let refreshResult = await ssoService.refreshSession()
                if let session = refreshResult.getOrNull() {
                    continuation.resume(returning: session)
                } else if let error = refreshResult.exceptionOrNull() {
                    continuation.resume(throwing: AuthError.tokenExpired)
                } else {
                    continuation.resume(throwing: AuthError.tokenExpired)
                }
            }
        }

        // Update user info from refreshed session
        currentUser = User(
            id: result.userId,
            email: result.email,
            name: result.name,
            avatarUrl: nil
        )
    }

    /// Get the current session token for API calls.
    func getSessionToken() -> String? {
        return ssoService.getSessionToken()
    }

    // MARK: - Private Methods

    private func loadCurrentUser() async {
        guard let token = accessToken else { return }

        isLoading = true
        defer { isLoading = false }

        // Try to restore the session with stored token
        let success = await ssoService.restoreSession(token: token)

        if success {
            // Get session info
            let result = await ssoService.getSession()
            if let session = result.getOrNull() {
                currentUser = User(
                    id: session.userId,
                    email: session.email,
                    name: session.name,
                    avatarUrl: nil
                )
            }
        } else {
            // Token is invalid, clear stored credentials
            clearStoredTokens()
            accessToken = nil
            refreshToken = nil
            currentUser = nil
        }
    }

    private func storeTokens(accessToken: String, refreshToken: String) {
        self.accessToken = accessToken
        self.refreshToken = refreshToken

        // Store in Keychain
        saveToKeychain(key: accessTokenKey, value: accessToken)
        if !refreshToken.isEmpty {
            saveToKeychain(key: refreshTokenKey, value: refreshToken)
        }
    }

    private func clearStoredTokens() {
        deleteFromKeychain(key: accessTokenKey)
        deleteFromKeychain(key: refreshTokenKey)
    }

    // MARK: - Keychain Operations

    private func saveToKeychain(key: String, value: String) {
        guard let data = value.data(using: .utf8) else { return }

        // Delete existing item first
        deleteFromKeychain(key: key)

        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: configuration.keychainService,
            kSecAttrAccount as String: key,
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly
        ]

        let status = SecItemAdd(query as CFDictionary, nil)
        if status != errSecSuccess {
            #if DEBUG
            print("Keychain save failed for key \(key): \(status)")
            #endif
        }
    }

    private func loadFromKeychain(key: String) -> String? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: configuration.keychainService,
            kSecAttrAccount as String: key,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]

        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)

        guard status == errSecSuccess,
              let data = result as? Data,
              let value = String(data: data, encoding: .utf8) else {
            #if DEBUG
            // Log non-expected errors for debugging (errSecItemNotFound is expected when key doesn't exist)
            if status != errSecSuccess && status != errSecItemNotFound {
                print("Keychain load failed for key \(key): \(status)")
            }
            #endif
            return nil
        }

        return value
    }

    private func deleteFromKeychain(key: String) {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: configuration.keychainService,
            kSecAttrAccount as String: key
        ]

        SecItemDelete(query as CFDictionary)
    }
}

// MARK: - Auth Errors

/// Authentication-related errors.
enum AuthError: LocalizedError {
    case invalidCredentials
    case networkError
    case noRefreshToken
    case tokenExpired
    case ssoRequired
    case ssoValidationFailed(String)
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
        case .ssoRequired:
            return "Please use the Property Management app to sign in."
        case .ssoValidationFailed(let message):
            return "SSO login failed: \(message)"
        case .notImplemented:
            return "This feature is not yet implemented."
        }
    }
}
