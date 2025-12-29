import SwiftUI

/// Main entry point for Reality Portal iOS app.
///
/// Epic 82 - Story 82.1: SwiftUI Project Setup
@main
struct RealityPortalApp: App {
    // MARK: - State Objects

    @State private var navigationCoordinator = NavigationCoordinator()
    @State private var authManager = AuthManager()

    // MARK: - App Body

    var body: some Scene {
        WindowGroup {
            MainTabView()
                .environment(navigationCoordinator)
                .environment(authManager)
                .onOpenURL { url in
                    handleDeepLink(url)
                }
                .onAppear {
                    configureApp()
                }
        }
    }

    // MARK: - Configuration

    private func configureApp() {
        // Log configuration for debugging
        #if DEBUG
        print("Reality Portal iOS App")
        print("Environment: \(Configuration.shared.environment.rawValue)")
        print("API Base URL: \(Configuration.shared.apiBaseUrl)")
        #endif

        // Restore user session if available
        authManager.restoreSession()
    }

    // MARK: - Deep Link Handling

    private func handleDeepLink(_ url: URL) {
        // Handle SSO callback separately
        if url.host == "sso" {
            handleSsoCallback(url)
            return
        }

        // Handle navigation deep links
        navigationCoordinator.handleDeepLink(url)
    }

    private func handleSsoCallback(_ url: URL) {
        guard let components = URLComponents(url: url, resolvingAgainstBaseURL: true),
              let token = components.queryItems?.first(where: { $0.name == "token" })?.value else {
            return
        }

        Task {
            do {
                try await authManager.loginWithSsoToken(token)

                // Navigate to pending destination if any
                if let pendingDestination = navigationCoordinator.pendingDestination {
                    navigationCoordinator.navigate(to: pendingDestination)
                    navigationCoordinator.pendingDestination = nil
                }
            } catch {
                // Handle SSO login error
                print("SSO login failed: \(error.localizedDescription)")
            }
        }
    }
}
