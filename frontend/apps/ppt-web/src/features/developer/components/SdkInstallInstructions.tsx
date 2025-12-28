/**
 * SDK Install Instructions Component (Epic 69 - Story 69.5)
 *
 * Displays installation instructions for different SDK languages.
 */

import { useState } from 'react';
import type { SdkLanguage } from '../types';

interface SdkInstallInstructionsProps {
  language: SdkLanguage;
  packageName?: string;
  version?: string;
}

interface InstallConfig {
  displayName: string;
  packageManager: string;
  installCommand: string;
  importExample: string;
  initExample: string;
  usageExample: string;
}

const INSTALL_CONFIGS: Record<SdkLanguage, InstallConfig> = {
  typescript: {
    displayName: 'TypeScript / JavaScript',
    packageManager: 'npm / yarn / pnpm',
    installCommand: 'npm install @ppt/api-client',
    importExample: `import { PptClient } from '@ppt/api-client';`,
    initExample: `const client = new PptClient({
  apiKey: process.env.PPT_API_KEY,
  baseUrl: 'https://api.ppt.example.com/v1',
});`,
    usageExample: `// List buildings
const buildings = await client.buildings.list({
  organizationId: 'org-123',
  limit: 10,
});

// Create a fault report
const fault = await client.faults.create({
  buildingId: 'building-456',
  title: 'Broken elevator',
  description: 'Elevator on floor 3 is not working',
  priority: 'high',
});`,
  },
  python: {
    displayName: 'Python',
    packageManager: 'pip',
    installCommand: 'pip install ppt-client',
    importExample: 'from ppt_client import PptClient',
    initExample: `import os

client = PptClient(
    api_key=os.environ["PPT_API_KEY"],
    base_url="https://api.ppt.example.com/v1",
)`,
    usageExample: `# List buildings
buildings = client.buildings.list(
    organization_id="org-123",
    limit=10,
)

# Create a fault report
fault = client.faults.create(
    building_id="building-456",
    title="Broken elevator",
    description="Elevator on floor 3 is not working",
    priority="high",
)`,
  },
  go: {
    displayName: 'Go',
    packageManager: 'go get',
    installCommand: 'go get github.com/ppt/ppt-go',
    importExample: `import "github.com/ppt/ppt-go"`,
    initExample: `client := ppt.NewClient(
    ppt.WithAPIKey(os.Getenv("PPT_API_KEY")),
    ppt.WithBaseURL("https://api.ppt.example.com/v1"),
)`,
    usageExample: `// List buildings
buildings, err := client.Buildings.List(ctx, &ppt.ListBuildingsParams{
    OrganizationID: "org-123",
    Limit:          10,
})

// Create a fault report
fault, err := client.Faults.Create(ctx, &ppt.CreateFaultParams{
    BuildingID:  "building-456",
    Title:       "Broken elevator",
    Description: "Elevator on floor 3 is not working",
    Priority:    ppt.PriorityHigh,
})`,
  },
  java: {
    displayName: 'Java',
    packageManager: 'Maven / Gradle',
    installCommand: `// Maven
<dependency>
    <groupId>com.ppt</groupId>
    <artifactId>ppt-client</artifactId>
    <version>1.0.0</version>
</dependency>

// Gradle
implementation 'com.ppt:ppt-client:1.0.0'`,
    importExample: `import com.ppt.client.PptClient;
import com.ppt.client.models.*;`,
    initExample: `PptClient client = PptClient.builder()
    .apiKey(System.getenv("PPT_API_KEY"))
    .baseUrl("https://api.ppt.example.com/v1")
    .build();`,
    usageExample: `// List buildings
BuildingList buildings = client.buildings().list(
    ListBuildingsParams.builder()
        .organizationId("org-123")
        .limit(10)
        .build()
);

// Create a fault report
Fault fault = client.faults().create(
    CreateFaultParams.builder()
        .buildingId("building-456")
        .title("Broken elevator")
        .description("Elevator on floor 3 is not working")
        .priority(Priority.HIGH)
        .build()
);`,
  },
  csharp: {
    displayName: 'C# / .NET',
    packageManager: 'NuGet',
    installCommand: 'dotnet add package Ppt.Client',
    importExample: `using Ppt.Client;
using Ppt.Client.Models;`,
    initExample: `var client = new PptClient(new PptClientOptions
{
    ApiKey = Environment.GetEnvironmentVariable("PPT_API_KEY"),
    BaseUrl = "https://api.ppt.example.com/v1"
});`,
    usageExample: `// List buildings
var buildings = await client.Buildings.ListAsync(new ListBuildingsParams
{
    OrganizationId = "org-123",
    Limit = 10
});

// Create a fault report
var fault = await client.Faults.CreateAsync(new CreateFaultParams
{
    BuildingId = "building-456",
    Title = "Broken elevator",
    Description = "Elevator on floor 3 is not working",
    Priority = Priority.High
});`,
  },
};

type Tab = 'install' | 'quickstart' | 'examples';

