import SwiftUI

/// Home screen for Reality Portal iOS app.
///
/// Displays featured listings, recent listings, and quick search categories.
///
/// Epic 82 - Story 82.3: Home and Search Screens
struct HomeView: View {
    @Environment(NavigationCoordinator.self) private var coordinator
    @Environment(AuthManager.self) private var authManager

    @State private var featuredListings: [ListingPreview] = []
    @State private var recentListings: [ListingPreview] = []
    @State private var isLoading = true
    @State private var errorMessage: String?

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                // Search bar button
                searchBarButton

                // Quick category filters
                categoryFilters

                if isLoading {
                    loadingView
                } else if let error = errorMessage {
                    errorView(message: error)
                } else {
                    // Featured listings carousel
                    if !featuredListings.isEmpty {
                        featuredSection
                    }

                    // Recent listings
                    if !recentListings.isEmpty {
                        recentSection
                    }
                }

                // View all button
                viewAllButton
            }
            .padding(.vertical)
        }
        .navigationTitle("Reality Portal")
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                toolbarContent
            }
        }
        .refreshable {
            await loadData()
        }
        .task {
            await loadData()
        }
    }

    // MARK: - Subviews

    private var searchBarButton: some View {
        Button {
            coordinator.selectedTab = .search
        } label: {
            HStack {
                Image(systemName: "magnifyingglass")
                    .foregroundStyle(.secondary)
                Text("Search for properties...")
                    .foregroundStyle(.secondary)
                Spacer()
            }
            .padding()
            .background(Color(.systemGray6))
            .clipShape(RoundedRectangle(cornerRadius: 12))
        }
        .buttonStyle(.plain)
        .padding(.horizontal)
    }

    private var categoryFilters: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 12) {
                CategoryChip(title: "For Sale", icon: "tag.fill") {
                    // Navigate to search with sale filter
                }
                CategoryChip(title: "For Rent", icon: "house.fill") {
                    // Navigate to search with rent filter
                }
                CategoryChip(title: "Apartments", icon: "building.2.fill") {
                    // Navigate to search with apartment filter
                }
                CategoryChip(title: "Houses", icon: "house.fill") {
                    // Navigate to search with house filter
                }
                CategoryChip(title: "Land", icon: "leaf.fill") {
                    // Navigate to search with land filter
                }
            }
            .padding(.horizontal)
        }
    }

    private var featuredSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            SectionHeader(title: "Featured Properties") {
                coordinator.selectedTab = .search
            }

            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 16) {
                    ForEach(featuredListings) { listing in
                        FeaturedListingCard(listing: listing) {
                            coordinator.navigate(to: .listingDetail(id: listing.id))
                        }
                    }
                }
                .padding(.horizontal)
            }
        }
    }

    private var recentSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            SectionHeader(title: "Recently Added") {
                coordinator.selectedTab = .search
            }

            LazyVStack(spacing: 12) {
                ForEach(recentListings) { listing in
                    ListingRowCard(listing: listing) {
                        coordinator.navigate(to: .listingDetail(id: listing.id))
                    }
                }
            }
            .padding(.horizontal)
        }
    }

    private var viewAllButton: some View {
        Button {
            coordinator.selectedTab = .search
        } label: {
            HStack {
                Image(systemName: "magnifyingglass")
                Text("View All Properties")
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(Color.accentColor)
            .foregroundStyle(.white)
            .clipShape(RoundedRectangle(cornerRadius: 12))
        }
        .padding(.horizontal)
    }

    private var loadingView: some View {
        VStack(spacing: 16) {
            ProgressView()
            Text("Loading listings...")
                .foregroundStyle(.secondary)
        }
        .frame(height: 200)
    }

    private func errorView(message: String) -> some View {
        VStack(spacing: 16) {
            Image(systemName: "exclamationmark.triangle")
                .font(.largeTitle)
                .foregroundStyle(.orange)
            Text(message)
                .foregroundStyle(.secondary)
            Button("Retry") {
                Task { await loadData() }
            }
        }
        .frame(height: 200)
        .padding()
    }

    @ViewBuilder
    private var toolbarContent: some View {
        if authManager.isAuthenticated {
            HStack {
                Button {
                    coordinator.selectedTab = .inquiries
                } label: {
                    Image(systemName: "envelope.fill")
                }

                Button {
                    coordinator.selectedTab = .favorites
                } label: {
                    Image(systemName: "heart.fill")
                }

                Button {
                    coordinator.selectedTab = .account
                } label: {
                    Image(systemName: "person.circle.fill")
                }
            }
        } else {
            Button("Sign In") {
                coordinator.selectedTab = .account
            }
        }
    }

    // MARK: - Data Loading

    private func loadData() async {
        isLoading = true
        errorMessage = nil

        // TODO: Integrate with KMP shared module
        // Placeholder data for now
        do {
            try await Task.sleep(nanoseconds: 500_000_000)

            featuredListings = ListingPreview.sampleFeatured
            recentListings = ListingPreview.sampleRecent
        } catch {
            errorMessage = error.localizedDescription
        }

        isLoading = false
    }
}

// MARK: - Supporting Views

