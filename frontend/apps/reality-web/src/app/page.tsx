/**
 * Homepage
 *
 * Reality Portal homepage with hero search and featured listings (Epic 44, Story 44.1).
 */

'use client';

import { CategoryCards, FeaturedListings, HeroSearch } from '@/components/home';
import { Footer, Header } from '@/components/ui';

export default function HomePage() {
  return (
    <div className="page-container">
      <Header />
      <main>
        <HeroSearch />
        <CategoryCards />
        <FeaturedListings />
      </main>
      <Footer />

      <style jsx>{`
        .page-container {
          min-height: 100vh;
          display: flex;
          flex-direction: column;
        }

        main {
          flex: 1;
        }
      `}</style>
    </div>
  );
}
