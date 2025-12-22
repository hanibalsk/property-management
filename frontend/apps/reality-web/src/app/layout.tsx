import { AuthProvider } from '@/lib/auth-context';
import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'Reality Portal',
  description: 'Find your perfect property',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <AuthProvider>{children}</AuthProvider>
      </body>
    </html>
  );
}
