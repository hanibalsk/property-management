import SwiftUI

/// Listing detail screen for Reality Portal iOS app.
///
/// Displays full property information, photos, and contact options.
///
/// Epic 82 - Story 82.4: Listing Detail and Favorites
struct ListingDetailView: View {
    @Environment(NavigationCoordinator.self) private var coordinator
    @Environment(AuthManager.self) private var authManager
    @Environment(\.dismiss) private var dismiss

    let listingId: String

    @State private var listing: ListingDetail?
    @State private var isLoading = true
    @State private var isFavorite = false
    @State private var showGallery = false
    @State private var showInquirySheet = false
    @State private var errorMessage: String?

    var body: some View {
        Group {
            if isLoading {
                loadingView
            } else if let listing = listing {
                listingContent(listing)
            } else {
                errorView
            }
        }
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                HStack(spacing: 16) {
                    favoriteButton
                    shareButton
                }
            }
        }
        .sheet(isPresented: $showInquirySheet) {
            NewInquirySheet(listingId: listingId)
        }
        .fullScreenCover(isPresented: $showGallery) {
            PhotoGalleryView(photos: listing?.photos ?? [])
        }
        .task {
            await loadListing()
        }
    }

    // MARK: - Subviews

    private var loadingView: some View {
        VStack(spacing: 16) {
            ProgressView()
            Text("Loading listing...")
                .foregroundStyle(.secondary)
        }
    }

    private var errorView: some View {
        VStack(spacing: 16) {
            Image(systemName: "exclamationmark.triangle")
                .font(.largeTitle)
                .foregroundStyle(.orange)
            Text(errorMessage ?? "Failed to load listing")
                .foregroundStyle(.secondary)
            Button("Retry") {
                Task { await loadListing() }
            }
        }
    }

    private func listingContent(_ listing: ListingDetail) -> some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 0) {
                // Photo header
                photoHeader(listing)

                VStack(alignment: .leading, spacing: 20) {
                    // Price and title
                    priceSection(listing)

                    // Key features
                    featuresRow(listing)

                    Divider()

                    // Description
                    descriptionSection(listing)

                    Divider()

                    // Amenities
                    amenitiesGrid(listing)

                    Divider()

                    // Location
                    locationSection(listing)

                    Divider()

                    // Agent contact
                    agentCard(listing)

                    // Contact button
                    contactButton
                }
                .padding()
            }
        }
    }

    private func photoHeader(_ listing: ListingDetail) -> some View {
        Button {
            showGallery = true
        } label: {
            ZStack(alignment: .bottomTrailing) {
                // Main image placeholder
                Rectangle()
                    .fill(Color(.systemGray5))
                    .frame(height: 280)
                    .overlay {
                        Image(systemName: "photo")
                            .font(.largeTitle)
                            .foregroundStyle(.secondary)
                    }

                // Photo count badge
                HStack(spacing: 4) {
                    Image(systemName: "photo.on.rectangle")
                    Text("\(listing.photos.count)")
                }
                .font(.caption)
                .fontWeight(.medium)
                .padding(.horizontal, 8)
                .padding(.vertical, 4)
                .background(.ultraThinMaterial)
                .clipShape(Capsule())
                .padding(12)
            }
        }
        .buttonStyle(.plain)
    }

    private func priceSection(_ listing: ListingDetail) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(listing.formattedPrice)
                .font(.title)
                .fontWeight(.bold)
                .foregroundStyle(Color.accentColor)

            Text(listing.title)
                .font(.title3)
                .fontWeight(.semibold)

            HStack(spacing: 4) {
                Image(systemName: "location.fill")
                    .font(.caption)
                Text(listing.address)
                    .font(.subheadline)
            }
            .foregroundStyle(.secondary)
        }
    }

    private func featuresRow(_ listing: ListingDetail) -> some View {
        HStack(spacing: 24) {
            if let area = listing.areaSqm {
                FeatureItem(icon: "square.fill", value: "\(area)", unit: "m2")
            }
            if let rooms = listing.rooms {
                FeatureItem(icon: "bed.double.fill", value: "\(rooms)", unit: "rooms")
            }
            if let bathrooms = listing.bathrooms {
                FeatureItem(icon: "shower.fill", value: "\(bathrooms)", unit: "baths")
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .clipShape(RoundedRectangle(cornerRadius: 12))
    }

    private func descriptionSection(_ listing: ListingDetail) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("Description")
                .font(.headline)

            Text(listing.description)
                .font(.body)
                .foregroundStyle(.secondary)
        }
    }

    private func amenitiesGrid(_ listing: ListingDetail) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Amenities")
                .font(.headline)

            LazyVGrid(columns: [
                GridItem(.flexible()),
                GridItem(.flexible())
            ], spacing: 12) {
                ForEach(listing.amenities, id: \.self) { amenity in
                    HStack(spacing: 8) {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundStyle(.green)
                        Text(amenity)
                            .font(.subheadline)
                        Spacer()
                    }
                }
            }
        }
    }

    private func locationSection(_ listing: ListingDetail) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Location")
                .font(.headline)

            // Map placeholder
            RoundedRectangle(cornerRadius: 12)
                .fill(Color(.systemGray5))
                .frame(height: 160)
                .overlay {
                    VStack {
                        Image(systemName: "map")
                            .font(.largeTitle)
                            .foregroundStyle(.secondary)
                        Text(listing.address)
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                }

            Button {
                coordinator.navigate(to: .listingMap(id: listingId))
            } label: {
                HStack {
                    Image(systemName: "map.fill")
                    Text("View on Map")
                }
                .font(.subheadline)
            }
        }
    }

    private func agentCard(_ listing: ListingDetail) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Contact Agent")
                .font(.headline)

            HStack(spacing: 12) {
                // Agent avatar placeholder
                Circle()
                    .fill(Color(.systemGray4))
                    .frame(width: 56, height: 56)
                    .overlay {
                        Text(listing.agentName.prefix(1).uppercased())
                            .font(.title3)
                            .fontWeight(.semibold)
                            .foregroundStyle(.secondary)
                    }

                VStack(alignment: .leading, spacing: 4) {
                    Text(listing.agentName)
                        .font(.subheadline)
                        .fontWeight(.semibold)

                    Text(listing.agentPhone ?? "Contact via inquiry")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }

                Spacer()

                if let phone = listing.agentPhone {
                    Button {
                        // Call agent
                        if let url = URL(string: "tel:\(phone)") {
                            UIApplication.shared.open(url)
                        }
                    } label: {
                        Image(systemName: "phone.fill")
                            .padding(12)
                            .background(Color.green)
                            .foregroundStyle(.white)
                            .clipShape(Circle())
                    }
                }
            }
            .padding()
            .background(Color(.systemGray6))
            .clipShape(RoundedRectangle(cornerRadius: 12))
        }
    }

    private var contactButton: some View {
        Button {
            showInquirySheet = true
        } label: {
            HStack {
                Image(systemName: "envelope.fill")
                Text("Send Inquiry")
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(Color.accentColor)
            .foregroundStyle(.white)
            .clipShape(RoundedRectangle(cornerRadius: 12))
        }
        .padding(.top, 8)
    }

    private var favoriteButton: some View {
        Button {
            withAnimation(.spring(response: 0.3, dampingFraction: 0.6)) {
                isFavorite.toggle()
            }
            // TODO: Persist favorite state
        } label: {
            Image(systemName: isFavorite ? "heart.fill" : "heart")
                .foregroundStyle(isFavorite ? .red : .primary)
        }
    }

    private var shareButton: some View {
        ShareLink(
            item: URL(string: "https://reality.example.com/listing/\(listingId)")!,
            subject: Text(listing?.title ?? "Property"),
            message: Text(listing?.formattedPrice ?? "")
        ) {
            Image(systemName: "square.and.arrow.up")
        }
    }

    // MARK: - Data Loading

    private func loadListing() async {
        isLoading = true
        errorMessage = nil

        // TODO: Integrate with KMP listing use case
        try? await Task.sleep(nanoseconds: 500_000_000)

        listing = ListingDetail.sample

        isLoading = false
    }
}

