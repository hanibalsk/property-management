/**
 * Agency Branding Page
 *
 * Customize agency branding (Epic 45, Story 45.4).
 */

import { AgencyBranding } from '@/components/agency';
import { Footer, Header } from '@/components/ui';

export default function BrandingPage() {
  return (
    <div className="page">
      <Header />
      <main>
        <AgencyBranding />
      </main>
      <Footer />
    </div>
  );
}
