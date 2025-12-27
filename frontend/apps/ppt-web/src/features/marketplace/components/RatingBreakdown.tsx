/**
 * RatingBreakdown component - displays detailed rating breakdown for a provider.
 * Epic 68: Service Provider Marketplace (Story 68.5)
 */

import { RatingStars } from './RatingStars';

export interface RatingBreakdownData {
  averageOverall: number;
  averageQuality: number;
  averageTimeliness: number;
  averageCommunication: number;
  averageValue: number;
  totalReviews: number;
  distribution: {
    fiveStar: number;
    fourStar: number;
    threeStar: number;
    twoStar: number;
    oneStar: number;
  };
}

interface RatingBreakdownProps {
  data: RatingBreakdownData;
  compact?: boolean;
}

function RatingBar({ label, count, total }: { label: string; count: number; total: number }) {
  const percentage = total > 0 ? (count / total) * 100 : 0;

  return (
    <div className="flex items-center gap-2">
      <span className="w-12 text-sm text-gray-600">{label}</span>
      <div className="flex-1 h-2 bg-gray-200 rounded-full overflow-hidden">
        <div
          className="h-full bg-yellow-400 rounded-full"
          style={{ width: `${percentage}%` }}
        />
      </div>
      <span className="w-8 text-sm text-gray-500 text-right">{count}</span>
    </div>
  );
}

function DimensionRating({ label, value }: { label: string; value: number }) {
  return (
    <div className="flex items-center justify-between">
      <span className="text-sm text-gray-600">{label}</span>
      <div className="flex items-center gap-2">
        <RatingStars rating={value} size="sm" />
        <span className="text-sm font-medium text-gray-900 w-8">{value.toFixed(1)}</span>
      </div>
    </div>
  );
}

export function RatingBreakdown({ data, compact = false }: RatingBreakdownProps) {
  const { distribution } = data;

  if (compact) {
    return (
      <div className="flex items-center gap-3">
        <div className="text-center">
          <p className="text-3xl font-bold text-gray-900">{data.averageOverall.toFixed(1)}</p>
          <RatingStars rating={data.averageOverall} size="sm" />
          <p className="text-sm text-gray-500 mt-1">{data.totalReviews} reviews</p>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h3 className="text-lg font-medium text-gray-900 mb-4">Rating Breakdown</h3>

      <div className="flex flex-col md:flex-row gap-8">
        {/* Overall Rating */}
        <div className="text-center md:border-r md:pr-8">
          <p className="text-5xl font-bold text-gray-900">{data.averageOverall.toFixed(1)}</p>
          <div className="mt-2 flex justify-center">
            <RatingStars rating={data.averageOverall} size="lg" />
          </div>
          <p className="mt-2 text-gray-500">Based on {data.totalReviews} reviews</p>
        </div>

        {/* Distribution */}
        <div className="flex-1 space-y-2">
          <RatingBar label="5 stars" count={distribution.fiveStar} total={data.totalReviews} />
          <RatingBar label="4 stars" count={distribution.fourStar} total={data.totalReviews} />
          <RatingBar label="3 stars" count={distribution.threeStar} total={data.totalReviews} />
          <RatingBar label="2 stars" count={distribution.twoStar} total={data.totalReviews} />
          <RatingBar label="1 star" count={distribution.oneStar} total={data.totalReviews} />
        </div>
      </div>

      {/* Dimension Ratings */}
      <div className="mt-6 pt-6 border-t">
        <h4 className="text-sm font-medium text-gray-700 mb-3">Rating by Category</h4>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <DimensionRating label="Quality" value={data.averageQuality} />
          <DimensionRating label="Timeliness" value={data.averageTimeliness} />
          <DimensionRating label="Communication" value={data.averageCommunication} />
          <DimensionRating label="Value" value={data.averageValue} />
        </div>
      </div>
    </div>
  );
}

interface RatingSummaryProps {
  averageRating: number;
  totalReviews: number;
  size?: 'sm' | 'md' | 'lg';
}

export function RatingSummary({ averageRating, totalReviews, size = 'md' }: RatingSummaryProps) {
  if (totalReviews === 0) {
    return <span className="text-sm text-gray-500">No reviews yet</span>;
  }

  const textSizes = {
    sm: 'text-sm',
    md: 'text-base',
    lg: 'text-lg',
  };

  return (
    <div className="flex items-center gap-2">
      <RatingStars rating={averageRating} size={size} />
      <span className={`${textSizes[size]} font-medium text-gray-700`}>
        {averageRating.toFixed(1)}
      </span>
      <span className={`${textSizes[size]} text-gray-500`}>
        ({totalReviews} review{totalReviews !== 1 ? 's' : ''})
      </span>
    </div>
  );
}
