/**
 * ReportsPage - Main reports and analytics page.
 *
 * Combines all Story 53.x components into a unified reports interface.
 */

import type {
  BuildingAnalytics,
  DataSource,
  KPIMetric,
  PeriodComparison,
  ReportDefinition,
  ReportSchedule,
  TrendAnalysis,
  TrendLine,
} from '@ppt/api-client';
import { useState } from 'react';
import {
  AnalyticsChart,
  BuildingMetricsCard,
  KPICard,
  PeriodComparisonChart,
  ReportBuilder,
  ScheduleForm,
  ScheduleList,
  TrendChart,
} from '../components';

interface ReportsPageProps {
  organizationId: string;
  dataSources: DataSource[];
  reports: ReportDefinition[];
  schedules: ReportSchedule[];
  kpis: KPIMetric[];
  buildings: BuildingAnalytics[];
  trendAnalysis: TrendAnalysis;
  trendLines: TrendLine[];
  periodComparison: PeriodComparison;
  isLoading?: boolean;
  onCreateReport?: (data: unknown) => Promise<void>;
  onPreviewReport?: (data: unknown) => Promise<unknown>;
  onCreateSchedule?: (data: unknown) => Promise<void>;
  onDeleteSchedule?: (id: string) => Promise<void>;
  onToggleSchedule?: (id: string, isActive: boolean) => Promise<void>;
  onRunScheduleNow?: (id: string) => Promise<void>;
  onPeriodChange?: (period: 'daily' | 'weekly' | 'monthly' | 'quarterly' | 'yearly') => void;
}

type Tab = 'dashboard' | 'reports' | 'schedules';

