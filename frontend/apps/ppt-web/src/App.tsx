import { AccessibilityProvider, SkipNavigation } from '@ppt/ui-kit';
import type { ReactNode } from 'react';
import { BrowserRouter, Link, Route, Routes, useParams } from 'react-router-dom';
import { ConnectionStatus, OfflineIndicator, ToastProvider } from './components';
import { AuthProvider, WebSocketProvider, useAuth } from './contexts';
import { DisputesPage, FileDisputePage } from './features/disputes';
import { DocumentDetailPage, DocumentUploadPage, DocumentsPage } from './features/documents';
import { EmergencyContactDirectoryPage } from './features/emergency';
import { ArticleDetailPage, NewsListPage } from './features/news';
import { PrivacySettingsPage } from './features/privacy';
import { AccessibilitySettingsPage } from './features/settings';
import { LoginPage } from './pages/LoginPage';

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
                <nav className="app-nav" aria-label="Main navigation">
                  <Link to="/">Home</Link>
                  <Link to="/documents">Documents</Link>
                  <Link to="/news">News</Link>
                  <Link to="/emergency">Emergency Contacts</Link>
                  <Link to="/disputes">Disputes</Link>
                  <Link to="/settings/accessibility">Accessibility</Link>
                  <Link to="/settings/privacy">Privacy</Link>
                  <ConnectionStatus />
                </nav>
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
  // In a real app, organizationId would come from auth context
  return <DocumentsPage organizationId="default-org" />;
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
 * Route wrapper for disputes page (Epic 77).
 *
 * Note: This component uses empty mock data as intentional placeholders for demonstration.
 * In production, data fetching should be implemented using React Query hooks from @ppt/api-client.
 * The DisputesPage component is a presentational component that receives data via props.
 */
function DisputesPageRoute() {
  // Demo mode: Empty data for UI demonstration purposes
  // Production implementation should use useDisputes() hook from api-client
  const handleNavigateToCreate = () => {
    window.location.href = '/disputes/new';
  };
  const handleNavigateToView = (id: string) => {
    window.location.href = `/disputes/${id}`;
  };
  const handleNavigateToManage = (id: string) => {
    window.location.href = `/disputes/${id}`;
  };
  const handleFilterChange = () => {
    // Filter handling will be implemented with React Query when data fetching is added
  };

  return (
    <DisputesPage
      disputes={[]}
      total={0}
      isLoading={false}
      onNavigateToCreate={handleNavigateToCreate}
      onNavigateToView={handleNavigateToView}
      onNavigateToManage={handleNavigateToManage}
      onFilterChange={handleFilterChange}
    />
  );
}

/**
 * Route wrapper for file dispute page (Epic 77).
 *
 * Note: Submission handlers are placeholders for demonstration purposes.
 * Production implementation should use useCreateDispute() mutation from api-client.
 */
function FileDisputePageRoute() {
  // Demo mode: Placeholder handlers for UI demonstration
  // Production implementation should use useCreateDispute() mutation from api-client
  const handleSubmit = () => {
    // Dispute submission will be implemented with React Query mutation
    window.location.href = '/disputes';
  };
  const handleCancel = () => {
    window.location.href = '/disputes';
  };

  return <FileDisputePage onSubmit={handleSubmit} onCancel={handleCancel} />;
}

/**
 * Route wrapper for dispute detail page (Epic 77).
 *
 * Note: This is a placeholder component. Full implementation requires:
 * 1. useDispute(disputeId) hook from api-client for data fetching
 * 2. DisputeDetailPage component with proper props interface
 * 3. Loading/error states handling
 */
function DisputeDetailRoute() {
  const { disputeId } = useParams<{ disputeId: string }>();
  if (!disputeId) return <div>Dispute not found</div>;
  // Placeholder: Data fetching will be implemented with useDispute() hook from api-client
  return (
    <div>
      <h1>Dispute Details</h1>
      <p>Loading dispute {disputeId}...</p>
      <p>
        <em>Note: Container component with data fetching needed for full implementation.</em>
      </p>
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
