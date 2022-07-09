module.exports = {
    roots: [
      './test',
      './src'
    ],
    testMatch: [
      '**/__tests__/**/*.+(ts|tsx|js)',
      '**/?(*.)+(spec|test).+(ts|tsx|js)'
    ],
    transform: {
      '^.+\\.(ts|tsx)$': 'ts-jest'
    },
    testEnvironment: 'jsdom',
    // watchPlugins: ['./test/WatchFilesWriteRoutes.js'],
    // watchPathIgnorePatterns: ["<rootDir>/src/routes.ts"]
  }