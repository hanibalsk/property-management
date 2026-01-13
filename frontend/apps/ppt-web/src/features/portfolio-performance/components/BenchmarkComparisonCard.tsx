// Epic 144: Portfolio Performance Analytics - Benchmark Comparison Card Component
import type React from 'react';

interface MetricComparison {
  metricName: string;
  actualValue?: number;
  benchmarkValue?: number;
  variance?: number;
  variancePct?: number;
  percentile?: number;
  status: string;
}

interface BenchmarkComparisonCardProps {
  benchmarkName: string;
  benchmarkSource: string;
  comparisonDate: string;
  metrics: MetricComparison[];
  overallPerformance: string;
  overallPercentile?: number;
  performanceScore?: number;
}

export const BenchmarkComparisonCard: React.FC<BenchmarkComparisonCardProps> = ({
  benchmarkName,
  benchmarkSource,
  comparisonDate,
  metrics,
  overallPerformance,
  overallPercentile,
  performanceScore,
}) => {
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'above_benchmark':
        return 'text-green-600 bg-green-100';
      case 'at_benchmark':
        return 'text-blue-600 bg-blue-100';
      case 'below_benchmark':
        return 'text-red-600 bg-red-100';
      default:
        return 'text-gray-600 bg-gray-100';
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case 'above_benchmark':
        return 'Above';
      case 'at_benchmark':
        return 'At';
      case 'below_benchmark':
        return 'Below';
      default:
        return 'N/A';
    }
  };

  const getPerformanceColor = (performance: string) => {
    switch (performance.toLowerCase()) {
      case 'excellent':
        return 'text-green-600';
      case 'good':
        return 'text-blue-600';
      case 'fair':
        return 'text-yellow-600';
      case 'poor':
        return 'text-red-600';
      default:
        return 'text-gray-600';
    }
  };

  const formatMetricName = (name: string) => {
    return name.replace(/_/g, ' ').replace(/\b\w/g, (l) => l.toUpperCase());
  };

  const formatValue = (value?: number) => {
    if (value === undefined || value === null) return '-';
    return value.toFixed(2);
  };

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <div className="flex justify-between items-start mb-4">
        <div>
          <h3 className="text-lg font-semibold text-gray-900">{benchmarkName}</h3>
          <p className="text-sm text-gray-500">
            {benchmarkSource} | {comparisonDate}
          </p>
        </div>
        <div className="text-right">
          <p className={`text-lg font-bold ${getPerformanceColor(overallPerformance)}`}>
            {overallPerformance}
          </p>
          {overallPercentile !== undefined && (
            <p className="text-sm text-gray-500">{overallPercentile}th percentile</p>
          )}
        </div>
      </div>

      {performanceScore !== undefined && (
        <div className="mb-4">
          <div className="flex justify-between items-center mb-1">
            <span className="text-sm text-gray-500">Performance Score</span>
            <span className="text-sm font-semibold">{performanceScore.toFixed(0)}/100</span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2">
            <div
              className={`h-2 rounded-full ${
                performanceScore >= 80
                  ? 'bg-green-500'
                  : performanceScore >= 60
                    ? 'bg-blue-500'
                    : performanceScore >= 40
                      ? 'bg-yellow-500'
                      : 'bg-red-500'
              }`}
              style={{ width: `${performanceScore}%` }}
            />
          </div>
        </div>
      )}

      <div className="space-y-3">
        {metrics.map((metric, index) => (
          <div
            key={index}
            className="flex items-center justify-between py-2 border-b border-gray-100 last:border-0"
          >
            <div className="flex-1">
              <p className="text-sm font-medium text-gray-900">
                {formatMetricName(metric.metricName)}
              </p>
              <div className="flex items-center space-x-2 text-xs text-gray-500">
                <span>Actual: {formatValue(metric.actualValue)}%</span>
                <span>|</span>
                <span>Benchmark: {formatValue(metric.benchmarkValue)}%</span>
              </div>
            </div>
            <div className="flex items-center space-x-2">
              {metric.variance !== undefined && (
                <span
                  className={`text-sm font-semibold ${metric.variance >= 0 ? 'text-green-600' : 'text-red-600'}`}
                >
                  {metric.variance >= 0 ? '+' : ''}
                  {metric.variance.toFixed(2)}
                </span>
              )}
              <span className={`px-2 py-1 text-xs rounded-full ${getStatusColor(metric.status)}`}>
                {getStatusLabel(metric.status)}
              </span>
            </div>
          </div>
        ))}
      </div>

      {metrics.length === 0 && (
        <p className="text-center text-gray-500 py-4">No metrics available for comparison</p>
      )}
    </div>
  );
};
