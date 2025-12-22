import { AuthButton } from '@/components/auth-button';

const styles = {
  container: {
    minHeight: '100vh',
  },
  header: {
    backgroundColor: '#fff',
    borderBottom: '1px solid #e5e7eb',
  },
  headerInner: {
    maxWidth: '1200px',
    margin: '0 auto',
    padding: '16px',
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  logo: {
    fontSize: '1.5rem',
    fontWeight: 'bold',
    color: '#2563eb',
    textDecoration: 'none',
  },
  main: {
    maxWidth: '1200px',
    margin: '0 auto',
    padding: '32px 16px',
  },
  title: {
    fontSize: '2.5rem',
    fontWeight: 'bold',
    color: '#111827',
    marginBottom: '16px',
  },
  subtitle: {
    fontSize: '1.125rem',
    color: '#4b5563',
    marginBottom: '32px',
  },
} as const;

export default function HomePage() {
  return (
    <div style={styles.container}>
      {/* Header with auth */}
      <header style={styles.header}>
        <div style={styles.headerInner}>
          <a href="/" style={styles.logo}>
            Reality Portal
          </a>
          <AuthButton />
        </div>
      </header>

      {/* Main content */}
      <main style={styles.main}>
        <h1 style={styles.title}>Find your perfect property</h1>
        <p style={styles.subtitle}>
          Search thousands of listings across Slovakia, Czech Republic, and beyond.
        </p>
        {/* TODO: Add listing search component */}
      </main>
    </div>
  );
}