export function SdkInstallInstructions({
  language,
  packageName,
  version,
}: SdkInstallInstructionsProps) {
  const [activeTab, setActiveTab] = useState<Tab>('install');
  const [copiedSection, setCopiedSection] = useState<string | null>(null);

  const config = INSTALL_CONFIGS[language];

  const handleCopy = async (text: string, section: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedSection(section);
      setTimeout(() => setCopiedSection(null), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const tabs = [
    { id: 'install' as Tab, label: 'Installation' },
    { id: 'quickstart' as Tab, label: 'Quick Start' },
    { id: 'examples' as Tab, label: 'Examples' },
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold">{config.displayName} SDK</h2>
          <p className="text-sm text-muted-foreground">
            {packageName && <span className="font-mono">{packageName}</span>}
            {version && <span className="ml-2">v{version}</span>}
          </p>
        </div>
        <span className="px-3 py-1 text-xs bg-blue-100 text-blue-800 rounded-full">
          {config.packageManager}
        </span>
      </div>

      {/* Tabs */}
      <div className="border-b border-gray-200">
        <nav className="-mb-px flex gap-6">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              type="button"
              onClick={() => setActiveTab(tab.id)}
              className={`py-3 px-1 border-b-2 text-sm font-medium transition-colors ${
                activeTab === tab.id
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              {tab.label}
            </button>
          ))}
        </nav>
      </div>

      {/* Installation Tab */}
      {activeTab === 'install' && (
        <div className="space-y-6">
          <div>
            <div className="flex items-center justify-between mb-2">
              <h3 className="text-sm font-medium">Install Package</h3>
              <CopyButton
                onClick={() => handleCopy(config.installCommand, 'install')}
                copied={copiedSection === 'install'}
              />
            </div>
            <CodeBlock code={config.installCommand} />
          </div>

          {/* Requirements */}
          <div className="p-4 bg-gray-50 rounded-md">
            <h4 className="text-sm font-medium mb-2">Requirements</h4>
            <ul className="text-sm text-muted-foreground space-y-1">
              {language === 'typescript' && (
                <>
                  <li>Node.js 18 or later</li>
                  <li>TypeScript 5.0+ (for TypeScript projects)</li>
                </>
              )}
              {language === 'python' && (
                <>
                  <li>Python 3.9 or later</li>
                  <li>pip or poetry for package management</li>
                </>
              )}
              {language === 'go' && (
                <>
                  <li>Go 1.21 or later</li>
                  <li>Go modules enabled</li>
                </>
              )}
              {language === 'java' && (
                <>
                  <li>Java 17 or later</li>
                  <li>Maven 3.6+ or Gradle 7+</li>
                </>
              )}
              {language === 'csharp' && (
                <>
                  <li>.NET 6.0 or later</li>
                  <li>NuGet package manager</li>
                </>
              )}
            </ul>
          </div>
        </div>
      )}

      {/* Quick Start Tab */}
      {activeTab === 'quickstart' && (
        <div className="space-y-6">
          {/* Import */}
          <div>
            <div className="flex items-center justify-between mb-2">
              <h3 className="text-sm font-medium">1. Import the SDK</h3>
              <CopyButton
                onClick={() => handleCopy(config.importExample, 'import')}
                copied={copiedSection === 'import'}
              />
            </div>
            <CodeBlock code={config.importExample} />
          </div>

          {/* Initialize */}
          <div>
            <div className="flex items-center justify-between mb-2">
              <h3 className="text-sm font-medium">2. Initialize the Client</h3>
              <CopyButton
                onClick={() => handleCopy(config.initExample, 'init')}
                copied={copiedSection === 'init'}
              />
            </div>
            <CodeBlock code={config.initExample} />
          </div>

          {/* API Key Info */}
          <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-md">
            <div className="flex gap-3">
              <svg
                className="w-5 h-5 text-yellow-600 flex-shrink-0 mt-0.5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                />
              </svg>
              <div>
                <h4 className="font-medium text-yellow-800">Security Tip</h4>
                <p className="text-sm text-yellow-700 mt-1">
                  Never commit your API key to version control. Use environment variables or a
                  secrets manager to store your credentials securely.
                </p>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Examples Tab */}
      {activeTab === 'examples' && (
        <div className="space-y-6">
          <div>
            <div className="flex items-center justify-between mb-2">
              <h3 className="text-sm font-medium">Common Operations</h3>
              <CopyButton
                onClick={() => handleCopy(config.usageExample, 'usage')}
                copied={copiedSection === 'usage'}
              />
            </div>
            <CodeBlock code={config.usageExample} />
          </div>

          {/* Additional Resources */}
          <div className="p-4 bg-blue-50 border border-blue-200 rounded-md">
            <h4 className="font-medium text-blue-800 mb-2">Additional Resources</h4>
            <ul className="text-sm text-blue-700 space-y-1">
              <li>
                <span className="hover:underline cursor-pointer">API Reference Documentation</span>
              </li>
              <li>
                <span className="hover:underline cursor-pointer">SDK GitHub Repository</span>
              </li>
              <li>
                <span className="hover:underline cursor-pointer">Code Examples on GitHub</span>
              </li>
            </ul>
          </div>
        </div>
      )}
    </div>
  );
}

// Helper Components

function CodeBlock({ code }: { code: string }) {
  return (
    <pre className="p-4 bg-gray-900 text-gray-100 rounded-md text-sm overflow-x-auto">
      <code>{code}</code>
    </pre>
  );
}

function CopyButton({ onClick, copied }: { onClick: () => void; copied: boolean }) {
  return (
    <button
      type="button"
      onClick={onClick}
      className={`px-3 py-1 text-xs rounded transition-colors ${
        copied ? 'bg-green-600 text-white' : 'bg-gray-100 hover:bg-gray-200 text-gray-700'
      }`}
    >
      {copied ? 'Copied!' : 'Copy'}
    </button>
  );
}
