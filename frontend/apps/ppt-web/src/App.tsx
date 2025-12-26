import { AccessibilityProvider, SkipNavigation } from '@ppt/ui-kit';
import { BrowserRouter, Route, Routes, useParams } from 'react-router-dom';
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
            <a href="/">Home</a>
            <a href="/news">News</a>
            <a href="/emergency">Emergency Contacts</a>
            <a href="/settings/accessibility">Accessibility</a>
            <a href="/settings/privacy">Privacy</a>
          </nav>
          <main id="main-content">
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/login" element={<Login />} />
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
