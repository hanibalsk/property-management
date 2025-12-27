/**
 * Developer Portal Page (Epic 69)
 *
 * Main page for the developer portal with tabs for API keys, webhooks,
 * documentation, rate limits, and SDKs.
 */

import { useState } from 'react';
import { DeveloperDashboard } from '../components/DeveloperDashboard';
import { ApiKeysList } from '../components/ApiKeysList';
import { ApiKeyCreateDialog } from '../components/ApiKeyCreateDialog';
import { ApiKeySecretDialog } from '../components/ApiKeySecretDialog';
import { WebhooksList } from '../components/WebhooksList';
import { WebhookCreateDialog } from '../components/WebhookCreateDialog';
import { WebhookSecretDialog } from '../components/WebhookSecretDialog';
import { ApiDocumentation, ApiChangelogList } from '../components/ApiDocumentation';
import { RateLimitStatus, RateLimitTierComparison } from '../components/RateLimitStatus';
import { SdkDownloadList } from '../components/SdkDownloadList';
import type {
  DeveloperAccount,
  DeveloperUsageSummary,
  RateLimitStatus as RateLimitStatusType,
  RateLimitConfig,
  ApiKey,
  CreateApiKey,
  CreateApiKeyResponse,
  WebhookSubscription,
  CreateWebhookSubscription,
  CreateWebhookResponse,
  ApiEndpointDoc,
  ApiChangelog,
  SdkLanguageInfo,
} from '../types';

interface DeveloperPortalPageProps {
  organizationId: string;
}

type TabValue = 'dashboard' | 'keys' | 'webhooks' | 'docs' | 'rate-limits' | 'sdks';

const tabs: { value: TabValue; label: string }[] = [
  { value: 'dashboard', label: 'Dashboard' },
  { value: 'keys', label: 'API Keys' },
  { value: 'webhooks', label: 'Webhooks' },
  { value: 'docs', label: 'Documentation' },
  { value: 'rate-limits', label: 'Rate Limits' },
  { value: 'sdks', label: 'SDKs' },
];

