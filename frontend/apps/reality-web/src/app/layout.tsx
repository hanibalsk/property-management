import type { Metadata } from 'next';

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
      <body>{children}</body>
    </html>
  );
}
