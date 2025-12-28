/**
 * Amenities Map Component
 * Story 70.3: Neighborhood Insights
 *
 * Displays nearby amenities on a map grouped by category.
 */

import { useMemo, useState } from 'react';

/** Amenity category constants */
export const AMENITY_CATEGORIES = {
  SCHOOL: 'school',
  KINDERGARTEN: 'kindergarten',
  UNIVERSITY: 'university',
  SUPERMARKET: 'supermarket',
  SHOP: 'shop',
  RESTAURANT: 'restaurant',
  CAFE: 'cafe',
  TRANSIT_STOP: 'transit_stop',
  TRAIN_STATION: 'train_station',
  HEALTHCARE: 'healthcare',
  PHARMACY: 'pharmacy',
  PARK: 'park',
  GYM: 'gym',
  BANK: 'bank',
} as const;

export type AmenityCategory = (typeof AMENITY_CATEGORIES)[keyof typeof AMENITY_CATEGORIES];

export interface NearbyAmenity {
  id: string;
  insightsId: string;
  category: AmenityCategory;
  name: string;
  address?: string;
  distanceMeters: number;
  latitude: number;
  longitude: number;
  rating?: number;
  details?: Record<string, unknown>;
}

export interface AmenitiesMapProps {
  amenities: NearbyAmenity[];
  centerLatitude: number;
  centerLongitude: number;
  className?: string;
}

const CATEGORY_ICONS: Record<string, { icon: string; color: string }> = {
  [AMENITY_CATEGORIES.SCHOOL]: { icon: 'S', color: 'bg-blue-500' },
  [AMENITY_CATEGORIES.KINDERGARTEN]: { icon: 'K', color: 'bg-pink-500' },
  [AMENITY_CATEGORIES.UNIVERSITY]: { icon: 'U', color: 'bg-indigo-500' },
  [AMENITY_CATEGORIES.SUPERMARKET]: { icon: 'M', color: 'bg-green-500' },
  [AMENITY_CATEGORIES.SHOP]: { icon: 'Sh', color: 'bg-emerald-500' },
  [AMENITY_CATEGORIES.RESTAURANT]: { icon: 'R', color: 'bg-orange-500' },
  [AMENITY_CATEGORIES.CAFE]: { icon: 'C', color: 'bg-amber-500' },
  [AMENITY_CATEGORIES.TRANSIT_STOP]: { icon: 'T', color: 'bg-purple-500' },
  [AMENITY_CATEGORIES.TRAIN_STATION]: { icon: 'Tr', color: 'bg-violet-500' },
  [AMENITY_CATEGORIES.HEALTHCARE]: { icon: 'H', color: 'bg-red-500' },
  [AMENITY_CATEGORIES.PHARMACY]: { icon: 'Ph', color: 'bg-rose-500' },
  [AMENITY_CATEGORIES.PARK]: { icon: 'P', color: 'bg-lime-500' },
  [AMENITY_CATEGORIES.GYM]: { icon: 'G', color: 'bg-cyan-500' },
  [AMENITY_CATEGORIES.BANK]: { icon: 'B', color: 'bg-slate-500' },
};

/**
 * Displays a map with nearby amenities.
 */