private struct CategoryChip: View {
    let title: String
    let icon: String
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 6) {
                Image(systemName: icon)
                    .font(.caption)
                Text(title)
                    .font(.subheadline)
            }
            .padding(.horizontal, 12)
            .padding(.vertical, 8)
            .background(Color(.systemGray6))
            .clipShape(Capsule())
        }
        .buttonStyle(.plain)
    }
}

private struct SectionHeader: View {
    let title: String
    let action: () -> Void

    var body: some View {
        HStack {
            Text(title)
                .font(.title3)
                .fontWeight(.bold)

            Spacer()

            Button(action: action) {
                HStack(spacing: 4) {
                    Text("See All")
                    Image(systemName: "chevron.right")
                }
                .font(.subheadline)
            }
        }
        .padding(.horizontal)
    }
}

private struct FeaturedListingCard: View {
    let listing: ListingPreview
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(alignment: .leading, spacing: 8) {
                // Image placeholder
                RoundedRectangle(cornerRadius: 12)
                    .fill(Color(.systemGray5))
                    .frame(width: 280, height: 160)
                    .overlay {
                        VStack {
                            Image(systemName: "photo")
                                .font(.largeTitle)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .overlay(alignment: .topLeading) {
                        Text("Featured")
                            .font(.caption)
                            .fontWeight(.semibold)
                            .padding(.horizontal, 8)
                            .padding(.vertical, 4)
                            .background(Color.accentColor)
                            .foregroundStyle(.white)
                            .clipShape(RoundedRectangle(cornerRadius: 4))
                            .padding(8)
                    }

                VStack(alignment: .leading, spacing: 4) {
                    Text(listing.formattedPrice)
                        .font(.title3)
                        .fontWeight(.bold)
                        .foregroundStyle(Color.accentColor)

                    Text(listing.title)
                        .font(.subheadline)
                        .lineLimit(1)
                        .foregroundStyle(.primary)

                    HStack(spacing: 4) {
                        Image(systemName: "location.fill")
                            .font(.caption)
                        Text(listing.location)
                            .font(.caption)
                    }
                    .foregroundStyle(.secondary)

                    HStack(spacing: 12) {
                        if let area = listing.areaSqm {
                            Text("\(area) m2")
                                .font(.caption)
                        }
                        if let rooms = listing.rooms {
                            Text("\(rooms) rooms")
                                .font(.caption)
                        }
                    }
                    .foregroundStyle(.secondary)
                }
                .padding(.horizontal, 4)
            }
            .frame(width: 280)
        }
        .buttonStyle(.plain)
    }
}

private struct ListingRowCard: View {
    let listing: ListingPreview
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 12) {
                // Image placeholder
                RoundedRectangle(cornerRadius: 8)
                    .fill(Color(.systemGray5))
                    .frame(width: 100, height: 80)
                    .overlay {
                        Image(systemName: "photo")
                            .foregroundStyle(.secondary)
                    }

                VStack(alignment: .leading, spacing: 4) {
                    Text(listing.formattedPrice)
                        .font(.headline)
                        .foregroundStyle(Color.accentColor)

                    Text(listing.title)
                        .font(.subheadline)
                        .lineLimit(1)
                        .foregroundStyle(.primary)

                    HStack(spacing: 4) {
                        Image(systemName: "location.fill")
                            .font(.caption)
                        Text(listing.location)
                            .font(.caption)
                    }
                    .foregroundStyle(.secondary)
                }

                Spacer()

                Image(systemName: "chevron.right")
                    .foregroundStyle(.tertiary)
            }
            .padding()
            .background(Color(.systemBackground))
            .clipShape(RoundedRectangle(cornerRadius: 12))
            .shadow(color: .black.opacity(0.05), radius: 4, y: 2)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Preview Data

/// Preview model for listings.
struct ListingPreview: Identifiable {
    let id: String
    let title: String
    let price: Int
    let currency: String
    let location: String
    let areaSqm: Int?
    let rooms: Int?

    var formattedPrice: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = currency
        formatter.maximumFractionDigits = 0
        return formatter.string(from: NSNumber(value: price)) ?? "\(price) \(currency)"
    }

    static let sampleFeatured: [ListingPreview] = [
        ListingPreview(id: "1", title: "Modern Apartment in City Center", price: 250000, currency: "EUR", location: "Bratislava", areaSqm: 85, rooms: 3),
        ListingPreview(id: "2", title: "Family House with Garden", price: 450000, currency: "EUR", location: "Bratislava", areaSqm: 180, rooms: 5),
        ListingPreview(id: "3", title: "Cozy Studio Near Park", price: 120000, currency: "EUR", location: "Kosice", areaSqm: 35, rooms: 1),
    ]

    static let sampleRecent: [ListingPreview] = [
        ListingPreview(id: "4", title: "Renovated Apartment", price: 180000, currency: "EUR", location: "Bratislava", areaSqm: 65, rooms: 2),
        ListingPreview(id: "5", title: "Penthouse with Terrace", price: 520000, currency: "EUR", location: "Bratislava", areaSqm: 120, rooms: 4),
        ListingPreview(id: "6", title: "Investment Property", price: 95000, currency: "EUR", location: "Zilina", areaSqm: 45, rooms: 2),
    ]
}

// MARK: - Preview

#Preview {
    NavigationStack {
        HomeView()
    }
    .environment(NavigationCoordinator())
    .environment(AuthManager())
}
