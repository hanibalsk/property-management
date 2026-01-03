import SwiftUI

/// Account screen for Reality Portal iOS app.
///
/// Displays user profile, settings, and sign in/out options.
///
/// Epic 82 - Story 82.5: Inquiries and Account
struct AccountView: View {
    @Environment(NavigationCoordinator.self) private var coordinator
    @Environment(AuthManager.self) private var authManager

    @State private var showLogoutConfirmation = false

    var body: some View {
        Group {
            if authManager.isAuthenticated, let user = authManager.currentUser {
                authenticatedView(user: user)
            } else {
                notAuthenticatedView
            }
        }
        .navigationTitle(String(localized: "tab_account"))
        .alert(String(localized: "sign_out"), isPresented: $showLogoutConfirmation) {
            Button(String(localized: "cancel"), role: .cancel) {}
            Button(String(localized: "sign_out"), role: .destructive) {
                authManager.logout()
            }
        } message: {
            Text(String(localized: "confirm_sign_out_message"))
        }
    }

    // MARK: - Authenticated View

    private func authenticatedView(user: User) -> some View {
        List {
            // Profile section
            Section {
                HStack(spacing: 16) {
                    // Avatar placeholder
                    Circle()
                        .fill(Color(.systemGray4))
                        .frame(width: 72, height: 72)
                        .overlay {
                            Text(user.name.prefix(1).uppercased())
                                .font(.title)
                                .fontWeight(.semibold)
                                .foregroundStyle(.secondary)
                        }

                    VStack(alignment: .leading, spacing: 4) {
                        Text(user.name)
                            .font(.title3)
                            .fontWeight(.bold)

                        Text(user.email)
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                    }

                    Spacer()

                    Button {
                        coordinator.navigate(to: .profile)
                    } label: {
                        Image(systemName: "pencil.circle.fill")
                            .font(.title2)
                            .foregroundStyle(.secondary)
                    }
                }
                .padding(.vertical, 8)
            }

            // Quick actions section
            Section(String(localized: "quick_actions")) {
                NavigationLink {
                    Text(String(localized: "my_favorites"))
                } label: {
                    Label(String(localized: "my_favorites"), systemImage: "heart.fill")
                }

                NavigationLink {
                    Text(String(localized: "my_inquiries"))
                } label: {
                    Label(String(localized: "my_inquiries"), systemImage: "envelope.fill")
                }

                NavigationLink {
                    Text(String(localized: "saved_searches"))
                } label: {
                    Label(String(localized: "saved_searches"), systemImage: "bookmark.fill")
                }
            }

            // Notifications section
            Section(String(localized: "notifications")) {
                Toggle(isOn: .constant(true)) {
                    Label(String(localized: "notification_new_listings"), systemImage: "bell.badge.fill")
                }

                Toggle(isOn: .constant(true)) {
                    Label(String(localized: "notification_price_drops"), systemImage: "tag.fill")
                }

                Toggle(isOn: .constant(true)) {
                    Label(String(localized: "notification_inquiry_responses"), systemImage: "message.fill")
                }

                Toggle(isOn: .constant(false)) {
                    Label(String(localized: "notification_marketing"), systemImage: "megaphone.fill")
                }
            }

            // App settings section
            Section(String(localized: "section_app_settings")) {
                NavigationLink {
                    Text(String(localized: "setting_language"))
                } label: {
                    HStack {
                        Label(String(localized: "setting_language"), systemImage: "globe")
                        Spacer()
                        Text(String(localized: "language_english"))
                            .foregroundStyle(.secondary)
                    }
                }

                NavigationLink {
                    Text(String(localized: "setting_currency"))
                } label: {
                    HStack {
                        Label(String(localized: "setting_currency"), systemImage: "eurosign.circle.fill")
                        Spacer()
                        Text(String(localized: "currency_eur"))
                            .foregroundStyle(.secondary)
                    }
                }

                NavigationLink {
                    Text(String(localized: "setting_units"))
                } label: {
                    HStack {
                        Label(String(localized: "setting_units"), systemImage: "ruler.fill")
                        Spacer()
                        Text(String(localized: "units_metric"))
                            .foregroundStyle(.secondary)
                    }
                }

                NavigationLink {
                    Text(String(localized: "setting_theme"))
                } label: {
                    HStack {
                        Label(String(localized: "setting_theme"), systemImage: "moon.fill")
                        Spacer()
                        Text(String(localized: "theme_system"))
                            .foregroundStyle(.secondary)
                    }
                }
            }

            // About section
            Section(String(localized: "about")) {
                HStack {
                    Label(String(localized: "version"), systemImage: "info.circle.fill")
                    Spacer()
                    Text(Configuration.shared.version)
                        .foregroundStyle(.secondary)
                }

                NavigationLink {
                    Text(String(localized: "terms_of_service"))
                } label: {
                    Label(String(localized: "terms_of_service"), systemImage: "doc.text.fill")
                }

                NavigationLink {
                    Text(String(localized: "privacy_policy"))
                } label: {
                    Label(String(localized: "privacy_policy"), systemImage: "lock.fill")
                }

                NavigationLink {
                    Text(String(localized: "help_support"))
                } label: {
                    Label(String(localized: "help_support"), systemImage: "questionmark.circle.fill")
                }

                NavigationLink {
                    Text(String(localized: "send_feedback"))
                } label: {
                    Label(String(localized: "send_feedback"), systemImage: "envelope.fill")
                }
            }

            // Sign out section
            Section {
                Button(role: .destructive) {
                    showLogoutConfirmation = true
                } label: {
                    HStack {
                        Spacer()
                        Label(String(localized: "sign_out"), systemImage: "rectangle.portrait.and.arrow.right")
                        Spacer()
                    }
                }
            }
        }
    }

