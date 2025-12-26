import { AccessibilityProvider, SkipNavigation } from '@ppt/ui-kit';
import { BrowserRouter, Link, Route, Routes, useParams } from 'react-router-dom';
import { DocumentDetailPage, DocumentsPage, DocumentUploadPage } from './features/documents';
import { EmergencyContactDirectoryPage } from './features/emergency';
import { ArticleDetailPage, NewsListPage } from './features/news';
import { PrivacySettingsPage } from './features/privacy';
import { AccessibilitySettingsPage } from './features/settings';

function App() {
  return (
    <AccessibilityProvider>
      <BrowserRouter>
        <SkipNavigation mainContentId="main-content" />
        <div className="app">
          <nav className="app-nav" aria-label="Main navigation">
            <Link to="/">Home</Link>
            <Link to="/documents">Documents</Link>
            <Link to="/news">News</Link>
            <Link to="/emergency">Emergency Contacts</Link>
            <Link to="/settings/accessibility">Accessibility</Link>
            <Link to="/settings/privacy">Privacy</Link>
          </nav>
          <main id="main-content">
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/login" element={<Login />} />
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
            </Routes>
          </main>
        </div>
      </BrowserRouter>
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
  if (!documentId) return <div>Document not found</div>;
  return <DocumentDetailPage documentId={documentId} />;
}

/** Route wrapper for article detail page to extract params */
function ArticleDetailRoute() {
  const { articleId } = useParams<{ articleId: string }>();
  if (!articleId) return <div>Article not found</div>;
  return <ArticleDetailPage articleId={articleId} />;
}

function Home() {
  return (
    <div>
      <h1>Property Management System</h1>
      <p>Welcome to the Property Management System.</p>
    </div>
  );
}

function Login() {
  return (
    <div>
      <h1>Login</h1>
      <form>
        <input type="email" placeholder="Email" />
        <input type="password" placeholder="Password" />
        <button type="submit">Login</button>
      </form>
    </div>
  );
}

export default App;