export function ReportsPage({
  dataSources,
  reports,
  schedules,
  kpis,
  buildings,
  trendAnalysis,
  trendLines,
  periodComparison,
  isLoading,
  onCreateReport,
  onPreviewReport,
  onCreateSchedule,
  onDeleteSchedule,
  onToggleSchedule,
  onRunScheduleNow,
  onPeriodChange,
}: ReportsPageProps) {
  const [activeTab, setActiveTab] = useState<Tab>('dashboard');
  const [showReportBuilder, setShowReportBuilder] = useState(false);
  const [showScheduleForm, setShowScheduleForm] = useState(false);
  const [selectedPeriod, setSelectedPeriod] = useState<
    'daily' | 'weekly' | 'monthly' | 'quarterly' | 'yearly'
  >('monthly');

  const handlePeriodChange = (period: 'daily' | 'weekly' | 'monthly' | 'quarterly' | 'yearly') => {
    setSelectedPeriod(period);
    onPeriodChange?.(period);
  };

  const tabs = [
    { id: 'dashboard' as Tab, label: 'Dashboard', icon: '📊' },
    { id: 'reports' as Tab, label: 'Reports', icon: '📋' },
    { id: 'schedules' as Tab, label: 'Schedules', icon: '⏰' },
  ];

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <h1 className="text-2xl font-bold text-gray-900">Reports & Analytics</h1>
            <div className="flex gap-3">
              {activeTab === 'reports' && (
                <button
                  type="button"
                  onClick={() => setShowReportBuilder(true)}
                  className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
                >
                  + New Report
                </button>
              )}
              {activeTab === 'schedules' && (
                <button
                  type="button"
                  onClick={() => setShowScheduleForm(true)}
                  className="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700"
                >
                  + New Schedule
                </button>
              )}
            </div>
          </div>

          {/* Tabs */}
          <div className="mt-4 border-b border-gray-200">
            <nav className="-mb-px flex gap-8">
              {tabs.map((tab) => (
                <button
                  key={tab.id}
                  type="button"
                  onClick={() => setActiveTab(tab.id)}
                  className={`py-4 px-1 border-b-2 font-medium text-sm transition-colors ${
                    activeTab === tab.id
                      ? 'border-blue-500 text-blue-600'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  <span className="mr-2">{tab.icon}</span>
                  {tab.label}
                </button>
              ))}
            </nav>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Dashboard Tab */}
        {activeTab === 'dashboard' && (
          <div className="space-y-8">
            {/* KPIs */}
            <section>
              <h2 className="text-lg font-medium text-gray-900 mb-4">Key Metrics</h2>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                {kpis.map((kpi) => (
                  <KPICard key={kpi.id} metric={kpi} />
                ))}
              </div>
            </section>

            {/* Trend Analysis */}
            <section>
              <h2 className="text-lg font-medium text-gray-900 mb-4">Trends</h2>
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <TrendChart
                  title="Revenue Trend"
                  analysis={trendAnalysis}
                  trendLines={trendLines}
                  selectedPeriod={selectedPeriod}
                  onPeriodChange={handlePeriodChange}
                  isLoading={isLoading}
                />
                <PeriodComparisonChart comparison={periodComparison} isLoading={isLoading} />
              </div>
            </section>

            {/* Charts */}
            <section>
              <h2 className="text-lg font-medium text-gray-900 mb-4">Analytics</h2>
              <AnalyticsChart
                title="Monthly Performance"
                data={trendLines}
                chartType="area"
                isLoading={isLoading}
              />
            </section>

            {/* Building Metrics */}
            <section>
              <h2 className="text-lg font-medium text-gray-900 mb-4">Building Performance</h2>
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {buildings.map((building) => (
                  <BuildingMetricsCard key={building.building_id} analytics={building} />
                ))}
              </div>
            </section>
          </div>
        )}

        {/* Reports Tab */}
        {activeTab === 'reports' && (
          <div>
            {showReportBuilder ? (
              <ReportBuilder
                dataSources={dataSources}
                onSave={async (data) => {
                  await onCreateReport?.(data);
                  setShowReportBuilder(false);
                }}
                onPreview={async (data) => {
                  const result = await onPreviewReport?.(data);
                  return result as unknown as import('@ppt/api-client').ReportResult;
                }}
                onCancel={() => setShowReportBuilder(false)}
              />
            ) : (
              <div className="bg-white rounded-lg shadow">
                <div className="px-6 py-4 border-b border-gray-200">
                  <h2 className="text-lg font-medium text-gray-900">Saved Reports</h2>
                </div>
                {reports.length === 0 ? (
                  <div className="p-12 text-center">
                    <svg
                      className="w-12 h-12 text-gray-400 mx-auto mb-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      aria-hidden="true"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                      />
                    </svg>
                    <p className="text-gray-500">No reports created yet</p>
                    <button
                      type="button"
                      onClick={() => setShowReportBuilder(true)}
                      className="mt-4 text-blue-600 hover:text-blue-800 font-medium"
                    >
                      Create your first report
                    </button>
                  </div>
                ) : (
                  <div className="divide-y divide-gray-200">
                    {reports.map((report) => (
                      <div
                        key={report.id}
                        className="px-6 py-4 hover:bg-gray-50 flex items-center justify-between"
                      >
                        <div>
                          <h3 className="text-sm font-medium text-gray-900">{report.name}</h3>
                          {report.description && (
                            <p className="text-sm text-gray-500 mt-1">{report.description}</p>
                          )}
                          <p className="text-xs text-gray-400 mt-1">
                            Created{' '}
                            {new Date(report.created_at).toLocaleDateString('en-US', {
                              month: 'short',
                              day: 'numeric',
                              year: 'numeric',
                            })}
                          </p>
                        </div>
                        <div className="flex items-center gap-2">
                          {report.is_public && (
                            <span className="px-2 py-0.5 text-xs bg-green-100 text-green-700 rounded-full">
                              Public
                            </span>
                          )}
                          <span className="px-2 py-0.5 text-xs bg-gray-100 text-gray-700 rounded-full">
                            {report.report_type}
                          </span>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}
          </div>
        )}

        {/* Schedules Tab */}
        {activeTab === 'schedules' && (
          <div>
            {showScheduleForm ? (
              <div className="bg-white rounded-lg shadow p-6">
                <h2 className="text-lg font-medium text-gray-900 mb-6">Create Schedule</h2>
                <ScheduleForm
                  reports={reports}
                  onSubmit={async (data) => {
                    await onCreateSchedule?.(data);
                    setShowScheduleForm(false);
                  }}
                  onCancel={() => setShowScheduleForm(false)}
                />
              </div>
            ) : (
              <ScheduleList
                schedules={schedules}
                isLoading={isLoading}
                onEdit={(schedule) => {
                  console.log('Edit schedule:', schedule.id);
                }}
                onDelete={async (id) => {
                  await onDeleteSchedule?.(id);
                }}
                onToggle={async (id, isActive) => {
                  await onToggleSchedule?.(id, isActive);
                }}
                onRunNow={async (id) => {
                  await onRunScheduleNow?.(id);
                }}
                onViewHistory={(id) => {
                  console.log('View history:', id);
                }}
              />
            )}
          </div>
        )}
      </div>
    </div>
  );
}
