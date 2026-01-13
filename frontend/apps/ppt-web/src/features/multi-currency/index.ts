/**
 * Multi-Currency & Cross-Border Support Feature (Epic 145)
 *
 * Exports for multi-currency configuration, exchange rate management,
 * cross-currency transactions, cross-border lease management, and reporting.
 */

// Components
export { CurrencyConfigForm } from './components/CurrencyConfigForm';
export type { CurrencyConfig } from './components/CurrencyConfigForm';
export { CurrencySelector } from './components/CurrencySelector';
export { ExchangeRateCard } from './components/ExchangeRateCard';
export { ExchangeRateTable } from './components/ExchangeRateTable';
export { TransactionsList } from './components/TransactionsList';
export { CrossBorderLeaseCard } from './components/CrossBorderLeaseCard';
export { CurrencyExposureChart } from './components/CurrencyExposureChart';
export { ReportConfigForm } from './components/ReportConfigForm';
export type { ReportConfig } from './components/ReportConfigForm';

// Pages
export { MultiCurrencyDashboardPage } from './pages/MultiCurrencyDashboardPage';
export { ExchangeRatesPage } from './pages/ExchangeRatesPage';
export { CrossBorderLeasesPage } from './pages/CrossBorderLeasesPage';
export { CurrencyReportsPage } from './pages/CurrencyReportsPage';
