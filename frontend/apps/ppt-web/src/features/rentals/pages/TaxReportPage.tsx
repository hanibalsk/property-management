/**
 * TaxReportPage - Tax report generation page with filters and export options.
 * Epic 18: Short-Term Rental Integration - UC-29 Tax Export
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TaxReportPreview } from '../components/TaxReportPreview';
import type {
  TaxExportFormat,
  TaxJurisdiction,
  TaxJurisdictionCountry,
  TaxReportData,
  TaxReportParams,
  TaxReportType,
} from '../types';

interface Building {
  id: string;
  name: string;
  units: { id: string; name: string }[];
}

interface TaxReportPageProps {
  buildings: Building[];
  onGenerateReport: (params: TaxReportParams) => Promise<TaxReportData>;
  onExportReport: (report: TaxReportData, format: TaxExportFormat) => Promise<void>;
  onBack?: () => void;
  isLoading?: boolean;
}

const JURISDICTIONS: TaxJurisdiction[] = [
  {
    country: 'SK',
    name: 'Slovakia',
    defaultTaxRate: 0.19,
    requiresGuestRegistration: true,
    localTaxPerNight: 1.5,
    localTaxCurrency: 'EUR',
  },
  {
    country: 'CZ',
    name: 'Czech Republic',
    defaultTaxRate: 0.15,
    requiresGuestRegistration: true,
    localTaxPerNight: 50,
    localTaxCurrency: 'CZK',
  },
  {
    country: 'DE',
    name: 'Germany',
    defaultTaxRate: 0.19,
    requiresGuestRegistration: true,
    localTaxPerNight: 5,
    localTaxCurrency: 'EUR',
  },
  {
    country: 'AT',
    name: 'Austria',
    defaultTaxRate: 0.2,
    requiresGuestRegistration: true,
    localTaxPerNight: 3.02,
    localTaxCurrency: 'EUR',
  },
  {
    country: 'HU',
    name: 'Hungary',
    defaultTaxRate: 0.15,
    requiresGuestRegistration: true,
    localTaxPerNight: 500,
    localTaxCurrency: 'HUF',
  },
  {
    country: 'PL',
    name: 'Poland',
    defaultTaxRate: 0.12,
    requiresGuestRegistration: false,
    localTaxPerNight: 3,
    localTaxCurrency: 'PLN',
  },
];

const REPORT_TYPES: { value: TaxReportType; labelKey: string; descriptionKey: string }[] = [
  {
    value: 'annual_summary',
    labelKey: 'rentals.tax.reportTypes.annual_summary',
    descriptionKey: 'rentals.tax.reportTypeDesc.annual_summary',
  },
  {
    value: 'monthly_breakdown',
    labelKey: 'rentals.tax.reportTypes.monthly_breakdown',
    descriptionKey: 'rentals.tax.reportTypeDesc.monthly_breakdown',
  },
  {
    value: 'per_booking',
    labelKey: 'rentals.tax.reportTypes.per_booking',
    descriptionKey: 'rentals.tax.reportTypeDesc.per_booking',
  },
];

function getYearOptions(): number[] {
  const currentYear = new Date().getFullYear();
  return [currentYear, currentYear - 1, currentYear - 2, currentYear - 3];
}

export function TaxReportPage({
  buildings,
  onGenerateReport,
  onExportReport,
  onBack,
  isLoading: isPageLoading,
}: TaxReportPageProps) {
  const { t } = useTranslation();
  const yearOptions = getYearOptions();

  const [year, setYear] = useState<number>(yearOptions[0]);
  const [reportType, setReportType] = useState<TaxReportType>('annual_summary');
  const [jurisdiction, setJurisdiction] = useState<TaxJurisdictionCountry>('SK');
  const [includeAllProperties, setIncludeAllProperties] = useState<boolean>(true);
  const [selectedBuildings, setSelectedBuildings] = useState<string[]>([]);
  const [selectedUnits, setSelectedUnits] = useState<string[]>([]);

  const [isGenerating, setIsGenerating] = useState(false);
  const [isExporting, setIsExporting] = useState(false);
  const [generatedReport, setGeneratedReport] = useState<TaxReportData | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleBuildingToggle = (buildingId: string) => {
    setSelectedBuildings((prev) =>
      prev.includes(buildingId) ? prev.filter((id) => id !== buildingId) : [...prev, buildingId]
    );
    // Clear units from this building if deselected
    const building = buildings.find((b) => b.id === buildingId);
    if (building && selectedBuildings.includes(buildingId)) {
      setSelectedUnits((prev) =>
        prev.filter((unitId) => !building.units.some((u) => u.id === unitId))
      );
    }
  };

  const handleUnitToggle = (unitId: string) => {
    setSelectedUnits((prev) =>
      prev.includes(unitId) ? prev.filter((id) => id !== unitId) : [...prev, unitId]
    );
  };

  const handleGenerateReport = async () => {
    setError(null);
    setIsGenerating(true);

    try {
      const params: TaxReportParams = {
        year,
        reportType,
        jurisdiction,
        includeAllProperties,
        buildingIds: includeAllProperties ? undefined : selectedBuildings,
        unitIds: includeAllProperties ? undefined : selectedUnits,
      };

      const report = await onGenerateReport(params);
      setGeneratedReport(report);
    } catch {
      setError(t('rentals.tax.errors.generateFailed'));
    } finally {
      setIsGenerating(false);
    }
  };

  const handleExport = async (format: TaxExportFormat) => {
    if (!generatedReport) return;

    setIsExporting(true);
    try {
      await onExportReport(generatedReport, format);
    } catch {
      setError(t('rentals.tax.errors.exportFailed'));
    } finally {
      setIsExporting(false);
    }
  };

  const selectedJurisdiction = JURISDICTIONS.find((j) => j.country === jurisdiction);

  if (isPageLoading) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600" />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-4">
              {onBack && (
                <button
                  type="button"
                  onClick={onBack}
                  className="p-2 text-gray-400 hover:text-gray-600 rounded-full"
                >
                  <svg
                    className="w-6 h-6"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M10 19l-7-7m0 0l7-7m-7 7h18"
                    />
                  </svg>
                </button>
              )}
              <div>
                <h1 className="text-2xl font-bold text-gray-900">{t('rentals.tax.pageTitle')}</h1>
                <p className="mt-1 text-sm text-gray-500">{t('rentals.tax.pageSubtitle')}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg">
            <div className="flex items-center gap-3">
              <svg
                className="w-5 h-5 text-red-400"
                fill="currentColor"
                viewBox="0 0 20 20"
                aria-hidden="true"
              >
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                  clipRule="evenodd"
                />
              </svg>
              <span className="text-sm text-red-700">{error}</span>
              <button
                type="button"
                onClick={() => setError(null)}
                className="ml-auto text-red-400 hover:text-red-600"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
                  <path
                    fillRule="evenodd"
                    d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                    clipRule="evenodd"
                  />
                </svg>
              </button>
            </div>
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Report Configuration */}
          <div className="lg:col-span-1">
            <div className="bg-white rounded-lg shadow">
              <div className="px-6 py-4 border-b">
                <h2 className="text-lg font-semibold text-gray-900">
                  {t('rentals.tax.configureReport')}
                </h2>
              </div>

              <div className="p-6 space-y-6">
                {/* Year Selector */}
                <div>
                  <label htmlFor="year" className="block text-sm font-medium text-gray-700 mb-2">
                    {t('rentals.tax.selectYear')}
                  </label>
                  <select
                    id="year"
                    value={year}
                    onChange={(e) => setYear(Number(e.target.value))}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                  >
                    {yearOptions.map((y) => (
                      <option key={y} value={y}>
                        {y}
                      </option>
                    ))}
                  </select>
                </div>

                {/* Jurisdiction Selector */}
                <div>
                  <label
                    htmlFor="jurisdiction"
                    className="block text-sm font-medium text-gray-700 mb-2"
                  >
                    {t('rentals.tax.selectJurisdiction')}
                  </label>
                  <select
                    id="jurisdiction"
                    value={jurisdiction}
                    onChange={(e) => setJurisdiction(e.target.value as TaxJurisdictionCountry)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                  >
                    {JURISDICTIONS.map((j) => (
                      <option key={j.country} value={j.country}>
                        {j.name} ({(j.defaultTaxRate * 100).toFixed(0)}%)
                      </option>
                    ))}
                  </select>
                  {selectedJurisdiction?.localTaxPerNight && (
                    <p className="mt-2 text-xs text-gray-500">
                      {t('rentals.tax.localTaxNote', {
                        amount: selectedJurisdiction.localTaxPerNight,
                        currency: selectedJurisdiction.localTaxCurrency,
                      })}
                    </p>
                  )}
                </div>

                {/* Report Type */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    {t('rentals.tax.selectReportType')}
                  </label>
                  <div className="space-y-3">
                    {REPORT_TYPES.map((type) => (
                      <label
                        key={type.value}
                        className={`flex items-start p-3 border rounded-lg cursor-pointer transition-colors ${
                          reportType === type.value
                            ? 'border-indigo-500 bg-indigo-50'
                            : 'border-gray-200 hover:border-gray-300'
                        }`}
                      >
                        <input
                          type="radio"
                          name="reportType"
                          value={type.value}
                          checked={reportType === type.value}
                          onChange={(e) => setReportType(e.target.value as TaxReportType)}
                          className="mt-0.5 h-4 w-4 text-indigo-600 focus:ring-indigo-500"
                        />
                        <div className="ml-3">
                          <span className="block text-sm font-medium text-gray-900">
                            {t(type.labelKey)}
                          </span>
                          <span className="block text-xs text-gray-500 mt-1">
                            {t(type.descriptionKey)}
                          </span>
                        </div>
                      </label>
                    ))}
                  </div>
                </div>

                {/* Property Selection */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    {t('rentals.tax.selectProperties')}
                  </label>
                  <div className="space-y-3">
                    <label className="flex items-center">
                      <input
                        type="checkbox"
                        checked={includeAllProperties}
                        onChange={(e) => setIncludeAllProperties(e.target.checked)}
                        className="h-4 w-4 text-indigo-600 focus:ring-indigo-500 rounded"
                      />
                      <span className="ml-2 text-sm text-gray-700">
                        {t('rentals.tax.includeAllProperties')}
                      </span>
                    </label>

                    {!includeAllProperties && (
                      <div className="ml-6 space-y-3 max-h-48 overflow-y-auto border rounded-lg p-3">
                        {buildings.map((building) => (
                          <div key={building.id}>
                            <label className="flex items-center">
                              <input
                                type="checkbox"
                                checked={selectedBuildings.includes(building.id)}
                                onChange={() => handleBuildingToggle(building.id)}
                                className="h-4 w-4 text-indigo-600 focus:ring-indigo-500 rounded"
                              />
                              <span className="ml-2 text-sm font-medium text-gray-700">
                                {building.name}
                              </span>
                            </label>
                            {selectedBuildings.includes(building.id) &&
                              building.units.length > 0 && (
                                <div className="ml-6 mt-2 space-y-1">
                                  {building.units.map((unit) => (
                                    <label key={unit.id} className="flex items-center">
                                      <input
                                        type="checkbox"
                                        checked={selectedUnits.includes(unit.id)}
                                        onChange={() => handleUnitToggle(unit.id)}
                                        className="h-3 w-3 text-indigo-600 focus:ring-indigo-500 rounded"
                                      />
                                      <span className="ml-2 text-xs text-gray-600">
                                        {unit.name}
                                      </span>
                                    </label>
                                  ))}
                                </div>
                              )}
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                </div>

                {/* Generate Button */}
                <button
                  type="button"
                  onClick={handleGenerateReport}
                  disabled={
                    isGenerating || (!includeAllProperties && selectedBuildings.length === 0)
                  }
                  className="w-full px-4 py-3 text-sm font-medium text-white bg-indigo-600 rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {isGenerating ? (
                    <span className="flex items-center justify-center">
                      <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2" />
                      {t('rentals.tax.generating')}
                    </span>
                  ) : (
                    t('rentals.tax.generateReport')
                  )}
                </button>
              </div>
            </div>
          </div>

          {/* Report Preview */}
          <div className="lg:col-span-2">
            {generatedReport ? (
              <TaxReportPreview
                report={generatedReport}
                onExport={handleExport}
                onClose={() => setGeneratedReport(null)}
                isExporting={isExporting}
              />
            ) : (
              <div className="bg-white rounded-lg shadow p-12 text-center">
                <div className="mx-auto w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mb-4">
                  <svg
                    className="w-8 h-8 text-gray-400"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    aria-hidden="true"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    />
                  </svg>
                </div>
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  {t('rentals.tax.noReportGenerated')}
                </h3>
                <p className="text-sm text-gray-500 max-w-sm mx-auto">
                  {t('rentals.tax.noReportGeneratedDesc')}
                </p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