export function DeveloperPortalPage({ organizationId }: DeveloperPortalPageProps) {
  const [activeTab, setActiveTab] = useState<TabValue>('dashboard');

  // Dialog states
  const [showCreateKeyDialog, setShowCreateKeyDialog] = useState(false);
  const [showKeySecretDialog, setShowKeySecretDialog] = useState(false);
  const [createdKey, setCreatedKey] = useState<CreateApiKeyResponse | null>(null);
  const [showCreateWebhookDialog, setShowCreateWebhookDialog] = useState(false);
  const [showWebhookSecretDialog, setShowWebhookSecretDialog] = useState(false);
  const [createdWebhook, setCreatedWebhook] = useState<CreateWebhookResponse | null>(null);

  // Mock data - in production, these would come from API calls
  const account: DeveloperAccount = {
    id: 'dev-123',
    userId: 'user-456',
    organizationId,
    companyName: 'Acme Corp',
    website: 'https://acme.example.com',
    description: 'Property management integration',
    contactEmail: 'developer@acme.example.com',
    contactName: 'John Developer',
    tier: 'professional',
    isVerified: true,
    isActive: true,
    createdAt: '2024-01-15T10:00:00Z',
    updatedAt: '2024-12-01T15:30:00Z',
    verifiedAt: '2024-01-20T09:00:00Z',
  };

  const usage: DeveloperUsageSummary = {
    developerAccountId: account.id,
    companyName: account.companyName,
    tier: account.tier,
    apiKeysCount: 3,
    webhooksCount: 2,
    totalRequestsToday: 4520,
    totalRequestsMonth: 125000,
    rateLimitHits: 12,
    lastApiCall: '2024-12-27T14:30:00Z',
  };

  const rateLimitStatus: RateLimitStatusType = {
    tier: 'professional',
    requestsPerMinute: {
      limit: 300,
      remaining: 285,
      resetAt: new Date(Date.now() + 45000).toISOString(),
    },
    requestsPerHour: {
      limit: 20000,
      remaining: 19200,
      resetAt: new Date(Date.now() + 2400000).toISOString(),
    },
    requestsPerDay: {
      limit: 200000,
      remaining: 175000,
      resetAt: new Date(Date.now() + 43200000).toISOString(),
    },
  };

  const apiKeys: ApiKey[] = [
    {
      id: 'key-1',
      developerAccountId: account.id,
      name: 'Production API Key',
      keyPrefix: 'ppt_prod_abc12',
      scopes: ['read', 'write', 'buildings:read', 'faults:write'],
      rateLimitPerMinute: 300,
      lastUsedAt: '2024-12-27T14:30:00Z',
      totalRequests: 125000,
      status: 'active',
      createdAt: '2024-06-01T10:00:00Z',
    },
    {
      id: 'key-2',
      developerAccountId: account.id,
      name: 'Development Key',
      keyPrefix: 'ppt_dev_xyz89',
      scopes: ['read'],
      lastUsedAt: '2024-12-26T10:00:00Z',
      totalRequests: 8500,
      status: 'active',
      createdAt: '2024-08-15T14:00:00Z',
    },
  ];

  const webhooks: WebhookSubscription[] = [
    {
      id: 'wh-1',
      developerAccountId: account.id,
      name: 'Fault Notifications',
      endpointUrl: 'https://api.acme.example.com/webhooks/faults',
      eventTypes: ['fault.created', 'fault.updated', 'fault.resolved'],
      isActive: true,
      retryCount: 3,
      timeoutSeconds: 30,
      totalDeliveries: 1250,
      successfulDeliveries: 1240,
      failedDeliveries: 10,
      lastTriggeredAt: '2024-12-27T13:45:00Z',
      lastSuccessAt: '2024-12-27T13:45:00Z',
      createdAt: '2024-06-15T10:00:00Z',
      updatedAt: '2024-12-01T09:00:00Z',
    },
  ];

  const sdkLanguages: SdkLanguageInfo[] = [
    {
      language: 'typescript',
      displayName: 'TypeScript / JavaScript',
      packageManager: 'npm',
      latestVersion: '1.2.0',
      installationCommand: 'npm install @ppt/api-client',
      documentationUrl: 'https://docs.ppt.example.com/sdks/typescript',
    },
    {
      language: 'python',
      displayName: 'Python',
      packageManager: 'pip',
      latestVersion: '1.2.0',
      installationCommand: 'pip install ppt-api-client',
      documentationUrl: 'https://docs.ppt.example.com/sdks/python',
    },
    {
      language: 'go',
      displayName: 'Go',
      packageManager: 'go modules',
      latestVersion: '1.2.0',
      installationCommand: 'go get github.com/ppt/api-client-go',
      documentationUrl: 'https://docs.ppt.example.com/sdks/go',
    },
  ];

  const rateLimitTiers: RateLimitConfig[] = [
    {
      id: 'tier-1',
      tier: 'free',
      requestsPerMinute: 60,
      requestsPerHour: 1000,
      requestsPerDay: 10000,
      burstLimit: 10,
      description: 'Free tier for development',
      isActive: true,
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: '2024-01-01T00:00:00Z',
    },
    {
      id: 'tier-2',
      tier: 'basic',
      requestsPerMinute: 120,
      requestsPerHour: 5000,
      requestsPerDay: 50000,
      burstLimit: 20,
      description: 'For small businesses',
      isActive: true,
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: '2024-01-01T00:00:00Z',
    },
    {
      id: 'tier-3',
      tier: 'professional',
      requestsPerMinute: 300,
      requestsPerHour: 20000,
      requestsPerDay: 200000,
      burstLimit: 50,
      description: 'For growing businesses',
      isActive: true,
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: '2024-01-01T00:00:00Z',
    },
    {
      id: 'tier-4',
      tier: 'enterprise',
      requestsPerMinute: 1000,
      requestsPerHour: 100000,
      requestsPerDay: 1000000,
      burstLimit: 100,
      description: 'Custom enterprise limits',
      isActive: true,
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: '2024-01-01T00:00:00Z',
    },
  ];

  // Handlers
  const handleCreateApiKey = async (data: CreateApiKey) => {
    // In production, this would be an API call
    const newKey: CreateApiKeyResponse = {
      id: `key-${Date.now()}`,
      name: data.name,
      keyPrefix: `ppt_${Math.random().toString(36).substring(2, 10)}`,
      secret: `ppt_${Math.random().toString(36).substring(2, 10)}_${Math.random().toString(36).substring(2, 32)}`,
      scopes: data.scopes,
      expiresAt: data.expiresAt,
      createdAt: new Date().toISOString(),
    };
    setCreatedKey(newKey);
    setShowCreateKeyDialog(false);
    setShowKeySecretDialog(true);
  };

  const handleCreateWebhook = async (data: CreateWebhookSubscription) => {
    // In production, this would be an API call
    const newWebhook: CreateWebhookResponse = {
      id: `wh-${Date.now()}`,
      name: data.name,
      endpointUrl: data.endpointUrl,
      secret: `whsec_${Math.random().toString(36).substring(2, 32)}`,
      eventTypes: data.eventTypes,
      createdAt: new Date().toISOString(),
    };
    setCreatedWebhook(newWebhook);
    setShowCreateWebhookDialog(false);
    setShowWebhookSecretDialog(true);
  };

  return (
    <div className="space-y-6">
      {/* Tab Navigation */}
      <div className="flex gap-2 border-b overflow-x-auto">
        {tabs.map((tab) => (
          <button
            type="button"
            key={tab.value}
            onClick={() => setActiveTab(tab.value)}
            className={`px-4 py-2 text-sm font-medium border-b-2 whitespace-nowrap transition-colors ${
              activeTab === tab.value
                ? 'border-primary text-primary'
                : 'border-transparent text-muted-foreground hover:text-foreground'
            }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Tab Content */}
      <div>
        {activeTab === 'dashboard' && (
          <DeveloperDashboard
            account={account}
            usage={usage}
            rateLimitStatus={rateLimitStatus}
          />
        )}

        {activeTab === 'keys' && (
          <ApiKeysList
            apiKeys={apiKeys}
            onCreateKey={() => setShowCreateKeyDialog(true)}
            onRotateKey={(id) => console.log('Rotate key:', id)}
            onRevokeKey={(id) => console.log('Revoke key:', id)}
            onViewUsage={(id) => console.log('View usage:', id)}
          />
        )}

        {activeTab === 'webhooks' && (
          <WebhooksList
            webhooks={webhooks}
            onCreateWebhook={() => setShowCreateWebhookDialog(true)}
            onEditWebhook={(id) => console.log('Edit webhook:', id)}
            onDeleteWebhook={(id) => console.log('Delete webhook:', id)}
            onTestWebhook={(id) => console.log('Test webhook:', id)}
            onViewDeliveries={(id) => console.log('View deliveries:', id)}
            onRotateSecret={(id) => console.log('Rotate secret:', id)}
            onToggleActive={(id, active) => console.log('Toggle active:', id, active)}
          />
        )}

        {activeTab === 'docs' && (
          <div className="space-y-8">
            <ApiDocumentation
              endpoints={[]}
              changelog={[]}
              onTestEndpoint={(endpoint) => console.log('Test endpoint:', endpoint)}
            />
            <ApiChangelogList changelog={[]} />
          </div>
        )}

        {activeTab === 'rate-limits' && (
          <div className="space-y-8">
            <RateLimitStatus status={rateLimitStatus} />
            <RateLimitTierComparison
              tiers={rateLimitTiers}
              currentTier={account.tier}
              onSelectTier={(tier) => console.log('Select tier:', tier)}
            />
          </div>
        )}

        {activeTab === 'sdks' && (
          <SdkDownloadList
            languages={sdkLanguages}
            onDownload={(lang) => console.log('Download SDK:', lang)}
            onViewVersions={(lang) => console.log('View versions:', lang)}
          />
        )}
      </div>

      {/* Dialogs */}
      <ApiKeyCreateDialog
        isOpen={showCreateKeyDialog}
        onClose={() => setShowCreateKeyDialog(false)}
        onSubmit={handleCreateApiKey}
      />

      <ApiKeySecretDialog
        isOpen={showKeySecretDialog}
        onClose={() => {
          setShowKeySecretDialog(false);
          setCreatedKey(null);
        }}
        apiKey={createdKey}
      />

      <WebhookCreateDialog
        isOpen={showCreateWebhookDialog}
        onClose={() => setShowCreateWebhookDialog(false)}
        onSubmit={handleCreateWebhook}
      />

      <WebhookSecretDialog
        isOpen={showWebhookSecretDialog}
        onClose={() => {
          setShowWebhookSecretDialog(false);
          setCreatedWebhook(null);
        }}
        webhook={createdWebhook}
      />
    </div>
  );
}