export function AmenitiesMap({
  amenities,
  centerLatitude,
  centerLongitude,
  className = '',
}: AmenitiesMapProps) {
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [selectedAmenity, setSelectedAmenity] = useState<NearbyAmenity | null>(null);

  // Group amenities by category
  const amenitiesByCategory = useMemo(() => {
    const grouped: Record<string, NearbyAmenity[]> = {};
    for (const amenity of amenities) {
      if (!grouped[amenity.category]) {
        grouped[amenity.category] = [];
      }
      grouped[amenity.category].push(amenity);
    }
    return grouped;
  }, [amenities]);

  const categories = Object.keys(amenitiesByCategory);

  const filteredAmenities = selectedCategory
    ? (amenitiesByCategory[selectedCategory] ?? [])
    : amenities;

  const formatDistance = (meters: number) => {
    if (meters < 1000) {
      return `${Math.round(meters)}m`;
    }
    return `${(meters / 1000).toFixed(1)}km`;
  };

  return (
    <div className={`bg-white rounded-lg shadow-sm border ${className}`}>
      {/* Category filters */}
      <div className="p-4 border-b">
        <h3 className="text-lg font-semibold text-gray-900 mb-3">Nearby Amenities</h3>
        <div className="flex flex-wrap gap-2">
          <button
            type="button"
            className={`px-3 py-1.5 text-sm rounded-full transition-colors ${
              selectedCategory === null
                ? 'bg-blue-600 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
            onClick={() => setSelectedCategory(null)}
          >
            All ({amenities.length})
          </button>
          {categories.map((category) => {
            const config = CATEGORY_ICONS[category] ?? {
              icon: '?',
              color: 'bg-gray-500',
            };
            return (
              <button
                key={category}
                type="button"
                className={`flex items-center gap-1.5 px-3 py-1.5 text-sm rounded-full transition-colors ${
                  selectedCategory === category
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
                onClick={() => setSelectedCategory(selectedCategory === category ? null : category)}
              >
                <span
                  className={`w-5 h-5 flex items-center justify-center text-xs text-white rounded-full ${config.color}`}
                >
                  {config.icon}
                </span>
                <span className="capitalize">{category.replace('_', ' ')}</span>
                <span className="text-xs">({amenitiesByCategory[category].length})</span>
              </button>
            );
          })}
        </div>
      </div>

      {/* Map placeholder */}
      <div className="relative h-64 bg-gray-100">
        {/* In production, this would be an interactive map (Leaflet, Mapbox, etc.) */}
        <div className="absolute inset-0 flex items-center justify-center">
          <div className="text-center">
            <svg
              className="w-12 h-12 text-gray-400 mx-auto mb-2"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 20l-5.447-2.724A1 1 0 013 16.382V5.618a1 1 0 011.447-.894L9 7m0 13l6-3m-6 3V7m6 10l4.553 2.276A1 1 0 0021 18.382V7.618a1 1 0 00-.553-.894L15 4m0 13V4m0 0L9 7"
              />
            </svg>
            <p className="text-sm text-gray-500">Interactive Map</p>
            <p className="text-xs text-gray-400">
              Center: {centerLatitude.toFixed(4)}, {centerLongitude.toFixed(4)}
            </p>
          </div>
        </div>

        {/* Amenity markers (positioned as overlay) */}
        <div className="absolute bottom-2 left-2 right-2">
          <div className="flex gap-1 flex-wrap justify-center">
            {filteredAmenities.slice(0, 10).map((amenity) => {
              const config = CATEGORY_ICONS[amenity.category] ?? {
                icon: '?',
                color: 'bg-gray-500',
              };
              return (
                <button
                  key={amenity.id}
                  type="button"
                  className={`w-6 h-6 flex items-center justify-center text-xs text-white rounded-full shadow cursor-pointer hover:scale-110 transition-transform ${config.color}`}
                  onClick={() => setSelectedAmenity(amenity)}
                  title={amenity.name}
                >
                  {config.icon}
                </button>
              );
            })}
          </div>
        </div>
      </div>

      {/* Selected amenity info */}
      {selectedAmenity && (
        <div className="p-4 border-t bg-blue-50">
          <div className="flex items-start justify-between">
            <div>
              <h4 className="font-medium text-gray-900">{selectedAmenity.name}</h4>
              {selectedAmenity.address && (
                <p className="text-sm text-gray-600">{selectedAmenity.address}</p>
              )}
              <div className="flex items-center gap-3 mt-1 text-sm text-gray-500">
                <span>{formatDistance(selectedAmenity.distanceMeters)}</span>
                {selectedAmenity.rating && (
                  <span className="flex items-center gap-1">
                    <svg
                      className="w-4 h-4 text-yellow-400"
                      fill="currentColor"
                      viewBox="0 0 20 20"
                    >
                      <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                    </svg>
                    {selectedAmenity.rating}
                  </span>
                )}
              </div>
            </div>
            <button
              type="button"
              className="text-gray-400 hover:text-gray-600"
              onClick={() => setSelectedAmenity(null)}
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              </svg>
            </button>
          </div>
        </div>
      )}

      {/* Amenities list */}
      <div className="p-4 max-h-64 overflow-y-auto">
        <div className="space-y-2">
          {filteredAmenities.map((amenity) => {
            const config = CATEGORY_ICONS[amenity.category] ?? {
              icon: '?',
              color: 'bg-gray-500',
            };
            return (
              <div
                key={amenity.id}
                className="flex items-center gap-3 p-2 hover:bg-gray-50 rounded-lg cursor-pointer transition-colors"
                onClick={() => setSelectedAmenity(amenity)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' || e.key === ' ') {
                    setSelectedAmenity(amenity);
                  }
                }}
                role="button"
                tabIndex={0}
              >
                <span
                  className={`w-8 h-8 flex items-center justify-center text-sm text-white rounded-full ${config.color}`}
                >
                  {config.icon}
                </span>
                <div className="flex-1 min-w-0">
                  <div className="font-medium text-gray-900 truncate">{amenity.name}</div>
                  <div className="text-sm text-gray-500 capitalize">
                    {amenity.category.replace('_', ' ')}
                  </div>
                </div>
                <div className="text-sm text-gray-400">
                  {formatDistance(amenity.distanceMeters)}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
}
