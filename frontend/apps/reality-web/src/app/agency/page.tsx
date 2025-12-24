/**
 * Agency Dashboard Page
 *
 * Agency owner dashboard (Epic 45, Story 45.1).
 */

import { AgencyDashboard } from '@/components/agency';
import { Footer, Header } from '@/components/ui';

export default function AgencyPage() {
  return (
    <div className="page">
      <Header />
      <main>
        <AgencyDashboard />
      </main>
      <Footer />
    </div>
  );
}
