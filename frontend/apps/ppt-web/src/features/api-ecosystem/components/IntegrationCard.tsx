export interface IntegrationCardProps {
  id: string;
  slug: string;
  name: string;
  description: string;
  category: string;
  iconUrl?: string;
  vendorName: string;
  status: 'available' | 'coming_soon' | 'deprecated' | 'maintenance';
  ratingAverage?: number;
  ratingCount: number;
  installCount: number;
  isFeatured: boolean;
  isPremium: boolean;
  isInstalled?: boolean;
  onInstall?: () => void;
  onViewDetails?: () => void;
}

const categoryColors: Record<string, string> = {
  accounting: 'bg-green-100 text-green-800',
  crm: 'bg-blue-100 text-blue-800',
  calendar: 'bg-purple-100 text-purple-800',
  communication: 'bg-yellow-100 text-yellow-800',
  payment: 'bg-red-100 text-red-800',
  property_portal: 'bg-indigo-100 text-indigo-800',
  iot: 'bg-cyan-100 text-cyan-800',
  analytics: 'bg-orange-100 text-orange-800',
  document_management: 'bg-pink-100 text-pink-800',
  other: 'bg-gray-100 text-gray-800',
};

const statusLabels: Record<string, { label: string; color: string }> = {
  available: { label: 'Available', color: 'text-green-600' },
  coming_soon: { label: 'Coming Soon', color: 'text-blue-600' },
  deprecated: { label: 'Deprecated', color: 'text-red-600' },
  maintenance: { label: 'Maintenance', color: 'text-yellow-600' },
};

export function IntegrationCard({
  id: _id,
  slug: _slug,
  name,
  description,
  category,
  iconUrl,
  vendorName,
  status,
  ratingAverage,
  ratingCount,
  installCount,
  isFeatured,
  isPremium,
  isInstalled,
  onInstall,
  onViewDetails,
}: IntegrationCardProps) {
  const categoryColor = categoryColors[category] || categoryColors.other;
  const statusInfo = statusLabels[status] || statusLabels.available;

  const formatNumber = (num: number): string => {
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
    return num.toString();
  };

  const renderStars = (rating: number) => {
    const stars = [];
    for (let i = 1; i <= 5; i++) {
      stars.push(
        <svg
          key={i}
          className={`h-4 w-4 ${i <= rating ? 'text-yellow-400' : 'text-gray-300'}`}
          fill="currentColor"
          viewBox="0 0 20 20"
        >
          <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
        </svg>
      );
    }
    return stars;
  };

  return (
    <div
      className={`relative rounded-lg border bg-white p-6 shadow-sm transition-shadow hover:shadow-md ${
        isFeatured ? 'ring-2 ring-primary-500' : ''
      }`}
    >
      {/* Featured badge */}
      {isFeatured && (
        <div className="absolute -top-2 -right-2">
          <span className="inline-flex items-center rounded-full bg-primary-500 px-2 py-1 text-xs font-medium text-white">
            Featured
          </span>
        </div>
      )}

      {/* Premium badge */}
      {isPremium && (
        <div className="absolute top-4 right-4">
          <span className="inline-flex items-center rounded-full bg-gradient-to-r from-yellow-400 to-orange-500 px-2 py-1 text-xs font-medium text-white">
            Premium
          </span>
        </div>
      )}

      <div className="flex items-start space-x-4">
        {/* Icon */}
        <div className="flex-shrink-0">
          {iconUrl ? (
            <img src={iconUrl} alt={name} className="h-12 w-12 rounded-lg" />
          ) : (
            <div className="flex h-12 w-12 items-center justify-center rounded-lg bg-gray-100">
              <span className="text-xl font-bold text-gray-400">
                {name.charAt(0).toUpperCase()}
              </span>
            </div>
          )}
        </div>

        {/* Content */}
        <div className="min-w-0 flex-1">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-medium text-gray-900">{name}</h3>
            <span className={`text-xs ${statusInfo.color}`}>{statusInfo.label}</span>
          </div>

          <p className="mt-1 text-sm text-gray-500">by {vendorName}</p>

          <p className="mt-2 text-sm text-gray-600 line-clamp-2">{description}</p>

          {/* Category badge */}
          <div className="mt-3">
            <span
              className={`inline-flex items-center rounded-full px-2 py-1 text-xs font-medium ${categoryColor}`}
            >
              {category.replace('_', ' ')}
            </span>
          </div>

          {/* Rating and stats */}
          <div className="mt-4 flex items-center justify-between">
            <div className="flex items-center space-x-4">
              {/* Rating */}
              {ratingAverage !== undefined && (
                <div className="flex items-center space-x-1">
                  <div className="flex">{renderStars(Math.round(ratingAverage))}</div>
                  <span className="text-sm text-gray-600">
                    {ratingAverage.toFixed(1)} ({ratingCount})
                  </span>
                </div>
              )}

              {/* Install count */}
              <div className="flex items-center text-sm text-gray-500">
                <svg className="mr-1 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                  />
                </svg>
                {formatNumber(installCount)} installs
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Actions */}
      <div className="mt-4 flex space-x-3 border-t pt-4">
        <button
          type="button"
          onClick={onViewDetails}
          className="flex-1 rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
        >
          View Details
        </button>
        {status === 'available' && (
          <button
            type="button"
            onClick={onInstall}
            disabled={isInstalled}
            className={`flex-1 rounded-lg px-4 py-2 text-sm font-medium ${
              isInstalled
                ? 'cursor-not-allowed bg-gray-100 text-gray-500'
                : 'bg-primary-600 text-white hover:bg-primary-700'
            }`}
          >
            {isInstalled ? 'Installed' : 'Install'}
          </button>
        )}
      </div>
    </div>
  );
}

export default IntegrationCard;
