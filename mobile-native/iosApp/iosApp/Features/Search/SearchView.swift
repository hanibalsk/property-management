import SwiftUI

/// Search screen for Reality Portal iOS app.
///
/// Provides property search with filters and results.
///
/// Epic 82 - Story 82.3: Home and Search Screens
struct SearchView: View {
    @Environment(NavigationCoordinator.self) private var coordinator

    @State private var searchText = ""
    @State private var results: [ListingPreview] = []
    @State private var isLoading = false
    @State private var showFilters = false
    @State private var filters = SearchFilters()

    var body: some View {
        VStack(spacing: 0) {
            // Search bar
            searchBar

            // Filter bar
            filterBar

            // Results
            if isLoading {
                loadingView
            } else if results.isEmpty && !searchText.isEmpty {
                emptyResultsView
            } else if results.isEmpty {
                searchPromptView
            } else {
                resultsGrid
            }
        }
        .navigationTitle("Search")
        .navigationBarTitleDisplayMode(.inline)
        .sheet(isPresented: $showFilters) {
            FilterSheet(filters: $filters) {
                Task { await performSearch() }
            }
        }
        .onChange(of: searchText) { _, newValue in
            // Debounced search
            Task {
                try? await Task.sleep(nanoseconds: 300_000_000)
                if searchText == newValue {
                    await performSearch()
                }
            }
        }
    }

    // MARK: - Subviews

    private var searchBar: some View {
        HStack(spacing: 12) {
            HStack {
                Image(systemName: "magnifyingglass")
                    .foregroundStyle(.secondary)

                TextField("Search properties...", text: $searchText)
                    .textFieldStyle(.plain)

                if !searchText.isEmpty {
                    Button {
                        searchText = ""
                    } label: {
                        Image(systemName: "xmark.circle.fill")
                            .foregroundStyle(.secondary)
                    }
                }
            }
            .padding(12)
            .background(Color(.systemGray6))
            .clipShape(RoundedRectangle(cornerRadius: 12))
        }
        .padding()
    }

    private var filterBar: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 8) {
                // Filter button
                Button {
                    showFilters = true
                } label: {
                    HStack(spacing: 4) {
                        Image(systemName: "slider.horizontal.3")
                        Text("Filters")
                        if filters.hasActiveFilters {
                            Circle()
                                .fill(Color.accentColor)
                                .frame(width: 8, height: 8)
                        }
                    }
                    .font(.subheadline)
                    .padding(.horizontal, 12)
                    .padding(.vertical, 8)
                    .background(filters.hasActiveFilters ? Color.accentColor.opacity(0.1) : Color(.systemGray6))
                    .clipShape(Capsule())
                }

                // Quick filter chips
                ForEach(PropertyType.allCases, id: \.self) { type in
                    FilterChip(
                        title: type.displayName,
                        isSelected: filters.propertyTypes.contains(type)
                    ) {
                        if filters.propertyTypes.contains(type) {
                            filters.propertyTypes.remove(type)
                        } else {
                            filters.propertyTypes.insert(type)
                        }
                        Task { await performSearch() }
                    }
                }
            }
            .padding(.horizontal)
        }
        .padding(.bottom, 8)
    }

    private var loadingView: some View {
        VStack {
            Spacer()
            ProgressView()
            Text("Searching...")
                .foregroundStyle(.secondary)
                .padding(.top, 8)
            Spacer()
        }
    }

    private var emptyResultsView: some View {
        VStack(spacing: 16) {
            Spacer()
            Image(systemName: "magnifyingglass")
                .font(.system(size: 48))
                .foregroundStyle(.secondary)
            Text("No results found")
                .font(.headline)
            Text("Try adjusting your search or filters")
                .font(.subheadline)
                .foregroundStyle(.secondary)
            Spacer()
        }
    }

    private var searchPromptView: some View {
        VStack(spacing: 16) {
            Spacer()
            Image(systemName: "house.fill")
                .font(.system(size: 48))
                .foregroundStyle(.secondary)
            Text("Search for properties")
                .font(.headline)
            Text("Enter a location, keyword, or use filters")
                .font(.subheadline)
                .foregroundStyle(.secondary)

            // Recent searches placeholder
            VStack(alignment: .leading, spacing: 8) {
                Text("Recent Searches")
                    .font(.subheadline)
                    .fontWeight(.semibold)
                    .foregroundStyle(.secondary)

                ForEach(["Bratislava apartments", "Houses for sale", "3 bedroom"], id: \.self) { term in
                    Button {
                        searchText = term
                    } label: {
                        HStack {
                            Image(systemName: "clock.arrow.circlepath")
                                .foregroundStyle(.secondary)
                            Text(term)
                                .foregroundStyle(.primary)
                            Spacer()
                        }
                        .padding(.vertical, 8)
                    }
                }
            }
            .padding()
            .background(Color(.systemGray6))
            .clipShape(RoundedRectangle(cornerRadius: 12))
            .padding(.horizontal)

            Spacer()
        }
    }

    private var resultsGrid: some View {
        ScrollView {
            LazyVStack(spacing: 12) {
                // Results count
                HStack {
                    Text("\(results.count) properties found")
                        .font(.subheadline)
                        .foregroundStyle(.secondary)
                    Spacer()
                }
                .padding(.horizontal)

                ForEach(results) { listing in
                    SearchResultCard(listing: listing) {
                        coordinator.navigate(to: .listingDetail(id: listing.id))
                    }
                }
            }
            .padding(.vertical)
        }
    }

    // MARK: - Search

    private func performSearch() async {
        guard !searchText.isEmpty || filters.hasActiveFilters else {
            results = []
            return
        }

        isLoading = true

        // TODO: Integrate with KMP search use case
        // Placeholder implementation
        try? await Task.sleep(nanoseconds: 500_000_000)

        results = ListingPreview.sampleFeatured + ListingPreview.sampleRecent

        isLoading = false
    }
}

