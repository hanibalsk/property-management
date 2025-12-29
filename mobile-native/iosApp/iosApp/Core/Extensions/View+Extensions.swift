import SwiftUI

/// SwiftUI View extensions for Reality Portal iOS app.
///
/// Epic 82 - Story 82.1: SwiftUI Project Setup
extension View {
    /// Apply a conditional modifier.
    /// - Parameters:
    ///   - condition: The condition to check.
    ///   - transform: The modifier to apply if condition is true.
    @ViewBuilder
    func `if`<Transform: View>(
        _ condition: Bool,
        transform: (Self) -> Transform
    ) -> some View {
        if condition {
            transform(self)
        } else {
            self
        }
    }

    /// Apply a conditional modifier with an else case.
    /// - Parameters:
    ///   - condition: The condition to check.
    ///   - ifTransform: The modifier to apply if condition is true.
    ///   - elseTransform: The modifier to apply if condition is false.
    @ViewBuilder
    func `if`<TrueContent: View, FalseContent: View>(
        _ condition: Bool,
        if ifTransform: (Self) -> TrueContent,
        else elseTransform: (Self) -> FalseContent
    ) -> some View {
        if condition {
            ifTransform(self)
        } else {
            elseTransform(self)
        }
    }

    /// Apply a modifier with an optional value.
    /// - Parameters:
    ///   - value: The optional value.
    ///   - transform: The modifier to apply if value is not nil.
    @ViewBuilder
    func ifLet<Value, Transform: View>(
        _ value: Value?,
        transform: (Self, Value) -> Transform
    ) -> some View {
        if let value = value {
            transform(self, value)
        } else {
            self
        }
    }

    /// Hide the view based on a condition.
    /// - Parameter hidden: Whether to hide the view.
    @ViewBuilder
    func hidden(_ hidden: Bool) -> some View {
        if hidden {
            self.hidden()
        } else {
            self
        }
    }

    /// Add a loading overlay to the view.
    /// - Parameter isLoading: Whether to show the loading overlay.
    func loadingOverlay(_ isLoading: Bool) -> some View {
        self.overlay {
            if isLoading {
                ZStack {
                    Color.black.opacity(0.3)
                        .ignoresSafeArea()
                    ProgressView()
                        .scaleEffect(1.5)
                        .tint(.white)
                }
            }
        }
    }

    /// Add error alert to the view.
    /// - Parameters:
    ///   - error: Binding to the optional error message.
    func errorAlert(_ error: Binding<String?>) -> some View {
        self.alert(
            "Error",
            isPresented: .init(
                get: { error.wrappedValue != nil },
                set: { if !$0 { error.wrappedValue = nil } }
            )
        ) {
            Button("OK", role: .cancel) {
                error.wrappedValue = nil
            }
        } message: {
            if let message = error.wrappedValue {
                Text(message)
            }
        }
    }
}

// MARK: - Preview Helpers

#if DEBUG
extension View {
    /// Wrap view in a preview with environment objects.
    func previewWithEnvironment() -> some View {
        self
            .environment(NavigationCoordinator())
            .environment(AuthManager())
    }
}
#endif
