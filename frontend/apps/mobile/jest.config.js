/** @type {import('jest').Config} */
module.exports = {
  preset: 'jest-expo',
  setupFilesAfterEnv: ['<rootDir>/src/test/setup.ts'],
  testMatch: ['**/*.test.{ts,tsx}'],
  moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx'],
  moduleNameMapper: {
    '^@ppt/api-client$': '<rootDir>/../../packages/api-client/src',
    '^@ppt/shared$': '<rootDir>/../../packages/shared/src',
  },
  // Clear transform to use expo's babel preset
  transform: {
    '^.+\\.(js|jsx|ts|tsx)$': 'babel-jest',
  },
  // Match any file within node_modules that needs transformation
  transformIgnorePatterns: [],
};
