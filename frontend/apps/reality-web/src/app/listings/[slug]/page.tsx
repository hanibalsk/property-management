/**
 * Listing Detail Page
 *
 * Property detail page with SSR (Epic 44, Story 44.3).
 */

import { ContactForm } from '@/components/listings/ContactForm';
import { PhotoGallery } from '@/components/listings/PhotoGallery';
import { Footer, Header } from '@/components/ui';
import type { ListingDetail, ListingFeatures } from '@ppt/reality-api-client';
import type { Metadata } from 'next';

const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8081';

interface PageProps {
  params: Promise<{ slug: string }>;
}

async function getListing(slug: string): Promise<ListingDetail | null> {
  try {
    const response = await fetch(`${API_BASE}/api/v1/listings/${slug}`, {
      next: { revalidate: 60 }, // Revalidate every minute
    });
    if (!response.ok) return null;
    return response.json();
  } catch {
    return null;
  }
}

export async function generateMetadata({ params }: PageProps): Promise<Metadata> {
  const { slug } = await params;
  const listing = await getListing(slug);

  if (!listing) {
    return {
      title: 'Listing Not Found - Reality Portal',
    };
  }

  const title = `${listing.title} - ${listing.address.city} | Reality Portal`;
  const description = listing.description.slice(0, 160);

  return {
    title,
    description,
    openGraph: {
      title,
      description,
      type: 'website',
      images: listing.primaryPhoto ? [listing.primaryPhoto.url] : [],
    },
  };
}

function formatPrice(price: number, currency: string) {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: currency,
    maximumFractionDigits: 0,
  }).format(price);
}

function getFeatureLabel(key: keyof ListingFeatures): string {
  const labels: Record<keyof ListingFeatures, string> = {
    balcony: 'Balcony',
    terrace: 'Terrace',
    garden: 'Garden',
    parking: 'Parking',
    garage: 'Garage',
    elevator: 'Elevator',
    cellar: 'Cellar',
    airConditioning: 'Air Conditioning',
    furnished: 'Furnished',
    petFriendly: 'Pet Friendly',
    disabledAccess: 'Disabled Access',
  };
  return labels[key] || key;
}

