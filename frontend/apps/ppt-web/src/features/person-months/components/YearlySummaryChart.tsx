/**
 * YearlySummaryChart component - displays yearly person-month trend.
 * Shows a bar chart of person counts across months for a year.
 */

import { useTranslation } from 'react-i18next';

export interface MonthlyData {
  month: number;
  personCount: number;
}

export interface YearlySummary {
  year: number;
  totalPersonMonths: number;
  averagePersons: number;
  monthlyData: MonthlyData[];
}

interface YearlySummaryChartProps {
  summary: YearlySummary;
  onMonthClick?: (month: number) => void;
}

const monthKeys = [
  'jan',
  'feb',
  'mar',
  'apr',
  'may',
  'jun',
  'jul',
  'aug',
  'sep',
  'oct',
  'nov',
  'dec',
];

export function YearlySummaryChart({ summary, onMonthClick }: YearlySummaryChartProps) {
  const { t } = useTranslation();

  const maxPersonCount = Math.max(...summary.monthlyData.map((d) => d.personCount), 1);

  // Create a map for quick lookup
  const monthDataMap = new Map(summary.monthlyData.map((d) => [d.month, d.personCount]));

  // Generate all 12 months
  const allMonths = Array.from({ length: 12 }, (_, i) => ({
    month: i + 1,
    personCount: monthDataMap.get(i + 1) ?? 0,
  }));

  const getBarHeight = (count: number): string => {
    if (count === 0) return '4px';
    return `${Math.max((count / maxPersonCount) * 100, 10)}%`;
  };

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-medium text-gray-900">
          {t('personMonths.yearlySummary')} - {summary.year}
        </h3>
        <div className="flex gap-4 text-sm">
          <div className="text-gray-600">
            <span className="font-medium text-gray-900">{summary.totalPersonMonths}</span>{' '}
            {t('personMonths.totalPersonMonths')}
          </div>
          <div className="text-gray-600">
            <span className="font-medium text-gray-900">{summary.averagePersons.toFixed(1)}</span>{' '}
            {t('personMonths.avgPersons')}
          </div>
        </div>
      </div>

      {/* Bar chart */}
      <div className="h-48 flex items-end justify-between gap-2">
        {allMonths.map((data) => (
          <div key={data.month} className="flex-1 flex flex-col items-center gap-1">
            <span className="text-xs text-gray-600 font-medium">{data.personCount}</span>
            <button
              type="button"
              onClick={() => onMonthClick?.(data.month)}
              className={`w-full rounded-t transition-all duration-300 ${
                data.personCount > 0
                  ? 'bg-blue-500 hover:bg-blue-600 cursor-pointer'
                  : 'bg-gray-200 cursor-default'
              }`}
              style={{ height: getBarHeight(data.personCount) }}
              title={`${t(`personMonths.months.${monthKeys[data.month - 1]}`)}: ${data.personCount} ${
                data.personCount === 1 ? t('personMonths.person') : t('personMonths.persons')
              }`}
              disabled={data.personCount === 0 && !onMonthClick}
            />
            <span className="text-xs text-gray-500">
              {t(`personMonths.monthsShort.${monthKeys[data.month - 1]}`)}
            </span>
          </div>
        ))}
      </div>

      {/* Legend */}
      <div className="mt-6 pt-4 border-t flex items-center justify-center gap-6 text-sm text-gray-600">
        <div className="flex items-center gap-2">
          <span className="w-3 h-3 rounded bg-blue-500" />
          <span>{t('personMonths.hasData')}</span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-3 h-3 rounded bg-gray-200" />
          <span>{t('personMonths.noData')}</span>
        </div>
      </div>
    </div>
  );
}
