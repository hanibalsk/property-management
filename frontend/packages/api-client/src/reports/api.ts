/**
 * Reports API client (Epic 53).
 *
 * Client functions for reports, schedules, analytics, and trends.
 */

import type {
  AnalyticsParams,
  AnalyticsSummary,
  BuildingAnalytics,
  CreateReportDefinition,
  CreateReportSchedule,
  DataSource,
  ListReportsParams,
  ListReportsResponse,
  ListSchedulesParams,
  ListSchedulesResponse,
  PeriodComparison,
  ReportDefinition,
  ReportResult,
  ReportRun,
  ReportSchedule,
  TrendAnalysis,
  TrendLine,
  TrendParams,
} from './types';

const API_BASE = '/api/v1/reports';

// NOTE: Authentication headers (Authorization, etc.) are handled at a higher level
// by the global fetch interceptor or API client wrapper, not in individual API calls.

async function fetchApi<T>(url: string, options?: RequestInit): Promise<T> {
  const response = await fetch(url, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'Request failed' }));
    throw new Error(error.message || `HTTP ${response.status}`);
  }

  return response.json();
}

// ============================================================================
// REPORT DEFINITIONS
// ============================================================================

export async function createReport(
  organizationId: string,
  data: CreateReportDefinition
): Promise<ReportDefinition> {
  return fetchApi(`${API_BASE}/definitions`, {
    method: 'POST',
    body: JSON.stringify({ organization_id: organizationId, ...data }),
  });
}

export async function listReports(params: ListReportsParams): Promise<ListReportsResponse> {
  const searchParams = new URLSearchParams();
  searchParams.set('organization_id', params.organization_id);
  if (params.type) searchParams.set('type', params.type);
  if (params.created_by) searchParams.set('created_by', params.created_by);
  if (params.limit) searchParams.set('limit', String(params.limit));
  if (params.offset) searchParams.set('offset', String(params.offset));
  return fetchApi(`${API_BASE}/definitions?${searchParams}`);
}

export async function getReport(id: string): Promise<ReportDefinition> {
  return fetchApi(`${API_BASE}/definitions/${id}`);
}

export async function updateReport(
  id: string,
  data: Partial<CreateReportDefinition>
): Promise<ReportDefinition> {
  return fetchApi(`${API_BASE}/definitions/${id}`, {
    method: 'PATCH',
    body: JSON.stringify(data),
  });
}

export async function deleteReport(id: string): Promise<void> {
  await fetchApi(`${API_BASE}/definitions/${id}`, { method: 'DELETE' });
}

export async function runReport(id: string): Promise<ReportResult> {
  return fetchApi(`${API_BASE}/definitions/${id}/run`, { method: 'POST' });
}

