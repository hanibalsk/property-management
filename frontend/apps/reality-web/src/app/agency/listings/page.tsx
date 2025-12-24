/**
 * Agency Listings Page
 *
 * Manage agency listings (Epic 45, Story 45.3).
 */

import { AgencyListings } from '@/components/agency';
import { Footer, Header } from '@/components/ui';

export default function AgencyListingsPage() {
  return (
    <div className="page">
      <Header />
      <main>
        <AgencyListings />
      </main>
      <Footer />
    </div>
  );
}