// MARK: - Supporting Views

private struct FeatureItem: View {
    let icon: String
    let value: String
    let unit: String

    var body: some View {
        VStack(spacing: 4) {
            Image(systemName: icon)
                .font(.title3)
                .foregroundStyle(.secondary)
            Text(value)
                .font(.headline)
            Text(unit)
                .font(.caption)
                .foregroundStyle(.secondary)
        }
        .frame(maxWidth: .infinity)
    }
}

// MARK: - New Inquiry Sheet

private struct NewInquirySheet: View {
    let listingId: String
    @Environment(\.dismiss) private var dismiss

    @State private var message = ""
    @State private var contactPreference = ContactPreference.email
    @State private var isSending = false

    var body: some View {
        NavigationStack {
            Form {
                Section("Your Message") {
                    TextEditor(text: $message)
                        .frame(minHeight: 150)
                }

                Section("Contact Preference") {
                    Picker("Prefer to be contacted by", selection: $contactPreference) {
                        Text("Email").tag(ContactPreference.email)
                        Text("Phone").tag(ContactPreference.phone)
                        Text("Either").tag(ContactPreference.either)
                    }
                }
            }
            .navigationTitle("Send Inquiry")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Send") {
                        Task {
                            isSending = true
                            // TODO: Send inquiry via KMP
                            try? await Task.sleep(nanoseconds: 1_000_000_000)
                            isSending = false
                            dismiss()
                        }
                    }
                    .disabled(message.isEmpty || isSending)
                }
            }
        }
    }
}