export async function previewReport(data: CreateReportDefinition): Promise<ReportResult> {
  return fetchApi(`${API_BASE}/preview`, {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

// ============================================================================
// DATA SOURCES
// ============================================================================

export async function listDataSources(organizationId: string): Promise<DataSource[]> {
  return fetchApi(`${API_BASE}/data-sources?organization_id=${organizationId}`);
}

export async function getDataSourceFields(sourceId: string): Promise<DataSource> {
  return fetchApi(`${API_BASE}/data-sources/${sourceId}`);
}

// ============================================================================
// REPORT SCHEDULES
// ============================================================================

export async function createSchedule(
  organizationId: string,
  data: CreateReportSchedule
): Promise<ReportSchedule> {
  return fetchApi(`${API_BASE}/schedules`, {
    method: 'POST',
    body: JSON.stringify({ organization_id: organizationId, ...data }),
  });
}

export async function listSchedules(params: ListSchedulesParams): Promise<ListSchedulesResponse> {
  const searchParams = new URLSearchParams();
  searchParams.set('organization_id', params.organization_id);
  if (params.report_id) searchParams.set('report_id', params.report_id);
  if (params.is_active !== undefined) searchParams.set('is_active', String(params.is_active));
  return fetchApi(`${API_BASE}/schedules?${searchParams}`);
}

export async function getSchedule(id: string): Promise<ReportSchedule> {
  return fetchApi(`${API_BASE}/schedules/${id}`);
}

export async function updateSchedule(
  id: string,
  data: Partial<CreateReportSchedule>
): Promise<ReportSchedule> {
  return fetchApi(`${API_BASE}/schedules/${id}`, {
    method: 'PATCH',
    body: JSON.stringify(data),
  });
}

export async function deleteSchedule(id: string): Promise<void> {
  await fetchApi(`${API_BASE}/schedules/${id}`, { method: 'DELETE' });
}

export async function toggleSchedule(id: string, isActive: boolean): Promise<ReportSchedule> {
  return fetchApi(`${API_BASE}/schedules/${id}/toggle`, {
    method: 'POST',
    body: JSON.stringify({ is_active: isActive }),
  });
}

export async function runScheduleNow(id: string): Promise<ReportRun> {
  return fetchApi(`${API_BASE}/schedules/${id}/run`, { method: 'POST' });
}

export async function listScheduleRuns(scheduleId: string, limit?: number): Promise<ReportRun[]> {
  const params = limit ? `?limit=${limit}` : '';
  return fetchApi(`${API_BASE}/schedules/${scheduleId}/runs${params}`);
}

// ============================================================================
// ANALYTICS
// ============================================================================

export async function getAnalyticsSummary(params: AnalyticsParams): Promise<AnalyticsSummary> {
  const searchParams = new URLSearchParams();
  searchParams.set('organization_id', params.organization_id);
  if (params.building_id) searchParams.set('building_id', params.building_id);
  if (params.start_date) searchParams.set('start_date', params.start_date);
  if (params.end_date) searchParams.set('end_date', params.end_date);
  return fetchApi(`${API_BASE}/analytics/summary?${searchParams}`);
}

export async function getBuildingAnalytics(
  buildingId: string,
  params?: Omit<AnalyticsParams, 'organization_id' | 'building_id'>
): Promise<BuildingAnalytics> {
  const searchParams = new URLSearchParams();
  if (params?.start_date) searchParams.set('start_date', params.start_date);
  if (params?.end_date) searchParams.set('end_date', params.end_date);
  return fetchApi(`${API_BASE}/analytics/buildings/${buildingId}?${searchParams}`);
}

// ============================================================================
// TREND ANALYSIS
// ============================================================================

export async function getTrendAnalysis(params: TrendParams): Promise<TrendAnalysis> {
  const searchParams = new URLSearchParams();
  searchParams.set('organization_id', params.organization_id);
  searchParams.set('metric', params.metric);
  searchParams.set('period', params.period);
  searchParams.set('start_date', params.start_date);
  searchParams.set('end_date', params.end_date);
  if (params.building_id) searchParams.set('building_id', params.building_id);
  if (params.compare_previous) searchParams.set('compare_previous', 'true');
  return fetchApi(`${API_BASE}/trends?${searchParams}`);
}

export async function getTrendLines(params: TrendParams): Promise<TrendLine[]> {
  const searchParams = new URLSearchParams();
  searchParams.set('organization_id', params.organization_id);
  searchParams.set('metric', params.metric);
  searchParams.set('period', params.period);
  searchParams.set('start_date', params.start_date);
  searchParams.set('end_date', params.end_date);
  if (params.building_id) searchParams.set('building_id', params.building_id);
  return fetchApi(`${API_BASE}/trends/lines?${searchParams}`);
}

export async function comparePeriods(
  organizationId: string,
  metric: string,
  periods: { start_date: string; end_date: string; label: string }[]
): Promise<PeriodComparison> {
  return fetchApi(`${API_BASE}/trends/compare`, {
    method: 'POST',
    body: JSON.stringify({ organization_id: organizationId, metric, periods }),
  });
}

// ============================================================================
// EXPORT
// ============================================================================

export async function exportReport(
  reportId: string,
  format: 'pdf' | 'excel' | 'csv'
): Promise<{ download_url: string }> {
  return fetchApi(`${API_BASE}/definitions/${reportId}/export`, {
    method: 'POST',
    body: JSON.stringify({ format }),
  });
}

export async function exportDashboard(
  dashboardId: string,
  format: 'pdf'
): Promise<{ download_url: string }> {
  return fetchApi(`${API_BASE}/dashboards/${dashboardId}/export`, {
    method: 'POST',
    body: JSON.stringify({ format }),
  });
}
