/**
 * ListingCard Component
 *
 * Property listing card for search results and featured sections (Epic 44).
 */

'use client';

import type { ListingSummary } from '@ppt/reality-api-client';
import { useTranslations } from 'next-intl';
import Link from 'next/link';

interface ListingCardProps {
  listing: ListingSummary;
  onToggleFavorite?: (listingId: string, isFavorite: boolean) => void;
  showFavoriteButton?: boolean;
}

export function ListingCard({
  listing,
  onToggleFavorite,
  showFavoriteButton = true,
}: ListingCardProps) {
  const t = useTranslations('listing');

  const formatPrice = (price: number, currency: string) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: currency,
      maximumFractionDigits: 0,
    }).format(price);
  };

  const handleFavoriteClick = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    onToggleFavorite?.(listing.id, listing.isFavorite ?? false);
  };

  return (
    <Link href={`/listings/${listing.slug}`} className="card">
      {/* Image */}
      <div className="image-container">
        {listing.primaryPhoto ? (
          <img src={listing.primaryPhoto.thumbnailUrl} alt={listing.title} className="image" />
        ) : (
          <div className="image-placeholder">
            <svg
              width="48"
              height="48"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="1"
              aria-hidden="true"
            >
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
              <circle cx="8.5" cy="8.5" r="1.5" />
              <polyline points="21 15 16 10 5 21" />
            </svg>
          </div>
        )}

        {/* Badges */}
        <div className="badges">
          {listing.isFeatured && <span className="badge featured">{t('featured')}</span>}
          <span className={`badge ${listing.transactionType}`}>
            {listing.transactionType === 'sale' ? t('forSale') : t('forRent')}
          </span>
        </div>

        {/* Favorite Button */}
        {showFavoriteButton && (
          <button
            type="button"
            className={`favorite-button ${listing.isFavorite ? 'active' : ''}`}
            onClick={handleFavoriteClick}
            aria-label={listing.isFavorite ? 'Remove from favorites' : 'Add to favorites'}
          >
            <svg
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill={listing.isFavorite ? 'currentColor' : 'none'}
              stroke="currentColor"
              strokeWidth="2"
              aria-hidden="true"
            >
              <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" />
            </svg>
          </button>
        )}
      </div>

      {/* Content */}
      <div className="content">
        <div className="price-row">
          <span className="price">{formatPrice(listing.price, listing.currency)}</span>
          {listing.transactionType === 'rent' && (
            <span className="price-suffix">{t('perMonth')}</span>
          )}
        </div>

        <h3 className="title">{listing.title}</h3>

        <p className="location">
          <svg
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            aria-hidden="true"
          >
            <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" />
            <circle cx="12" cy="10" r="3" />
          </svg>
          {listing.address.city}
          {listing.address.district && `, ${listing.address.district}`}
        </p>

        {/* Features */}
        <div className="features">
          {listing.rooms !== undefined && (
            <div className="feature">
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                aria-hidden="true"
              >
                <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
              </svg>
              <span>{t('roomsCount', { count: listing.rooms })}</span>
            </div>
          )}
          <div className="feature">
            <svg
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              aria-hidden="true"
            >
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
            </svg>
            <span>
              {listing.area} {t('sqm')}
            </span>
          </div>
          {listing.floor !== undefined && (
            <div className="feature">
              <svg
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                strokeWidth="2"
                aria-hidden="true"
              >
                <path d="M18 20V4H6v16" />
                <path d="M2 20h20" />
              </svg>
              <span>{t('floorNumber', { number: listing.floor })}</span>
            </div>
          )}
        </div>
      </div>

      <style jsx>{`
        .card {
          display: block;
          background: #fff;
          border-radius: 12px;
          overflow: hidden;
          box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
          text-decoration: none;
          transition: all 0.2s;
        }

        .card:hover {
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
          transform: translateY(-2px);
        }

        .image-container {
          position: relative;
          aspect-ratio: 4 / 3;
          background: #f3f4f6;
        }

        .image {
          width: 100%;
          height: 100%;
          object-fit: cover;
        }

        .image-placeholder {
          width: 100%;
          height: 100%;
          display: flex;
          align-items: center;
          justify-content: center;
          color: #9ca3af;
        }

        .badges {
          position: absolute;
          top: 12px;
          left: 12px;
          display: flex;
          gap: 8px;
        }

        .badge {
          padding: 4px 8px;
          border-radius: 4px;
          font-size: 12px;
          font-weight: 600;
          text-transform: uppercase;
        }

        .badge.featured {
          background: #fbbf24;
          color: #78350f;
        }

        .badge.sale {
          background: #10b981;
          color: #fff;
        }

        .badge.rent {
          background: #3b82f6;
          color: #fff;
        }

        .favorite-button {
          position: absolute;
          top: 12px;
          right: 12px;
          width: 36px;
          height: 36px;
          border-radius: 50%;
          background: rgba(255, 255, 255, 0.9);
          border: none;
          cursor: pointer;
          display: flex;
          align-items: center;
          justify-content: center;
          color: #9ca3af;
          transition: all 0.2s;
        }

        .favorite-button:hover {
          background: #fff;
          color: #ef4444;
        }

        .favorite-button.active {
          color: #ef4444;
        }

        .content {
          padding: 16px;
        }

        .price-row {
          display: flex;
          align-items: baseline;
          gap: 4px;
          margin-bottom: 8px;
        }

        .price {
          font-size: 1.25rem;
          font-weight: bold;
          color: #111827;
        }

        .price-suffix {
          font-size: 14px;
          color: #6b7280;
        }

        .title {
          font-size: 1rem;
          font-weight: 600;
          color: #374151;
          margin: 0 0 8px;
          display: -webkit-box;
          -webkit-line-clamp: 2;
          -webkit-box-orient: vertical;
          overflow: hidden;
        }

        .location {
          display: flex;
          align-items: center;
          gap: 4px;
          font-size: 14px;
          color: #6b7280;
          margin: 0 0 12px;
        }

        .features {
          display: flex;
          gap: 16px;
          flex-wrap: wrap;
        }

        .feature {
          display: flex;
          align-items: center;
          gap: 4px;
          font-size: 13px;
          color: #6b7280;
        }
      `}</style>
    </Link>
  );
}
