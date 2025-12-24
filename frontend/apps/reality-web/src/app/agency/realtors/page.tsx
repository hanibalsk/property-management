/**
 * Realtor Management Page
 *
 * Manage agency realtors (Epic 45, Story 45.2).
 */

import { RealtorManagement } from '@/components/agency';
import { Footer, Header } from '@/components/ui';

export default function RealtorsPage() {
  return (
    <div className="page">
      <Header />
      <main>
        <RealtorManagement />
      </main>
      <Footer />
    </div>
  );
}
