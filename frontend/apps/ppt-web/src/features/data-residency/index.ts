/**
 * Data Residency feature (Epic 146).
 *
 * Enhanced Data Residency Controls for regional compliance.
 */

// Components
export {
  AuditLogCard,
  ComplianceVerificationCard,
  DataResidencyConfigCard,
} from './components';
export type {
  AuditChange,
  AuditLogEntry,
  ComplianceImplication,
  ComplianceIssue,
  ComplianceVerificationResult,
  DataLocationSummary,
  DataRegionInfo,
  DataResidencyConfig,
  RegionAccessSummary,
} from './components';

// Pages
export { DataResidencyPage } from './pages';