// MARK: - Supporting Views

private struct FilterChip: View {
    let title: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.subheadline)
                .padding(.horizontal, 12)
                .padding(.vertical, 8)
                .background(isSelected ? Color.accentColor.opacity(0.1) : Color(.systemGray6))
                .foregroundStyle(isSelected ? Color.accentColor : .primary)
                .clipShape(Capsule())
                .overlay {
                    Capsule()
                        .stroke(isSelected ? Color.accentColor : Color.clear, lineWidth: 1)
                }
        }
        .buttonStyle(.plain)
    }
}

private struct SearchResultCard: View {
    let listing: ListingPreview
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(alignment: .leading, spacing: 0) {
                // Image placeholder
                RoundedRectangle(cornerRadius: 12)
                    .fill(Color(.systemGray5))
                    .frame(height: 180)
                    .overlay {
                        Image(systemName: "photo")
                            .font(.largeTitle)
                            .foregroundStyle(.secondary)
                    }
                    .overlay(alignment: .topTrailing) {
                        Button {
                            // Toggle favorite
                        } label: {
                            Image(systemName: "heart")
                                .font(.title3)
                                .padding(8)
                                .background(.ultraThinMaterial)
                                .clipShape(Circle())
                        }
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
                            HStack(spacing: 2) {
                                Image(systemName: "square.fill")
                                    .font(.caption2)
                                Text("\(area) m2")
                            }
                        }
                        if let rooms = listing.rooms {
                            HStack(spacing: 2) {
                                Image(systemName: "bed.double.fill")
                                    .font(.caption2)
                                Text("\(rooms)")
                            }
                        }
                    }
                    .font(.caption)
                    .foregroundStyle(.secondary)
                }
                .padding(12)
            }
            .background(Color(.systemBackground))
            .clipShape(RoundedRectangle(cornerRadius: 12))
            .shadow(color: .black.opacity(0.05), radius: 4, y: 2)
        }
        .buttonStyle(.plain)
        .padding(.horizontal)
    }
}

// MARK: - Filter Sheet

private struct FilterSheet: View {
    @Binding var filters: SearchFilters
    let onApply: () -> Void
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        NavigationStack {
            Form {
                Section("Price Range") {
                    HStack {
                        TextField("Min", value: $filters.priceMin, format: .number)
                            .textFieldStyle(.roundedBorder)
                            .keyboardType(.numberPad)

                        Text("-")

                        TextField("Max", value: $filters.priceMax, format: .number)
                            .textFieldStyle(.roundedBorder)
                            .keyboardType(.numberPad)
                    }
                }

                Section("Property Type") {
                    ForEach(PropertyType.allCases, id: \.self) { type in
                        Toggle(type.displayName, isOn: Binding(
                            get: { filters.propertyTypes.contains(type) },
                            set: { isOn in
                                if isOn {
                                    filters.propertyTypes.insert(type)
                                } else {
                                    filters.propertyTypes.remove(type)
                                }
                            }
                        ))
                    }
                }

                Section("Bedrooms") {
                    Stepper("Min: \(filters.bedroomsMin ?? 0)", value: Binding(
                        get: { filters.bedroomsMin ?? 0 },
                        set: { filters.bedroomsMin = $0 > 0 ? $0 : nil }
                    ), in: 0...10)
                }

                Section {
                    Button("Reset Filters") {
                        filters.reset()
                    }
                    .foregroundStyle(.red)
                }
            }
            .navigationTitle("Filters")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Apply") {
                        onApply()
                        dismiss()
                    }
                }
            }
        }
    }
}

// MARK: - Preview

#Preview {
    NavigationStack {
        SearchView()
    }
    .environment(NavigationCoordinator())
}
