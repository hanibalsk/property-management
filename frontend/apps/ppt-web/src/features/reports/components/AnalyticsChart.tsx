/**
 * AnalyticsChart component - Story 53.3
 *
 * Interactive chart for analytics dashboards.
 */

import type { TrendLine } from '@ppt/api-client';

interface AnalyticsChartProps {
  title: string;
  data: TrendLine[];
  chartType?: 'line' | 'bar' | 'area';
  height?: number;
  isLoading?: boolean;
  onDrillDown?: (point: { date: string; value: number }) => void;
}

function getMaxValue(data: TrendLine[]): number {
  let max = 1;
  for (const line of data) {
    for (const point of line.data) {
      if (point.value > max) {
        max = point.value;
      }
    }
  }
  return max;
}

function formatValue(value: number): string {
  if (value >= 1000000) {
    return `${(value / 1000000).toFixed(1)}M`;
  }
  if (value >= 1000) {
    return `${(value / 1000).toFixed(1)}K`;
  }
  return value.toFixed(0);
}

const COLORS = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#ec4899'];

export function AnalyticsChart({
  title,
  data,
  chartType = 'line',
  height = 300,
  isLoading,
  onDrillDown,
}: AnalyticsChartProps) {
  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow p-6">
        <div className="h-6 bg-gray-200 rounded w-1/4 mb-4" />
        <div className="animate-pulse bg-gray-200 rounded" style={{ height: `${height}px` }} />
      </div>
    );
  }

  if (data.length === 0 || data.every((line) => line.data.length === 0)) {
    return (
      <div className="bg-white rounded-lg shadow p-6">
        <h3 className="text-lg font-medium text-gray-900 mb-4">{title}</h3>
        <div
          className="flex items-center justify-center text-gray-500"
          style={{ height: `${height}px` }}
        >
          No data available
        </div>
      </div>
    );
  }

  const maxValue = getMaxValue(data);
  const chartWidth = 100;
  const maxPoints = Math.max(...data.map((line) => line.data.length));

  // Generate Y-axis labels
  const yAxisLabels = [0, 0.25, 0.5, 0.75, 1].map((ratio) => ({
    value: maxValue * ratio,
    position: 100 - ratio * 100,
  }));

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-medium text-gray-900">{title}</h3>
        {/* Legend */}
        <div className="flex items-center gap-4">
          {data.map((line, index) => (
            <div key={line.id} className="flex items-center gap-2">
              <span
                className="w-3 h-3 rounded-full"
                style={{ backgroundColor: line.color || COLORS[index % COLORS.length] }}
              />
              <span className="text-sm text-gray-600">{line.name}</span>
              {line.change_percentage !== 0 && (
                <span
                  className={`text-xs ${
                    line.trend === 'up'
                      ? 'text-green-600'
                      : line.trend === 'down'
                        ? 'text-red-600'
                        : 'text-gray-500'
                  }`}
                >
                  {line.change_percentage > 0 ? '+' : ''}
                  {line.change_percentage.toFixed(1)}%
                </span>
              )}
            </div>
          ))}
        </div>
      </div>

      <div className="relative" style={{ height: `${height}px` }}>
        {/* Y-axis */}
        <div className="absolute left-0 top-0 bottom-0 w-12 flex flex-col justify-between text-xs text-gray-400">
          {yAxisLabels.map((label) => (
            <span key={label.position} style={{ transform: 'translateY(-50%)' }}>
              {formatValue(label.value)}
            </span>
          ))}
        </div>

        {/* Chart Area */}
        <div className="ml-14 relative h-full">
          {/* Grid lines */}
          <div className="absolute inset-0">
            {yAxisLabels.map((label) => (
              <div
                key={label.position}
                className="absolute left-0 right-0 border-t border-gray-100"
                style={{ top: `${label.position}%` }}
              />
            ))}
          </div>

          {/* SVG Chart */}
          <svg className="w-full h-full" preserveAspectRatio="xMidYMid meet" aria-hidden="true">
            {data.map((line, lineIndex) => {
              const color = line.color || COLORS[lineIndex % COLORS.length];
              const points = line.data.map((point, pointIndex) => ({
                x: (pointIndex / (maxPoints - 1 || 1)) * chartWidth,
                y: 100 - (point.value / maxValue) * 100,
                ...point,
              }));

              if (chartType === 'area') {
                // Validate points array before rendering
                if (
                  points.length === 0 ||
                  points.some((p) => p.x === undefined || p.y === undefined)
                ) {
                  return null;
                }
                const pathData = `M ${points[0].x} ${points[0].y} ${points
                  .slice(1)
                  .map((p) => `L ${p.x} ${p.y}`)
                  .join(' ')} L ${points[points.length - 1].x} 100 L 0 100 Z`;
                return (
                  <g key={line.id}>
                    <path d={pathData} fill={color} fillOpacity={0.1} />
                    <polyline
                      fill="none"
                      stroke={color}
                      strokeWidth="2"
                      points={points.map((p) => `${p.x},${p.y}`).join(' ')}
                    />
                  </g>
                );
              }

              if (chartType === 'bar') {
                const barWidth = (chartWidth / maxPoints / data.length) * 0.8;
                return (
                  <g key={line.id}>
                    {points.map((point, i) => (
                      <rect
                        key={`bar-${line.id}-${i}`}
                        x={`${point.x - barWidth / 2 + lineIndex * barWidth}%`}
                        y={`${point.y}%`}
                        width={`${barWidth}%`}
                        height={`${100 - point.y}%`}
                        fill={color}
                        opacity={0.8}
                        className={onDrillDown ? 'cursor-pointer hover:opacity-100' : ''}
                        onClick={() => onDrillDown?.({ date: point.date, value: point.value })}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter' || e.key === ' ') {
                            onDrillDown?.({ date: point.date, value: point.value });
                          }
                        }}
                        role={onDrillDown ? 'button' : undefined}
                        tabIndex={onDrillDown ? 0 : undefined}
                        aria-label={
                          onDrillDown
                            ? `View details for ${point.date}: ${formatValue(point.value)}`
                            : undefined
                        }
                      />
                    ))}
                  </g>
                );
              }

              // Line chart (default)
              return (
                <g key={line.id}>
                  <polyline
                    fill="none"
                    stroke={color}
                    strokeWidth="2"
                    points={points.map((p) => `${p.x}%,${p.y}%`).join(' ')}
                  />
                  {points.map((point, i) => (
                    <circle
                      key={`point-${line.id}-${i}`}
                      cx={`${point.x}%`}
                      cy={`${point.y}%`}
                      r="4"
                      fill={color}
                      className={onDrillDown ? 'cursor-pointer hover:r-6' : ''}
                      onClick={() => onDrillDown?.({ date: point.date, value: point.value })}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter' || e.key === ' ') {
                          onDrillDown?.({ date: point.date, value: point.value });
                        }
                      }}
                      role={onDrillDown ? 'button' : undefined}
                      tabIndex={onDrillDown ? 0 : undefined}
                      aria-label={
                        onDrillDown
                          ? `View details for ${point.label || point.date}: ${formatValue(point.value)}`
                          : undefined
                      }
                    >
                      <title>{`${point.label || point.date}: ${formatValue(point.value)}`}</title>
                    </circle>
                  ))}
                </g>
              );
            })}
          </svg>

          {/* X-axis labels */}
          <div className="absolute bottom-0 left-0 right-0 flex justify-between text-xs text-gray-400 transform translate-y-full pt-2">
            {data[0]?.data
              .filter(
                (_, i, arr) =>
                  i === 0 || i === arr.length - 1 || i % Math.ceil(arr.length / 6) === 0
              )
              .map((point) => (
                <span key={point.date}>
                  {point.label ||
                    new Date(point.date).toLocaleDateString('en-US', {
                      month: 'short',
                      day: 'numeric',
                    })}
                </span>
              ))}
          </div>
        </div>
      </div>
    </div>
  );
}
