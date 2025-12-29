import SwiftUI

/// Login screen for Reality Portal iOS app.
///
/// Provides SSO login via Property Management app and email/password login.
///
/// Epic 82 - Story 82.5: Inquiries and Account
struct LoginView: View {
    @Environment(NavigationCoordinator.self) private var coordinator
    @Environment(AuthManager.self) private var authManager
    @Environment(\.dismiss) private var dismiss

    @State private var email = ""
    @State private var password = ""
    @State private var showPassword = false
    @State private var isLoading = false
    @State private var errorMessage: String?

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 32) {
                    // Header
                    headerSection

                    // SSO Login
                    ssoLoginSection

                    // Divider
                    dividerSection

                    // Email/Password Login
                    emailLoginSection

                    // Error message
                    if let error = errorMessage {
                        errorBanner(error)
                    }

                    // Register link
                    registerSection
                }
                .padding()
            }
            .navigationTitle("Sign In")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }

    // MARK: - Subviews

    private var headerSection: some View {
        VStack(spacing: 16) {
            Image(systemName: "house.fill")
                .font(.system(size: 48))
                .foregroundStyle(Color.accentColor)

            Text("Welcome to Reality Portal")
                .font(.title2)
                .fontWeight(.bold)

            Text("Sign in to save favorites, track inquiries, and get personalized recommendations.")
                .font(.subheadline)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)
        }
        .padding(.top, 16)
    }

    private var ssoLoginSection: some View {
        VStack(spacing: 16) {
            Button {
                loginWithSso()
            } label: {
                HStack(spacing: 12) {
                    Image(systemName: "building.2.fill")
                    Text("Sign in with Property Management")
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(Color.accentColor)
                .foregroundStyle(.white)
                .clipShape(RoundedRectangle(cornerRadius: 12))
            }
            .disabled(isLoading)

            Text("Securely sign in using your Property Management account")
                .font(.caption)
                .foregroundStyle(.secondary)
        }
    }

    private var dividerSection: some View {
        HStack {
            Rectangle()
                .fill(Color(.separator))
                .frame(height: 1)

            Text("or")
                .font(.subheadline)
                .foregroundStyle(.secondary)
                .padding(.horizontal, 16)

            Rectangle()
                .fill(Color(.separator))
                .frame(height: 1)
        }
    }

    private var emailLoginSection: some View {
        VStack(spacing: 16) {
            // Email field
            VStack(alignment: .leading, spacing: 8) {
                Text("Email")
                    .font(.subheadline)
                    .fontWeight(.medium)

                TextField("Enter your email", text: $email)
                    .textFieldStyle(.plain)
                    .textContentType(.emailAddress)
                    .keyboardType(.emailAddress)
                    .autocapitalization(.none)
                    .autocorrectionDisabled()
                    .padding()
                    .background(Color(.systemGray6))
                    .clipShape(RoundedRectangle(cornerRadius: 12))
            }

            // Password field
            VStack(alignment: .leading, spacing: 8) {
                Text("Password")
                    .font(.subheadline)
                    .fontWeight(.medium)

                HStack {
                    Group {
                        if showPassword {
                            TextField("Enter your password", text: $password)
                        } else {
                            SecureField("Enter your password", text: $password)
                        }
                    }
                    .textFieldStyle(.plain)
                    .textContentType(.password)
                    .autocapitalization(.none)
                    .autocorrectionDisabled()

                    Button {
                        showPassword.toggle()
                    } label: {
                        Image(systemName: showPassword ? "eye.slash.fill" : "eye.fill")
                            .foregroundStyle(.secondary)
                    }
                }
                .padding()
                .background(Color(.systemGray6))
                .clipShape(RoundedRectangle(cornerRadius: 12))
            }

            // Forgot password
            HStack {
                Spacer()
                Button("Forgot Password?") {
                    // Open forgot password flow
                }
                .font(.subheadline)
            }

            // Login button
            Button {
                Task {
                    await loginWithEmail()
                }
            } label: {
                HStack(spacing: 8) {
                    if isLoading {
                        ProgressView()
                            .tint(.white)
                    }
                    Text("Sign In")
                }
                .frame(maxWidth: .infinity)
                .padding()
                .background(canLogin ? Color.accentColor : Color(.systemGray4))
                .foregroundStyle(.white)
                .clipShape(RoundedRectangle(cornerRadius: 12))
            }
            .disabled(!canLogin || isLoading)
        }
    }

    private func errorBanner(_ message: String) -> some View {
        HStack(spacing: 8) {
            Image(systemName: "exclamationmark.triangle.fill")
                .foregroundStyle(.orange)
            Text(message)
                .font(.subheadline)
            Spacer()
        }
        .padding()
        .background(Color.orange.opacity(0.1))
        .clipShape(RoundedRectangle(cornerRadius: 12))
    }

    private var registerSection: some View {
        HStack(spacing: 4) {
            Text("Don't have an account?")
                .foregroundStyle(.secondary)
            Button("Create Account") {
                coordinator.navigate(to: .register)
            }
        }
        .font(.subheadline)
    }

    // MARK: - Computed Properties

    private var canLogin: Bool {
        !email.isEmpty && !password.isEmpty && email.contains("@")
    }

    // MARK: - Actions

    private func loginWithSso() {
        // Open Property Management app for SSO
        // The app will redirect back with a token via deep link
        if let url = URL(string: "propertymanagement://sso?callback=realityportal://sso") {
            UIApplication.shared.open(url)
        }
    }

    private func loginWithEmail() async {
        isLoading = true
        errorMessage = nil

        do {
            try await authManager.login(email: email, password: password)
            dismiss()

            // Navigate to pending destination if any
            if let pendingDestination = coordinator.pendingDestination {
                coordinator.navigate(to: pendingDestination)
                coordinator.pendingDestination = nil
            }
        } catch {
            errorMessage = error.localizedDescription
        }

        isLoading = false
    }
}

// MARK: - Preview

#Preview {
    LoginView()
        .environment(NavigationCoordinator())
        .environment(AuthManager())
}