    // MARK: - Not Authenticated View

    private var notAuthenticatedView: some View {
        VStack(spacing: 32) {
            Spacer()

            // Icon
            Image(systemName: "person.circle.fill")
                .font(.system(size: 80))
                .foregroundStyle(.secondary)

            // Title and description
            VStack(spacing: 8) {
                Text(String(localized: "sign_in_title"))
                    .font(.title2)
                    .fontWeight(.bold)

                Text(String(localized: "sign_in_benefits"))
                    .font(.subheadline)
                    .foregroundStyle(.secondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal, 24)
            }

            // Sign in button
            VStack(spacing: 16) {
                Button {
                    coordinator.navigate(to: .login)
                } label: {
                    HStack {
                        Image(systemName: "person.fill")
                        Text(String(localized: "sign_in_pm_app"))
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.accentColor)
                    .foregroundStyle(.white)
                    .clipShape(RoundedRectangle(cornerRadius: 12))
                }

                Text(String(localized: "sign_in_redirect_notice"))
                    .font(.caption)
                    .foregroundStyle(.secondary)
                    .multilineTextAlignment(.center)
            }
            .padding(.horizontal, 32)

            Spacer()

            // About section for non-authenticated users
            VStack(spacing: 16) {
                Divider()

                HStack(spacing: 24) {
                    Button(String(localized: "terms")) {
                        // Open terms
                    }
                    .font(.subheadline)

                    Button(String(localized: "privacy")) {
                        // Open privacy
                    }
                    .font(.subheadline)

                    Button(String(localized: "help")) {
                        // Open help
                    }
                    .font(.subheadline)
                }
                .foregroundStyle(.secondary)

                Text(String(format: String(localized: "version_format"), Configuration.shared.version))
                    .font(.caption)
                    .foregroundStyle(.tertiary)
            }
            .padding()
        }
    }
}

// MARK: - Preview

#Preview("Authenticated") {
    NavigationStack {
        AccountView()
    }
    .environment(NavigationCoordinator())
    .environment({
        let auth = AuthManager()
        // Note: In real implementation, we would set up test user
        return auth
    }())
}

#Preview("Not Authenticated") {
    NavigationStack {
        AccountView()
    }
    .environment(NavigationCoordinator())
    .environment(AuthManager())
}