export default async function ListingDetailPage({ params }: PageProps) {
  const { slug } = await params;
  const listing = await getListing(slug);

  if (!listing) {
    return (
      <div className="page-container">
        <Header />
        <main className="main">
          <div className="not-found">
            <h1>Listing Not Found</h1>
            <p>The listing you&apos;re looking for doesn&apos;t exist or has been removed.</p>
            <a href="/listings" className="back-link">
              Browse all listings
            </a>
          </div>
        </main>
        <Footer />
        <style jsx>{`
          .page-container {
            min-height: 100vh;
            display: flex;
            flex-direction: column;
          }
          .main {
            flex: 1;
            display: flex;
            align-items: center;
            justify-content: center;
          }
          .not-found {
            text-align: center;
            padding: 64px 16px;
          }
          .not-found h1 {
            font-size: 2rem;
            color: #111827;
            margin: 0 0 16px;
          }
          .not-found p {
            color: #6b7280;
            margin: 0 0 24px;
          }
          .back-link {
            color: #2563eb;
            text-decoration: none;
          }
          .back-link:hover {
            text-decoration: underline;
          }
        `}</style>
      </div>
    );
  }

  const activeFeatures = Object.entries(listing.features)
    .filter(([, value]) => value === true)
    .map(([key]) => key as keyof ListingFeatures);

  // JSON-LD structured data
  const jsonLd = {
    '@context': 'https://schema.org',
    '@type': 'RealEstateListing',
    name: listing.title,
    description: listing.description,
    url: `${process.env.NEXT_PUBLIC_SITE_URL || ''}/listings/${listing.slug}`,
    image: listing.photos.map((p) => p.url),
    address: {
      '@type': 'PostalAddress',
      streetAddress: listing.address.street,
      addressLocality: listing.address.city,
      addressRegion: listing.address.district,
      postalCode: listing.address.postalCode,
      addressCountry: listing.address.country,
    },
    offers: {
      '@type': 'Offer',
      price: listing.price,
      priceCurrency: listing.currency,
      availability: listing.status === 'active' ? 'InStock' : 'OutOfStock',
    },
    numberOfRooms: listing.rooms,
    floorSize: {
      '@type': 'QuantitativeValue',
      value: listing.area,
      unitCode: 'MTK',
    },
  };

  return (
    <div className="page-container">
      {/* JSON-LD structured data for SEO */}
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />
      <Header />
      <main className="main">
        <div className="container">
          {/* Breadcrumb */}
          <nav className="breadcrumb" aria-label="Breadcrumb">
            <a href="/">Home</a>
            <span className="separator">/</span>
            <a href="/listings">Listings</a>
            <span className="separator">/</span>
            <span className="current">{listing.address.city}</span>
          </nav>

          <div className="content-grid">
            {/* Main Content */}
            <div className="main-content">
              {/* Photo Gallery */}
              <PhotoGallery photos={listing.photos} title={listing.title} />

              {/* Header */}
              <div className="listing-header">
                <div className="badges">
                  <span className={`badge ${listing.transactionType}`}>
                    {listing.transactionType === 'sale' ? 'For Sale' : 'For Rent'}
                  </span>
                  <span className="badge type">{listing.propertyType}</span>
                </div>
                <h1 className="title">{listing.title}</h1>
                <p className="address">
                  <svg
                    width="18"
                    height="18"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2"
                    aria-hidden="true"
                  >
                    <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" />
                    <circle cx="12" cy="10" r="3" />
                  </svg>
                  {listing.address.street && `${listing.address.street}, `}
                  {listing.address.city}
                  {listing.address.district && `, ${listing.address.district}`}
                </p>
                <div className="price-row">
                  <span className="price">{formatPrice(listing.price, listing.currency)}</span>
                  {listing.transactionType === 'rent' && (
                    <span className="price-suffix">/month</span>
                  )}
                  {listing.pricePerSqm && (
                    <span className="price-per-sqm">
                      ({formatPrice(listing.pricePerSqm, listing.currency)}/m²)
                    </span>
                  )}
                </div>
              </div>

              {/* Key Details */}
              <div className="key-details">
                {listing.rooms !== undefined && (
                  <div className="detail-item">
                    <span className="detail-value">{listing.rooms}</span>
                    <span className="detail-label">Rooms</span>
                  </div>
                )}
                {listing.bedrooms !== undefined && (
                  <div className="detail-item">
                    <span className="detail-value">{listing.bedrooms}</span>
                    <span className="detail-label">Bedrooms</span>
                  </div>
                )}
                {listing.bathrooms !== undefined && (
                  <div className="detail-item">
                    <span className="detail-value">{listing.bathrooms}</span>
                    <span className="detail-label">Bathrooms</span>
                  </div>
                )}
                <div className="detail-item">
                  <span className="detail-value">{listing.area}</span>
                  <span className="detail-label">m²</span>
                </div>
                {listing.floor !== undefined && (
                  <div className="detail-item">
                    <span className="detail-value">
                      {listing.floor}
                      {listing.totalFloors && `/${listing.totalFloors}`}
                    </span>
                    <span className="detail-label">Floor</span>
                  </div>
                )}
                {listing.yearBuilt !== undefined && (
                  <div className="detail-item">
                    <span className="detail-value">{listing.yearBuilt}</span>
                    <span className="detail-label">Built</span>
                  </div>
                )}
              </div>

              {/* Description */}
              <section className="section">
                <h2 className="section-title">Description</h2>
                <p className="description">{listing.description}</p>
              </section>

              {/* Features */}
              {activeFeatures.length > 0 && (
                <section className="section">
                  <h2 className="section-title">Features</h2>
                  <div className="features-grid">
                    {activeFeatures.map((feature) => (
                      <div key={feature} className="feature-item">
                        <svg
                          width="16"
                          height="16"
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="currentColor"
                          strokeWidth="2"
                          aria-hidden="true"
                        >
                          <polyline points="20 6 9 17 4 12" />
                        </svg>
                        <span>{getFeatureLabel(feature)}</span>
                      </div>
                    ))}
                  </div>
                </section>
              )}

              {/* Additional Info */}
              <section className="section">
                <h2 className="section-title">Additional Information</h2>
                <div className="info-grid">
                  {listing.energyRating && (
                    <div className="info-item">
                      <span className="info-label">Energy Rating</span>
                      <span className="info-value">{listing.energyRating}</span>
                    </div>
                  )}
                  {listing.monthlyCharges !== undefined && (
                    <div className="info-item">
                      <span className="info-label">Monthly Charges</span>
                      <span className="info-value">
                        {formatPrice(listing.monthlyCharges, listing.currency)}
                      </span>
                    </div>
                  )}
                  {listing.availableFrom && (
                    <div className="info-item">
                      <span className="info-label">Available From</span>
                      <span className="info-value">
                        {new Date(listing.availableFrom).toLocaleDateString()}
                      </span>
                    </div>
                  )}
                  <div className="info-item">
                    <span className="info-label">Listed</span>
                    <span className="info-value">
                      {new Date(listing.createdAt).toLocaleDateString()}
                    </span>
                  </div>
                </div>
              </section>

              {/* Virtual Tour / Floor Plan */}
              {(listing.virtualTourUrl || listing.floorPlanUrl) && (
                <section className="section">
                  <h2 className="section-title">Additional Resources</h2>
                  <div className="resources">
                    {listing.virtualTourUrl && (
                      <a
                        href={listing.virtualTourUrl}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="resource-link"
                      >
                        <svg
                          width="20"
                          height="20"
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="currentColor"
                          strokeWidth="2"
                          aria-hidden="true"
                        >
                          <circle cx="12" cy="12" r="10" />
                          <polygon points="10 8 16 12 10 16 10 8" />
                        </svg>
                        Virtual Tour
                      </a>
                    )}
                    {listing.floorPlanUrl && (
                      <a
                        href={listing.floorPlanUrl}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="resource-link"
                      >
                        <svg
                          width="20"
                          height="20"
                          viewBox="0 0 24 24"
                          fill="none"
                          stroke="currentColor"
                          strokeWidth="2"
                          aria-hidden="true"
                        >
                          <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                          <line x1="3" y1="9" x2="21" y2="9" />
                          <line x1="9" y1="21" x2="9" y2="9" />
                        </svg>
                        Floor Plan
                      </a>
                    )}
                  </div>
                </section>
              )}
            </div>

            {/* Sidebar */}
            <div className="sidebar">
              <ContactForm listingId={listing.id} agent={listing.agent} />
            </div>
          </div>
        </div>
      </main>
      <Footer />

      <style jsx>{`
        .page-container {
          min-height: 100vh;
          display: flex;
          flex-direction: column;
          background: #f9fafb;
        }

        .main {
          flex: 1;
          padding: 24px 0;
        }

        .container {
          max-width: 1280px;
          margin: 0 auto;
          padding: 0 16px;
        }

        .breadcrumb {
          font-size: 14px;
          color: #6b7280;
          margin-bottom: 24px;
        }

        .breadcrumb a {
          color: #6b7280;
          text-decoration: none;
        }

        .breadcrumb a:hover {
          color: #2563eb;
        }

        .separator {
          margin: 0 8px;
        }

        .current {
          color: #111827;
        }

        .content-grid {
          display: grid;
          gap: 32px;
        }

        @media (min-width: 1024px) {
          .content-grid {
            grid-template-columns: 1fr 380px;
          }
        }

        .main-content {
          min-width: 0;
        }

        .listing-header {
          margin-top: 24px;
        }

        .badges {
          display: flex;
          gap: 8px;
          margin-bottom: 12px;
        }

        .badge {
          padding: 4px 12px;
          border-radius: 4px;
          font-size: 12px;
          font-weight: 600;
          text-transform: uppercase;
        }

        .badge.sale {
          background: #10b981;
          color: #fff;
        }

        .badge.rent {
          background: #3b82f6;
          color: #fff;
        }

        .badge.type {
          background: #f3f4f6;
          color: #374151;
        }

        .title {
          font-size: 1.75rem;
          font-weight: bold;
          color: #111827;
          margin: 0 0 12px;
        }

        .address {
          display: flex;
          align-items: center;
          gap: 8px;
          font-size: 1rem;
          color: #6b7280;
          margin: 0 0 16px;
        }

        .price-row {
          display: flex;
          align-items: baseline;
          gap: 8px;
        }

        .price {
          font-size: 2rem;
          font-weight: bold;
          color: #111827;
        }

        .price-suffix {
          font-size: 1rem;
          color: #6b7280;
        }

        .price-per-sqm {
          font-size: 14px;
          color: #9ca3af;
        }

        .key-details {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
          gap: 16px;
          padding: 24px;
          background: #fff;
          border-radius: 12px;
          margin-top: 24px;
        }

        .detail-item {
          text-align: center;
        }

        .detail-value {
          display: block;
          font-size: 1.5rem;
          font-weight: 600;
          color: #111827;
        }

        .detail-label {
          font-size: 14px;
          color: #6b7280;
        }

        .section {
          padding: 24px;
          background: #fff;
          border-radius: 12px;
          margin-top: 24px;
        }

        .section-title {
          font-size: 1.125rem;
          font-weight: 600;
          color: #111827;
          margin: 0 0 16px;
        }

        .description {
          color: #374151;
          line-height: 1.7;
          white-space: pre-line;
          margin: 0;
        }

        .features-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
          gap: 12px;
        }

        .feature-item {
          display: flex;
          align-items: center;
          gap: 8px;
          color: #374151;
        }

        .feature-item svg {
          color: #10b981;
        }

        .info-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
          gap: 16px;
        }

        .info-item {
          display: flex;
          justify-content: space-between;
          padding: 12px 0;
          border-bottom: 1px solid #f3f4f6;
        }

        .info-label {
          color: #6b7280;
        }

        .info-value {
          font-weight: 500;
          color: #111827;
        }

        .resources {
          display: flex;
          gap: 16px;
          flex-wrap: wrap;
        }

        .resource-link {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 12px 20px;
          background: #f3f4f6;
          border-radius: 8px;
          color: #374151;
          text-decoration: none;
          font-weight: 500;
          transition: background 0.2s;
        }

        .resource-link:hover {
          background: #e5e7eb;
        }

        .sidebar {
          position: sticky;
          top: 88px;
          height: fit-content;
        }
      `}</style>
    </div>
  );
}
