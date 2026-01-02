import type { UserFlow } from '../types';

/**
 * All user flows across applications
 */
export const allFlows: UserFlow[] = [
  // Authentication Flows
  {
    id: 'flow-login-basic',
    name: 'Basic Login Flow',
    apps: ['ppt-web', 'api-server'],
    description: 'User logs in with email and password',
    category: 'authentication',
    requiredRole: 'anonymous',
    useCases: ['UC-14.1'],
    steps: [
      {
        order: 1,
        name: 'Navigate to login page',
        route: 'ppt-login',
        action: { type: 'navigate', target: '/login' },
        assertions: [
          { type: 'url', target: 'path', expected: '/login' },
          { type: 'visible', target: '[data-testid="login-form"]', expected: true },
        ],
      },
      {
        order: 2,
        name: 'Enter email',
        action: { type: 'fill', target: '[name="email"]', value: '{{user.email}}' },
      },
      {
        order: 3,
        name: 'Enter password',
        action: { type: 'fill', target: '[name="password"]', value: '{{user.password}}' },
      },
      {
        order: 4,
        name: 'Submit login form',
        apiCall: 'auth_login',
        action: { type: 'submit', target: '[data-testid="login-form"]' },
        assertions: [{ type: 'api_response', target: 'status', expected: 200 }],
      },
      {
        order: 5,
        name: 'Redirect to dashboard',
        action: { type: 'wait', waitMs: 500 },
        assertions: [
          { type: 'url', target: 'path', expected: '/' },
          { type: 'visible', target: '[data-testid="dashboard"]', expected: true },
        ],
      },
    ],
    expectedOutcome: { authenticated: true, redirectedTo: '/' },
    tags: ['auth', 'critical', 'smoke'],
  },
  {
    id: 'flow-logout',
    name: 'Logout Flow',
    apps: ['ppt-web', 'api-server'],
    description: 'User logs out of the system',
    category: 'authentication',
    requiredRole: 'tenant',
    prerequisites: ['flow-login-basic'],
    steps: [
      {
        order: 1,
        name: 'Click logout button',
        action: { type: 'click', target: '[data-testid="logout-btn"]' },
      },
      {
        order: 2,
        name: 'Confirm logout',
        apiCall: 'auth_logout',
        action: { type: 'click', target: '[data-testid="confirm-logout"]' },
        assertions: [{ type: 'api_response', target: 'status', expected: 200 }],
      },
      {
        order: 3,
        name: 'Redirect to login',
        assertions: [{ type: 'url', target: 'path', expected: '/login' }],
      },
    ],
    expectedOutcome: { authenticated: false, redirectedTo: '/login' },
    tags: ['auth', 'smoke'],
  },

  // Document Flows
  {
    id: 'flow-document-upload',
    name: 'Document Upload Flow',
    apps: ['ppt-web', 'api-server'],
    description: 'User uploads a new document',
    category: 'documents',
    requiredRole: 'owner',
    prerequisites: ['flow-login-basic'],
    useCases: ['UC-08.1'],
    steps: [
      {
        order: 1,
        name: 'Navigate to documents',
        route: 'ppt-documents',
        action: { type: 'navigate', target: '/documents' },
        assertions: [{ type: 'url', target: 'path', expected: '/documents' }],
      },
      {
        order: 2,
        name: 'Click upload button',
        action: { type: 'click', target: '[data-testid="upload-btn"]' },
        assertions: [{ type: 'url', target: 'path', expected: '/documents/upload' }],
      },
      {
        order: 3,
        name: 'Fill upload form',
        action: {
          type: 'fill',
          value: {
            file: '{{document.file}}',
            title: '{{document.title}}',
            category: '{{document.category}}',
          },
        },
      },
      {
        order: 4,
        name: 'Submit upload',
        apiCall: 'documents_upload',
        action: { type: 'submit', target: '[data-testid="upload-form"]' },
        assertions: [{ type: 'api_response', target: 'status', expected: 201 }],
      },
      {
        order: 5,
        name: 'Verify success',
        assertions: [
          { type: 'visible', target: '[data-testid="toast-success"]', expected: true },
        ],
      },
    ],
    expectedOutcome: { documentUploaded: true },
    tags: ['documents', 'upload', 'smoke'],
  },
  {
    id: 'flow-document-view',
    name: 'Document View Flow',
    apps: ['ppt-web', 'api-server'],
    description: 'User views document details',
    category: 'documents',
    requiredRole: 'tenant',
    prerequisites: ['flow-login-basic'],
    steps: [
      {
        order: 1,
        name: 'Navigate to documents',
        route: 'ppt-documents',
        action: { type: 'navigate', target: '/documents' },
      },
      {
        order: 2,
        name: 'Click on document',
        apiCall: 'documents_get',
        action: { type: 'click', target: '[data-testid="document-item"]:first-child' },
        assertions: [{ type: 'api_response', target: 'status', expected: 200 }],
      },
      {
        order: 3,
        name: 'View document detail',
        assertions: [
          { type: 'visible', target: '[data-testid="document-detail"]', expected: true },
        ],
      },
    ],
    expectedOutcome: { documentViewed: true },
    tags: ['documents', 'view'],
  },

  // Fault Management Flows
  {
    id: 'flow-report-fault',
    name: 'Report Fault Flow',
    apps: ['mobile', 'api-server'],
    description: 'User reports a new fault',
    category: 'fault_management',
    requiredRole: 'tenant',
    prerequisites: ['flow-login-basic'],
    useCases: ['UC-03.1'],
    steps: [
      {
        order: 1,
        name: 'Navigate to faults',
        route: 'mobile-faults-list',
        action: { type: 'navigate', target: 'FaultsList' },
      },
      {
        order: 2,
        name: 'Tap report button',
        action: { type: 'click', target: '[data-testid="report-fault-btn"]' },
      },
      {
        order: 3,
        name: 'Fill fault form',
        action: {
          type: 'fill',
          value: {
            title: '{{fault.title}}',
            description: '{{fault.description}}',
            category: '{{fault.category}}',
            priority: '{{fault.priority}}',
          },
        },
      },
      {
        order: 4,
        name: 'Submit fault',
        apiCall: 'faults_create',
        action: { type: 'submit', target: '[data-testid="fault-form"]' },
        assertions: [{ type: 'api_response', target: 'status', expected: 201 }],
      },
    ],
    expectedOutcome: { faultCreated: true },
    tags: ['faults', 'create', 'smoke'],
  },

  // Dispute Resolution Flows
  {
    id: 'flow-file-dispute',
    name: 'File Dispute Flow',
    apps: ['ppt-web', 'api-server'],
    description: 'User files a new dispute',
    category: 'disputes',
    requiredRole: 'tenant',
    prerequisites: ['flow-login-basic'],
    steps: [
      {
        order: 1,
        name: 'Navigate to disputes',
        route: 'ppt-disputes',
        action: { type: 'navigate', target: '/disputes' },
      },
      {
        order: 2,
        name: 'Click file dispute',
        action: { type: 'click', target: '[data-testid="file-dispute-btn"]' },
        assertions: [{ type: 'url', target: 'path', expected: '/disputes/new' }],
      },
      {
        order: 3,
        name: 'Fill dispute form',
        action: {
          type: 'fill',
          value: {
            subject: '{{dispute.subject}}',
            description: '{{dispute.description}}',
            category: '{{dispute.category}}',
          },
        },
      },
      {
        order: 4,
        name: 'Submit dispute',
        apiCall: 'disputes_create',
        action: { type: 'submit', target: '[data-testid="dispute-form"]' },
        assertions: [{ type: 'api_response', target: 'status', expected: 201 }],
      },
    ],
    expectedOutcome: { disputeFiled: true },
    tags: ['disputes', 'create', 'smoke'],
  },

  // Voting Flows
  {
    id: 'flow-cast-vote',
    name: 'Cast Vote Flow',
    apps: ['mobile', 'api-server'],
    description: 'User casts a vote on a poll',
    category: 'voting',
    requiredRole: 'owner',
    prerequisites: ['flow-login-basic'],
    useCases: ['UC-04.1'],
    steps: [
      {
        order: 1,
        name: 'Navigate to voting',
        route: 'mobile-voting',
        action: { type: 'navigate', target: 'Voting' },
      },
      {
        order: 2,
        name: 'Select active vote',
        apiCall: 'voting_get',
        action: { type: 'click', target: '[data-testid="vote-item"]:first-child' },
      },
      {
        order: 3,
        name: 'Select option',
        action: { type: 'click', target: '[data-testid="vote-option-1"]' },
      },
      {
        order: 4,
        name: 'Confirm vote',
        apiCall: 'voting_cast',
        action: { type: 'click', target: '[data-testid="confirm-vote-btn"]' },
        assertions: [{ type: 'api_response', target: 'status', expected: 200 }],
      },
    ],
    expectedOutcome: { voteCast: true },
    tags: ['voting', 'cast', 'smoke'],
  },

  // Reality Portal Flows
  {
    id: 'flow-search-listings',
    name: 'Search Listings Flow',
    apps: ['reality-web', 'reality-server'],
    description: 'User searches for property listings',
    category: 'listings',
    requiredRole: 'anonymous',
    steps: [
      {
        order: 1,
        name: 'Navigate to home',
        route: 'reality-home',
        action: { type: 'navigate', target: '/' },
      },
      {
        order: 2,
        name: 'Enter search criteria',
        action: {
          type: 'fill',
          value: {
            type: 'sale',
            city: '{{search.city}}',
            price_max: '{{search.price_max}}',
          },
        },
      },
      {
        order: 3,
        name: 'Submit search',
        apiCall: 'listings_search',
        action: { type: 'submit', target: '[data-testid="search-form"]' },
        assertions: [{ type: 'api_response', target: 'status', expected: 200 }],
      },
      {
        order: 4,
        name: 'View results',
        assertions: [
          { type: 'url', target: 'path', expected: '/listings' },
          { type: 'visible', target: '[data-testid="listing-grid"]', expected: true },
        ],
      },
    ],
    expectedOutcome: { searchPerformed: true, resultsDisplayed: true },
    tags: ['listings', 'search', 'smoke'],
  },
  {
    id: 'flow-add-favorite',
    name: 'Add to Favorites Flow',
    apps: ['reality-web', 'reality-server'],
    description: 'User adds a listing to favorites',
    category: 'listings',
    requiredRole: 'portal_user',
    prerequisites: ['flow-search-listings'],
    steps: [
      {
        order: 1,
        name: 'View listing detail',
        route: 'reality-listing-detail',
        apiCall: 'listings_get',
        action: { type: 'click', target: '[data-testid="listing-card"]:first-child' },
      },
      {
        order: 2,
        name: 'Click favorite button',
        apiCall: 'favorites_add',
        action: { type: 'click', target: '[data-testid="favorite-btn"]' },
        assertions: [{ type: 'api_response', target: 'status', expected: 201 }],
      },
      {
        order: 3,
        name: 'Verify favorite added',
        assertions: [
          { type: 'visible', target: '[data-testid="favorite-btn"][data-favorited="true"]', expected: true },
        ],
      },
    ],
    expectedOutcome: { favoriteAdded: true },
    tags: ['favorites', 'add'],
  },
];
