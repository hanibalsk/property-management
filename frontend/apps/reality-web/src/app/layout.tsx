import { AuthProvider } from '@/lib/auth-context';
import { ComparisonProvider } from '@/lib/comparison-context';
import { QueryProvider } from '@/lib/query-provider';
import type { Metadata } from 'next';
import './globals.css';

import { ComparisonTray } from '../components/comparison';

export const metadata: Metadata = {
  title: 'Reality Portal - Find Your Perfect Property',
  description:
    'Search thousands of property listings across Slovakia, Czech Republic, and beyond. Find apartments, houses, and commercial properties for sale or rent.',
  keywords:
    'real estate, property, apartments, houses, for sale, for rent, Slovakia, Czech Republic',
  openGraph: {
    title: 'Reality Portal - Find Your Perfect Property',
    description: 'Search thousands of property listings for sale or rent.',
    type: 'website',
    locale: 'en_US',
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <QueryProvider>
          <AuthProvider>
            <ComparisonProvider>
              {children}
              <ComparisonTray />
            </ComparisonProvider>
          </AuthProvider>
        </QueryProvider>
      </body>
    </html>
  );
}
