/**
 * Outages feature module barrel export.
 * UC-12: Utility Outages
 */

// Components
export {
  OutageCard,
  OutageList,
  OutageForm,
} from './components';

export type {
  OutageSummary,
  OutageStatus,
  OutageCommodity,
  OutageSeverity,
  OutageFormData,
} from './components';

// Pages
export {
  OutagesPage,
  ViewOutagePage,
  CreateOutagePage,
  EditOutagePage,
} from './pages';

export type { ListOutagesParams, OutageDetail } from './pages';