private enum ContactPreference: String, CaseIterable {
    case email
    case phone
    case either
}

// MARK: - Photo Gallery View

private struct PhotoGalleryView: View {
    let photos: [String]
    @Environment(\.dismiss) private var dismiss

    @State private var currentIndex = 0

    var body: some View {
        ZStack(alignment: .topLeading) {
            Color.black.ignoresSafeArea()

            TabView(selection: $currentIndex) {
                ForEach(photos.indices, id: \.self) { index in
                    Rectangle()
                        .fill(Color(.systemGray5))
                        .overlay {
                            Image(systemName: "photo")
                                .font(.largeTitle)
                                .foregroundStyle(.secondary)
                        }
                        .tag(index)
                }
            }
            .tabViewStyle(.page(indexDisplayMode: .never))

            // Close button
            Button {
                dismiss()
            } label: {
                Image(systemName: "xmark.circle.fill")
                    .font(.title)
                    .foregroundStyle(.white)
                    .padding()
            }

            // Photo counter
            Text("\(currentIndex + 1) / \(photos.count)")
                .foregroundStyle(.white)
                .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .bottom)
                .padding()
        }
    }
}

// MARK: - Preview Data

struct ListingDetail {
    let id: String
    let title: String
    let price: Int
    let currency: String
    let address: String
    let description: String
    let areaSqm: Int?
    let rooms: Int?
    let bathrooms: Int?
    let amenities: [String]
    let photos: [String]
    let agentName: String
    let agentPhone: String?
    let latitude: Double?
    let longitude: Double?

    var formattedPrice: String {
        let formatter = NumberFormatter()
        formatter.numberStyle = .currency
        formatter.currencyCode = currency
        formatter.maximumFractionDigits = 0
        return formatter.string(from: NSNumber(value: price)) ?? "\(price) \(currency)"
    }

    static let sample = ListingDetail(
        id: "1",
        title: "Modern Apartment in City Center",
        price: 250000,
        currency: "EUR",
        address: "Sturova 8, 811 02 Bratislava, Slovakia",
        description: "Beautiful modern apartment located in the heart of Bratislava. This property features high ceilings, hardwood floors, and large windows that flood the space with natural light. Recently renovated with high-end finishes throughout. Perfect for professionals or small families looking for a city center lifestyle.",
        areaSqm: 85,
        rooms: 3,
        bathrooms: 2,
        amenities: ["Balcony", "Parking", "Elevator", "Air Conditioning", "Heating", "Storage"],
        photos: ["photo1", "photo2", "photo3", "photo4"],
        agentName: "Maria Kovacova",
        agentPhone: "+421 900 123 456",
        latitude: 48.1486,
        longitude: 17.1077
    )
}

// MARK: - Preview

#Preview {
    NavigationStack {
        ListingDetailView(listingId: "1")
    }
    .environment(NavigationCoordinator())
    .environment(AuthManager())
}
