import { useCreateDispute, useDispute, useDisputes } from '@ppt/api-client';
import type {
  Dispute as ApiDispute,
  DisputeStatus as ApiDisputeStatus,
  DisputeType as ApiDisputeType,
} from '@ppt/api-client';
import { AccessibilityProvider, SkipNavigation } from '@ppt/ui-kit';
import { type ReactNode, useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { BrowserRouter, Link, Route, Routes, useNavigate, useParams } from 'react-router-dom';
import { ConnectionStatus, LanguageSwitcher, OfflineIndicator, ToastProvider, useToast } from './components';
import { AuthProvider, WebSocketProvider, useAuth } from './contexts';
import { DisputesPage, FileDisputePage } from './features/disputes';
import type {
  DisputeCategory,
  DisputePriority,
  DisputeSummary,
  DisputeStatus as UiDisputeStatus,
} from './features/disputes/components/DisputeCard';
import { DocumentDetailPage, DocumentUploadPage, DocumentsPage } from './features/documents';
import { EmergencyContactDirectoryPage } from './features/emergency';
import { ArticleDetailPage, NewsListPage } from './features/news';
import { PrivacySettingsPage } from './features/privacy';
import { AccessibilitySettingsPage } from './features/settings';
import { LoginPage } from './pages/LoginPage';

// ============================================
// Type Mapping Utilities (API <-> UI)
// ============================================

/** Map API DisputeType to UI DisputeCategory */
function mapTypeToCategory(type: ApiDisputeType): DisputeCategory {
  const mapping: Record<ApiDisputeType, DisputeCategory> = {
    noise: 'noise',
    damage: 'damage',
    payment: 'payment',
    lease: 'lease_terms',
    maintenance: 'maintenance',
    other: 'other',
  };
  return mapping[type];
}

/** Map UI DisputeCategory to API DisputeType */
function mapCategoryToType(category: DisputeCategory): ApiDisputeType {
  const mapping: Record<DisputeCategory, ApiDisputeType> = {
    noise: 'noise',
    damage: 'damage',
    payment: 'payment',
    lease_terms: 'lease',
    common_area: 'other',
    parking: 'other',
    pets: 'other',
    maintenance: 'maintenance',
    privacy: 'other',
    harassment: 'other',
    other: 'other',
  };
  return mapping[category];
}

/** Map API DisputeStatus to UI DisputeStatus */
function mapApiStatusToUiStatus(status: ApiDisputeStatus): UiDisputeStatus {
  const mapping: Record<ApiDisputeStatus, UiDisputeStatus> = {
    filed: 'filed',
    under_review: 'under_review',
    mediation: 'mediation',
    escalated: 'escalated',
    resolved: 'resolved',
    closed: 'closed',
  };
  return mapping[status];
}

/** Map UI DisputeStatus to API DisputeStatus (for filtering) */
function mapUiStatusToApiStatus(status: UiDisputeStatus): ApiDisputeStatus | undefined {
  const mapping: Record<UiDisputeStatus, ApiDisputeStatus | undefined> = {
    filed: 'filed',
    under_review: 'under_review',
    mediation: 'mediation',
    awaiting_response: 'under_review', // No direct mapping, use under_review
    resolved: 'resolved',
    escalated: 'escalated',
    withdrawn: 'closed', // No direct mapping, use closed
    closed: 'closed',
  };
  return mapping[status];
}

/** Transform API Dispute to UI DisputeSummary */
function transformDisputeToSummary(dispute: ApiDispute): DisputeSummary {
  return {
    id: dispute.id,
    referenceNumber: `DSP-${dispute.id.toUpperCase()}`,
    category: mapTypeToCategory(dispute.type),
    title: dispute.subject,
    status: mapApiStatusToUiStatus(dispute.status),
    // Priority is UI-only; API does not support priority field yet
    priority: 'medium' as DisputePriority,
    filedByName: dispute.filedBy,
    assignedToName: dispute.assignedMediator,
    partyCount: dispute.respondentId || dispute.respondent ? 2 : 1,
    createdAt: dispute.createdAt,
    updatedAt: dispute.updatedAt,
  };
}

/**
 * WebSocket wrapper that bridges AuthContext to WebSocketProvider.
 * Must be rendered inside AuthProvider to access auth state.
 */
function WebSocketWrapper({ children }: { children: ReactNode }) {
  const { getAccessToken, isAuthenticated } = useAuth();
  return (
    <WebSocketProvider
      auth={{
        accessToken: getAccessToken(),
        isAuthenticated,
      }}
    >
      {children}
    </WebSocketProvider>
  );
}

function AppNavigation() {
  const { t } = useTranslation();
  return (
    <nav className="app-nav" aria-label="Main navigation">
      <Link to="/">{t('nav.home')}</Link>
      <Link to="/documents">{t('nav.documents')}</Link>
      <Link to="/news">{t('nav.news')}</Link>
      <Link to="/emergency">{t('nav.emergency')}</Link>
      <Link to="/disputes">{t('nav.disputes')}</Link>
      <Link to="/settings/accessibility">{t('nav.accessibility')}</Link>
      <Link to="/settings/privacy">{t('nav.privacy')}</Link>
      <ConnectionStatus />
      <LanguageSwitcher />
    </nav>
  );
}

function App() {
  return (
    <AccessibilityProvider>
      <AuthProvider>
        <ToastProvider>
          <WebSocketWrapper>
            <BrowserRouter>
              <SkipNavigation mainContentId="main-content" />
              <OfflineIndicator />
              <div className="app">
                <AppNavigation />
                <main id="main-content">
                  <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/login" element={<LoginPage />} />
                    {/* Document Intelligence routes (Epic 39) */}
                    <Route path="/documents" element={<DocumentsPageRoute />} />
                    <Route path="/documents/upload" element={<DocumentUploadPage />} />
                    <Route path="/documents/:documentId" element={<DocumentDetailRoute />} />
                    {/* News routes (Epic 59) */}
                    <Route path="/news" element={<NewsListPage />} />
                    <Route path="/news/:articleId" element={<ArticleDetailRoute />} />
                    {/* Emergency contacts route (Epic 62) */}
                    <Route path="/emergency" element={<EmergencyContactDirectoryPage />} />
                    {/* Accessibility settings route (Epic 60) */}
                    <Route path="/settings/accessibility" element={<AccessibilitySettingsPage />} />
                    {/* Privacy settings route (Epic 63) */}
                    <Route path="/settings/privacy" element={<PrivacySettingsPage />} />
                    {/* Dispute Resolution routes (Epic 77) */}
                    <Route path="/disputes" element={<DisputesPageRoute />} />
                    <Route path="/disputes/new" element={<FileDisputePageRoute />} />
                    <Route path="/disputes/:disputeId" element={<DisputeDetailRoute />} />
                  </Routes>
                </main>
              </div>
            </BrowserRouter>
          </WebSocketWrapper>
        </ToastProvider>
      </AuthProvider>
    </AccessibilityProvider>
  );
}

/** Route wrapper for documents page */
function DocumentsPageRoute() {
  const { user } = useAuth();
  const organizationId = user?.organizationId ?? 'default-org';
  return <DocumentsPage organizationId={organizationId} />;
}

/** Route wrapper for document detail page to extract params */
function DocumentDetailRoute() {
  const { documentId } = useParams<{ documentId: string }>();
  if (!documentId) {
    return (
      <div className="error-page">
        <h1>Document not found</h1>
        <p>
          We couldn&apos;t find the document you&apos;re looking for. It may have been moved,
          deleted, or the link might be incorrect.
        </p>
        <Link to="/documents">Back to documents</Link>
      </div>
    );
  }
  return <DocumentDetailPage documentId={documentId} />;
}

/** Route wrapper for article detail page to extract params */
function ArticleDetailRoute() {
  const { articleId } = useParams<{ articleId: string }>();
  if (!articleId) return <div>Article not found</div>;
  return <ArticleDetailPage articleId={articleId} />;
}

/**
 * Route wrapper for disputes page (Epic 77, Story 80.1).
 *
 * Uses useDisputes hook from @ppt/api-client for data fetching.
 * Implements real API integration with TanStack Query.
 * Transforms API types to UI types for component compatibility.
 */
function DisputesPageRoute() {
  const navigate = useNavigate();
  const { user } = useAuth();
  const { showToast } = useToast();

  // Require organization context for disputes
  if (!user?.organizationId) {
    return (
      <div className="error-page">
        <h1>Authentication Required</h1>
        <p>Unable to load disputes: missing organization context. Please sign in again.</p>
        <Link to="/login">Sign In</Link>
      </div>
    );
  }

  const organizationId = user.organizationId;

  // Filter state for API query (UI types)
  // Note: priority and search are accepted from UI for future compatibility,
  // but the current DisputeListQuery API does not support these fields.
  // These will be ignored until backend API is extended to support them.
  const [filters, setFilters] = useState<{
    status?: UiDisputeStatus;
    priority?: DisputePriority; // UI-only: API does not support priority filtering yet
    category?: DisputeCategory;
    search?: string; // UI-only: API does not support text search yet
    page: number;
    pageSize: number;
  }>({ page: 1, pageSize: 10 });

  // Map UI filters to API query parameters
  // Note: priority and search are not passed to API as DisputeListQuery does not support them.
  // When backend adds support, update apiQuery to include these fields.
  const apiQuery = {
    status: filters.status ? mapUiStatusToApiStatus(filters.status) : undefined,
    type: filters.category ? mapCategoryToType(filters.category) : undefined,
    limit: filters.pageSize,
    page: filters.page,
  };

  // Use the disputes API hook
  const { data, isLoading, error } = useDisputes(organizationId, apiQuery);

  // Show error toast if query fails (use useEffect to prevent toast spam)
  useEffect(() => {
    if (error) {
      showToast({
        type: 'error',
        title: 'Failed to load disputes',
        message: error instanceof Error ? error.message : 'An unexpected error occurred',
      });
    }
  }, [error, showToast]);

  // Transform API response to match component interface
  const disputes: DisputeSummary[] = (data?.data ?? []).map(transformDisputeToSummary);
  const total = data?.total ?? 0;

  const handleNavigateToCreate = () => {
    navigate('/disputes/new');
  };
  const handleNavigateToView = (id: string) => {
    navigate(`/disputes/${id}`);
  };
  const handleNavigateToManage = (id: string) => {
    navigate(`/disputes/${id}`);
  };
  const handleFilterChange = (newFilters: {
    status?: UiDisputeStatus;
    priority?: DisputePriority;
    category?: DisputeCategory;
    search?: string;
    page: number;
    pageSize: number;
  }) => {
    setFilters(newFilters);
  };

  return (
    <DisputesPage
      disputes={disputes}
      total={total}
      isLoading={isLoading}
      onNavigateToCreate={handleNavigateToCreate}
      onNavigateToView={handleNavigateToView}
      onNavigateToManage={handleNavigateToManage}
      onFilterChange={handleFilterChange}
    />
  );
}

/**
 * Route wrapper for file dispute page (Epic 77, Story 80.2).
 *
 * Uses useCreateDispute mutation from @ppt/api-client for creating disputes.
 * Implements real API integration with toast notifications.
 * Transforms UI form data to API CreateDisputeRequest format.
 */
function FileDisputePageRoute() {
  const navigate = useNavigate();
  const { user } = useAuth();
  const { showToast } = useToast();

  // Require organization context for filing disputes
  if (!user?.organizationId) {
    return (
      <div className="error-page">
        <h1>Authentication Required</h1>
        <p>Unable to file dispute: missing organization context. Please sign in again.</p>
        <Link to="/login">Sign In</Link>
      </div>
    );
  }

  const organizationId = user.organizationId;

  const createDispute = useCreateDispute(organizationId);

  // Handle form submission - transform UI data to API format
  const handleSubmit = async (formData: {
    category: DisputeCategory;
    title: string;
    description: string;
    desiredResolution?: string;
    respondentIds: string[];
    buildingId?: string;
    unitId?: string;
  }) => {
    // Validate unitId is provided before submission
    if (!formData.unitId) {
      showToast({
        type: 'error',
        title: 'Unit required',
        message: 'Please select a unit for this dispute.',
      });
      return;
    }

    // Warn if multiple respondents selected (API only supports one)
    if (formData.respondentIds.length > 1) {
      showToast({
        type: 'warning',
        title: 'Multiple respondents',
        message: `Only the first respondent will be recorded. ${formData.respondentIds.length - 1} additional respondent(s) will not be included.`,
      });
    }

    try {
      // Transform UI form data to API CreateDisputeRequest
      const apiRequest = {
        type: mapCategoryToType(formData.category),
        subject: formData.title,
        // Combine description and desired resolution with clear delimiters
        description: formData.desiredResolution
          ? `Description:\n${formData.description}\n\n---\nDesired Resolution:\n${formData.desiredResolution}`
          : formData.description,
        unitId: formData.unitId,
        respondentId: formData.respondentIds[0],
      };

      await createDispute.mutateAsync(apiRequest);
      showToast({
        type: 'success',
        title: 'Dispute filed successfully',
        message: 'Your dispute has been submitted and will be reviewed.',
      });
      navigate('/disputes');
    } catch (error) {
      showToast({
        type: 'error',
        title: 'Failed to file dispute',
        message: error instanceof Error ? error.message : 'An unexpected error occurred',
      });
    }
  };

  const handleCancel = () => {
    navigate('/disputes');
  };

  return (
    <FileDisputePage
      onSubmit={handleSubmit}
      onCancel={handleCancel}
      isSubmitting={createDispute.isPending}
    />
  );
}

/**
 * Route wrapper for dispute detail page (Epic 77, Story 80.1).
 *
 * Uses useDispute hook from @ppt/api-client for data fetching.
 * Implements real API integration with loading/error states.
 * Maps API DisputeWithDetails to UI-friendly display format.
 */
function DisputeDetailRoute() {
  const { disputeId } = useParams<{ disputeId: string }>();
  const { showToast } = useToast();

  const { data: dispute, isLoading, error, refetch } = useDispute(disputeId ?? '');

  // Use useEffect for error toast to prevent spam on re-renders
  useEffect(() => {
    if (error) {
      showToast({
        type: 'error',
        title: 'Failed to load dispute',
        message: error instanceof Error ? error.message : 'An unexpected error occurred',
      });
    }
  }, [error, showToast]);

  if (!disputeId) {
    return (
      <div className="error-page">
        <h1>Dispute not found</h1>
        <p>We couldn't find the dispute you're looking for.</p>
        <Link to="/disputes">Back to disputes</Link>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="loading-page">
        <h1>Loading dispute...</h1>
        <p>Please wait while we fetch the dispute details.</p>
      </div>
    );
  }

  // Add retry button for error states
  if (error) {
    return (
      <div className="error-page">
        <h1>Error loading dispute</h1>
        <p>We encountered an error while loading the dispute details.</p>
        <div className="error-actions">
          <button
            type="button"
            onClick={() => refetch()}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 mr-2"
          >
            Try Again
          </button>
          <Link to="/disputes" className="px-4 py-2 border border-gray-300 rounded-lg">
            Back to disputes
          </Link>
        </div>
      </div>
    );
  }

  if (!dispute) {
    return (
      <div className="error-page">
        <h1>Dispute not found</h1>
        <p>The dispute you're looking for does not exist.</p>
        <Link to="/disputes">Back to disputes</Link>
      </div>
    );
  }

  // Map API type to UI category for display
  const category = mapTypeToCategory(dispute.type);
  const status = mapApiStatusToUiStatus(dispute.status);

  return (
    <div className="dispute-detail-page">
      <h1>Dispute: {dispute.subject}</h1>
      <div className="dispute-meta">
        <span className={`status status--${status}`}>{status.split('_').join(' ')}</span>
        <span className="type">{category.split('_').join(' ')}</span>
      </div>
      <div className="dispute-description">
        <h2>Description</h2>
        <p>{dispute.description}</p>
      </div>
      <div className="dispute-timeline">
        <h2>Timeline</h2>
        {dispute.timeline && dispute.timeline.length > 0 ? (
          <ul>
            {dispute.timeline.map((event) => (
              <li key={event.id}>
                <strong>{event.eventType.split('_').join(' ')}</strong>: {event.description}
                <span className="text-gray-500 ml-2">
                  ({new Date(event.createdAt).toLocaleDateString()})
                </span>
              </li>
            ))}
          </ul>
        ) : (
          <p>
            <em>No timeline events yet.</em>
          </p>
        )}
      </div>
      <Link to="/disputes">Back to disputes</Link>
    </div>
  );
}

function Home() {
  return (
    <div>
      <h1>Property Management System</h1>
      <p>Welcome to the Property Management System.</p>
    </div>
  );
}

// Login component removed - now using LoginPage from ./pages/LoginPage

export default App;
