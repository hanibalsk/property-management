/**
 * Property comparison page.
 *
 * Epic 51 - Story 51.2: Comparison View
 * Epic 51 - Story 51.3: Share Comparison
 */

import type { Metadata } from 'next';

import { ComparisonUrlHandler, ComparisonView } from '../../components/comparison';
import { Header } from '../../components/ui';

export const metadata: Metadata = {
  title: 'Compare Properties | Reality Portal',
  description: 'Compare properties side by side to find your perfect home.',
};

interface ComparePageProps {
  searchParams: Promise<{ ids?: string }>;
}

export default async function ComparePage({ searchParams }: ComparePageProps) {
  const params = await searchParams;
  const sharedIds = params.ids?.split(',').filter(Boolean) ?? [];

  return (
    <>
      <Header />
      <main className="compare-page">
        <div className="container">
          <div className="page-header">
            <h1>Compare Properties</h1>
            <p>See how your selected properties stack up against each other.</p>
          </div>
          {/* Handle shared URL ids parameter */}
          <ComparisonUrlHandler sharedIds={sharedIds} />
          <ComparisonView />
        </div>

        <style jsx>{`
          .compare-page {
            min-height: 100vh;
            background: #f9fafb;
          }

          .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 24px;
          }

          .page-header {
            padding: 32px 0;
            border-bottom: 1px solid #e5e7eb;
            background: white;
            margin: 0 -24px 24px;
            padding-left: 24px;
            padding-right: 24px;
          }

          h1 {
            font-size: 28px;
            font-weight: bold;
            color: #111827;
            margin: 0 0 8px;
          }

          p {
            color: #6b7280;
            margin: 0;
          }
        `}</style>
      </main>
    </>
  );
}
