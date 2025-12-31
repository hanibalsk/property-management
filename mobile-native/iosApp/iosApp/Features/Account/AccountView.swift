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
        .navigationTitle("Account")
        .alert("Sign Out", isPresented: $showLogoutConfirmation) {
            Button("Cancel", role: .cancel) {}
            Button("Sign Out", role: .destructive) {
                authManager.logout()
            }
        } message: {
            Text("Are you sure you want to sign out?")
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
            Section("Quick Actions") {
                NavigationLink {
                    Text("My Favorites")
                } label: {
                    Label("My Favorites", systemImage: "heart.fill")
                }

                NavigationLink {
                    Text("My Inquiries")
                } label: {
                    Label("My Inquiries", systemImage: "envelope.fill")
                }

                NavigationLink {
                    Text("Saved Searches")
                } label: {
                    Label("Saved Searches", systemImage: "bookmark.fill")
                }
            }

            // Notifications section
            Section("Notifications") {
                Toggle(isOn: .constant(true)) {
                    Label("New Listings", systemImage: "bell.badge.fill")
                }

                Toggle(isOn: .constant(true)) {
                    Label("Price Drops", systemImage: "tag.fill")
                }

                Toggle(isOn: .constant(true)) {
                    Label("Inquiry Responses", systemImage: "message.fill")
                }

                Toggle(isOn: .constant(false)) {
                    Label("Marketing", systemImage: "megaphone.fill")
                }
            }

            // App settings section
            Section("App Settings") {
                NavigationLink {
                    Text("Language Settings")
                } label: {
                    HStack {
                        Label("Language", systemImage: "globe")
                        Spacer()
                        Text("English")
                            .foregroundStyle(.secondary)
                    }
                }

                NavigationLink {
                    Text("Currency Settings")
                } label: {
                    HStack {
                        Label("Currency", systemImage: "eurosign.circle.fill")
                        Spacer()
                        Text("EUR")
                            .foregroundStyle(.secondary)
                    }
                }

                NavigationLink {
                    Text("Units Settings")
                } label: {
                    HStack {
                        Label("Units", systemImage: "ruler.fill")
                        Spacer()
                        Text("Metric (m2)")
                            .foregroundStyle(.secondary)
                    }
                }

                NavigationLink {
                    Text("Theme Settings")
                } label: {
                    HStack {
                        Label("Theme", systemImage: "moon.fill")
                        Spacer()
                        Text("System")
                            .foregroundStyle(.secondary)
                    }
                }
            }

            // About section
            Section("About") {
                HStack {
                    Label("Version", systemImage: "info.circle.fill")
                    Spacer()
                    Text(Configuration.shared.version)
                        .foregroundStyle(.secondary)
                }

                NavigationLink {
                    Text("Terms of Service")
                } label: {
                    Label("Terms of Service", systemImage: "doc.text.fill")
                }

                NavigationLink {
                    Text("Privacy Policy")
                } label: {
                    Label("Privacy Policy", systemImage: "lock.fill")
                }

                NavigationLink {
                    Text("Help & Support")
                } label: {
                    Label("Help & Support", systemImage: "questionmark.circle.fill")
                }

                NavigationLink {
                    Text("Send Feedback")
                } label: {
                    Label("Send Feedback", systemImage: "envelope.fill")
                }
            }

            // Sign out section
            Section {
                Button(role: .destructive) {
                    showLogoutConfirmation = true
                } label: {
                    HStack {
                        Spacer()
                        Label("Sign Out", systemImage: "rectangle.portrait.and.arrow.right")
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
                Text("Sign in to Reality Portal")
                    .font(.title2)
                    .fontWeight(.bold)

                Text("Save favorites, track inquiries, and get notified about new listings")
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
                        Text("Sign In via PM App")
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.accentColor)
                    .foregroundStyle(.white)
                    .clipShape(RoundedRectangle(cornerRadius: 12))
                }

                Text("You'll be redirected to the Property Management app to sign in securely.")
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
                    Button("Terms") {
                        // Open terms
                    }
                    .font(.subheadline)

                    Button("Privacy") {
                        // Open privacy
                    }
                    .font(.subheadline)

                    Button("Help") {
                        // Open help
                    }
                    .font(.subheadline)
                }
                .foregroundStyle(.secondary)

                Text("Version \(Configuration.shared.version)")
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
