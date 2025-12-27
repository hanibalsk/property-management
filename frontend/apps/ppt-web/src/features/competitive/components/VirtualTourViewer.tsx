/**
 * Virtual Tour Viewer Component
 * Story 70.1: Virtual Tour Integration
 *
 * Displays 360 photos, Matterport embeds, or video tours with hotspot navigation.
 */

import { useState } from 'react';

/** Tour type constants */
export const TOUR_TYPES = {
  PHOTO_360: 'photo_360',
  MATTERPORT: 'matterport',
  VIDEO: 'video',
  EXTERNAL_EMBED: 'external_embed',
} as const;

export type TourType = (typeof TOUR_TYPES)[keyof typeof TOUR_TYPES];

/** Hotspot on a virtual tour */
export interface TourHotspot {
  id: string;
  label: string;
  description?: string;
  positionX: number;
  positionY: number;
  linkToTourId?: string;
  actionType?: string;
}

/** Virtual tour data */
export interface VirtualTour {
  id: string;
  listingId: string;
  tourType: TourType;
  title?: string;
  description?: string;
  photoUrl?: string;
  embedUrl?: string;
  externalId?: string;
  videoUrl?: string;
  thumbnailUrl?: string;
  displayOrder: number;
  isFeatured: boolean;
  hotspots: TourHotspot[];
}

export interface VirtualTourViewerProps {
  tour: VirtualTour;
  onHotspotClick?: (hotspot: TourHotspot) => void;
  onTourChange?: (tourId: string) => void;
  className?: string;
}

/**
 * Renders a virtual tour with appropriate viewer based on tour type.
 */
export function VirtualTourViewer({
  tour,
  onHotspotClick,
  onTourChange,
  className = '',
}: VirtualTourViewerProps) {
  const [activeHotspot, setActiveHotspot] = useState<TourHotspot | null>(null);
  const [isFullscreen, setIsFullscreen] = useState(false);

  const handleHotspotClick = (hotspot: TourHotspot) => {
    setActiveHotspot(hotspot);
    if (hotspot.linkToTourId && onTourChange) {
      onTourChange(hotspot.linkToTourId);
    }
    onHotspotClick?.(hotspot);
  };

  const renderTourContent = () => {
    switch (tour.tourType) {
      case TOUR_TYPES.MATTERPORT:
      case TOUR_TYPES.EXTERNAL_EMBED:
        return (
          <div className="relative w-full h-full">
            <iframe
              src={tour.embedUrl}
              title={tour.title ?? 'Virtual Tour'}
              className="w-full h-full border-0"
              allowFullScreen
              allow="xr-spatial-tracking"
            />
          </div>
        );

      case TOUR_TYPES.VIDEO:
        return (
          <div className="relative w-full h-full">
            <video
              src={tour.videoUrl}
              poster={tour.thumbnailUrl}
              controls
              className="w-full h-full object-cover"
            >
              <track kind="captions" />
              Your browser does not support video playback.
            </video>
          </div>
        );

      case TOUR_TYPES.PHOTO_360:
      default:
        return (
          <div className="relative w-full h-full">
            {/* 360 photo viewer - in production would use a library like Photo Sphere Viewer */}
            <div
              className="w-full h-full bg-cover bg-center rounded-lg"
              style={{ backgroundImage: `url(${tour.photoUrl})` }}
            >
              {/* Hotspots overlay */}
              {tour.hotspots.map((hotspot) => (
                <button
                  key={hotspot.id}
                  type="button"
                  className="absolute w-8 h-8 -ml-4 -mt-4 bg-blue-600 rounded-full border-2 border-white shadow-lg cursor-pointer hover:bg-blue-700 transition-colors flex items-center justify-center"
                  style={{
                    left: `${hotspot.positionX}%`,
                    top: `${hotspot.positionY}%`,
                  }}
                  onClick={() => handleHotspotClick(hotspot)}
                  title={hotspot.label}
                >
                  <span className="text-white text-xs font-bold">+</span>
                </button>
              ))}
            </div>
          </div>
        );
    }
  };

  return (
    <div className={`relative bg-gray-900 rounded-lg overflow-hidden ${className}`}>
      {/* Tour header */}
      <div className="absolute top-0 left-0 right-0 z-10 bg-gradient-to-b from-black/70 to-transparent p-4">
        <div className="flex items-center justify-between text-white">
          <div>
            {tour.title && <h3 className="text-lg font-semibold">{tour.title}</h3>}
            {tour.description && <p className="text-sm text-gray-300">{tour.description}</p>}
          </div>
          <div className="flex items-center gap-2">
            {tour.isFeatured && (
              <span className="px-2 py-1 bg-yellow-500 text-black text-xs font-medium rounded">
                Featured
              </span>
            )}
            <button
              type="button"
              className="p-2 hover:bg-white/20 rounded-full transition-colors"
              onClick={() => setIsFullscreen(!isFullscreen)}
              title={isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'}
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d={
                    isFullscreen
                      ? 'M9 9L4 4m0 0v5m0-5h5m6 6l5 5m0 0v-5m0 5h-5'
                      : 'M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5v-4m0 4h-4m4 0l-5-5'
                  }
                />
              </svg>
            </button>
          </div>
        </div>
      </div>

      {/* Tour content */}
      <div className="aspect-video">{renderTourContent()}</div>

      {/* Hotspot info popup */}
      {activeHotspot && (
        <div className="absolute bottom-4 left-4 right-4 bg-white rounded-lg shadow-xl p-4 z-20">
          <div className="flex items-start justify-between">
            <div>
              <h4 className="font-semibold text-gray-900">{activeHotspot.label}</h4>
              {activeHotspot.description && (
                <p className="text-sm text-gray-600 mt-1">{activeHotspot.description}</p>
              )}
            </div>
            <button
              type="button"
              className="text-gray-400 hover:text-gray-600"
              onClick={() => setActiveHotspot(null)}
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

      {/* Tour type indicator */}
      <div className="absolute bottom-4 right-4 z-10">
        <span className="px-2 py-1 bg-black/50 text-white text-xs rounded capitalize">
          {tour.tourType.replace('_', ' ')}
        </span>
      </div>
    </div>
  );
}
