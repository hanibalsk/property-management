/**
 * Developer Portal Page (Epic 69)
 *
 * Main page for the developer portal with tabs for API keys, webhooks,
 * documentation, rate limits, and SDKs.
 */

import { useState } from 'react';
import {
  useApiKeys,
  useCreateApiKey,
  useCreateWebhook,
  useDeleteWebhook,
  useDeveloperAccount,
  useDeveloperUsage,
  useDownloadSdk,
  useRateLimitStatus,
  useRateLimitTiers,
  useRevokeApiKey,
  useRotateApiKey,
  useRotateWebhookSecret,
  useSdkLanguages,
  useTestWebhook,
  useUpdateWebhook,
  useWebhooks,
} from '../api';
import { ApiChangelogList, ApiDocumentation } from '../components/ApiDocumentation';
import { ApiKeyCreateDialog } from '../components/ApiKeyCreateDialog';
import { ApiKeySecretDialog } from '../components/ApiKeySecretDialog';
import { ApiKeysList } from '../components/ApiKeysList';
import { DeveloperDashboard } from '../components/DeveloperDashboard';
import { RateLimitStatus, RateLimitTierComparison } from '../components/RateLimitStatus';
import { SdkDownloadList } from '../components/SdkDownloadList';
import { WebhookCreateDialog } from '../components/WebhookCreateDialog';
import { WebhookSecretDialog } from '../components/WebhookSecretDialog';
import { WebhooksList } from '../components/WebhooksList';
import type {
  CreateApiKey,
  CreateApiKeyResponse,
  CreateWebhookResponse,
  CreateWebhookSubscription,
  SdkLanguage,
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
  const [showSdkVersionsDialog, setShowSdkVersionsDialog] = useState(false);
  const [selectedSdkLanguage, setSelectedSdkLanguage] = useState<SdkLanguage | null>(null);

  // API Hooks
  const { data: account } = useDeveloperAccount();
  const { data: usage } = useDeveloperUsage();
  const { data: rateLimitStatus } = useRateLimitStatus();
  const { data: apiKeys = [] } = useApiKeys();
  const { data: webhooks = [] } = useWebhooks();
  const { data: sdkLanguages = [] } = useSdkLanguages();
  const { data: rateLimitTiers = [] } = useRateLimitTiers();

  // Mutations
  const createApiKeyMutation = useCreateApiKey();
  const rotateApiKeyMutation = useRotateApiKey();
  const revokeApiKeyMutation = useRevokeApiKey();
  const createWebhookMutation = useCreateWebhook();
  const updateWebhookMutation = useUpdateWebhook();
  const deleteWebhookMutation = useDeleteWebhook();
  const testWebhookMutation = useTestWebhook();
  const rotateWebhookSecretMutation = useRotateWebhookSecret();
  const downloadSdkMutation = useDownloadSdk();

  // Handlers
  const handleCreateApiKey = async (data: CreateApiKey) => {
    try {
      const newKey = await createApiKeyMutation.mutateAsync(data);
      setCreatedKey(newKey);
      setShowCreateKeyDialog(false);
      setShowKeySecretDialog(true);
    } catch (error) {
      console.error('Failed to create API key:', error);
    }
  };

  const handleRotateApiKey = async (id: string) => {
    try {
      const result = await rotateApiKeyMutation.mutateAsync(id);
      setCreatedKey(result.newKey);
      setShowKeySecretDialog(true);
    } catch (error) {
      console.error('Failed to rotate API key:', error);
    }
  };

  const handleRevokeApiKey = async (id: string) => {
    if (
      window.confirm('Are you sure you want to revoke this API key? This action cannot be undone.')
    ) {
      try {
        await revokeApiKeyMutation.mutateAsync(id);
      } catch (error) {
        console.error('Failed to revoke API key:', error);
      }
    }
  };

  const handleCreateWebhook = async (data: CreateWebhookSubscription) => {
    try {
      const newWebhook = await createWebhookMutation.mutateAsync(data);
      setCreatedWebhook(newWebhook);
      setShowCreateWebhookDialog(false);
      setShowWebhookSecretDialog(true);
    } catch (error) {
      console.error('Failed to create webhook:', error);
    }
  };

  const handleDeleteWebhook = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this webhook?')) {
      try {
        await deleteWebhookMutation.mutateAsync(id);
      } catch (error) {
        console.error('Failed to delete webhook:', error);
      }
    }
  };

  const handleTestWebhook = async (id: string) => {
    try {
      const result = await testWebhookMutation.mutateAsync({
        id,
        request: { eventType: 'fault.created', payload: { test: true } },
      });
      if (result.success) {
        alert(`Webhook test successful! Response: ${result.responseStatusCode}`);
      } else {
        alert(`Webhook test failed: ${result.errorMessage}`);
      }
    } catch (error) {
      console.error('Failed to test webhook:', error);
    }
  };

  const handleRotateWebhookSecret = async (id: string) => {
    if (
      window.confirm(
        'Are you sure you want to rotate this webhook secret? You will need to update your endpoint.'
      )
    ) {
      try {
        const result = await rotateWebhookSecretMutation.mutateAsync(id);
        setCreatedWebhook({
          id: result.webhookId,
          name: 'Webhook',
          endpointUrl: '',
          secret: result.newSecret,
          eventTypes: [],
          createdAt: result.rotatedAt,
        });
        setShowWebhookSecretDialog(true);
      } catch (error) {
        console.error('Failed to rotate webhook secret:', error);
      }
    }
  };

  const handleToggleWebhookActive = async (id: string, isActive: boolean) => {
    try {
      await updateWebhookMutation.mutateAsync({ id, data: { isActive } });
    } catch (error) {
      console.error('Failed to toggle webhook active state:', error);
    }
  };

  const handleDownloadSdk = (language: string) => {
    downloadSdkMutation.mutate(language as SdkLanguage);
  };

  const handleViewSdkVersions = (language: string) => {
    setSelectedSdkLanguage(language as SdkLanguage);
    setShowSdkVersionsDialog(true);
  };

  // Default values for when data is loading
  const defaultAccount = {
    id: '',
    userId: '',
    contactEmail: '',
    tier: 'free' as const,
    isVerified: false,
    isActive: false,
    createdAt: '',
    updatedAt: '',
  };

  const defaultUsage = {
    developerAccountId: '',
    tier: 'free' as const,
    apiKeysCount: 0,
    webhooksCount: 0,
    totalRequestsToday: 0,
    totalRequestsMonth: 0,
    rateLimitHits: 0,
  };

  const defaultRateLimitStatus = {
    tier: 'free' as const,
    requestsPerMinute: { limit: 60, remaining: 60, resetAt: new Date().toISOString() },
    requestsPerHour: { limit: 1000, remaining: 1000, resetAt: new Date().toISOString() },
    requestsPerDay: { limit: 10000, remaining: 10000, resetAt: new Date().toISOString() },
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
            account={account ?? defaultAccount}
            usage={usage ?? defaultUsage}
            rateLimitStatus={rateLimitStatus ?? defaultRateLimitStatus}
          />
        )}

        {activeTab === 'keys' && (
          <ApiKeysList
            apiKeys={apiKeys}
            onCreateKey={() => setShowCreateKeyDialog(true)}
            onRotateKey={handleRotateApiKey}
            onRevokeKey={handleRevokeApiKey}
            onViewUsage={(id) => {
              // Navigate to usage analytics page
              window.location.href = `/developer/keys/${id}/usage`;
            }}
          />
        )}

        {activeTab === 'webhooks' && (
          <WebhooksList
            webhooks={webhooks}
            onCreateWebhook={() => setShowCreateWebhookDialog(true)}
            onEditWebhook={(id) => {
              // Navigate to webhook edit page
              window.location.href = `/developer/webhooks/${id}/edit`;
            }}
            onDeleteWebhook={handleDeleteWebhook}
            onTestWebhook={handleTestWebhook}
            onViewDeliveries={(id) => {
              // Navigate to delivery logs page
              window.location.href = `/developer/webhooks/${id}/deliveries`;
            }}
            onRotateSecret={handleRotateWebhookSecret}
            onToggleActive={handleToggleWebhookActive}
          />
        )}

        {activeTab === 'docs' && (
          <div className="space-y-8">
            <ApiDocumentation
              endpoints={[]}
              changelog={[]}
              onTestEndpoint={(endpoint) => {
                // Open API tester in new window with the endpoint
                window.open(
                  `/developer/sandbox?endpoint=${encodeURIComponent(endpoint.path)}&method=${endpoint.method}`,
                  '_blank'
                );
              }}
            />
            <ApiChangelogList changelog={[]} />
          </div>
        )}

        {activeTab === 'rate-limits' && (
          <div className="space-y-8">
            <RateLimitStatus status={rateLimitStatus ?? defaultRateLimitStatus} />
            <RateLimitTierComparison
              tiers={rateLimitTiers}
              currentTier={account?.tier ?? 'free'}
              onSelectTier={(tier) => {
                // Navigate to tier upgrade flow
                window.location.href = `/developer/upgrade?tier=${tier}`;
              }}
            />
          </div>
        )}

        {activeTab === 'sdks' && (
          <SdkDownloadList
            languages={sdkLanguages}
            onDownload={handleDownloadSdk}
            onViewVersions={handleViewSdkVersions}
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

      {/* SDK Versions Dialog */}
      {showSdkVersionsDialog && selectedSdkLanguage && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-lg font-semibold mb-4">SDK Versions - {selectedSdkLanguage}</h3>
            <p className="text-gray-600 mb-4">Organization: {organizationId}</p>
            <p className="text-sm text-gray-500">
              Version history will be available in a future update.
            </p>
            <button
              type="button"
              onClick={() => {
                setShowSdkVersionsDialog(false);
                setSelectedSdkLanguage(null);
              }}
              className="mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              Close
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
