/** @type {import('next').NextConfig} */
const nextConfig = {
  // Enable React strict mode
  reactStrictMode: true,

  // i18n configuration for multi-region deployment
  // Handled by next-intl middleware

  // Image optimization
  images: {
    domains: ['api.reality-portal.sk', 'api.reality-portal.cz', 'api.reality-portal.eu'],
  },

  // Environment variables
  env: {
    REGION: process.env.REGION || 'local',
  },
};

module.exports = nextConfig;
