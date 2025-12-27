/**
 * SDK Download List Component (Epic 69 - Story 69.5)
 *
 * Displays available SDKs with download links and installation instructions.
 */

import { useState } from 'react';
import type { SdkLanguageInfo, SdkVersion } from '../types';

interface SdkDownloadListProps {
  languages: SdkLanguageInfo[];
  onDownload: (language: string) => void;
  onViewVersions: (language: string) => void;
}

export function SdkDownloadList({ languages, onDownload, onViewVersions }: SdkDownloadListProps) {
  const [copiedCommand, setCopiedCommand] = useState<string | null>(null);

  const handleCopyCommand = async (language: string, command: string) => {
    try {
      await navigator.clipboard.writeText(command);
      setCopiedCommand(language);
      setTimeout(() => setCopiedCommand(null), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight">SDKs</h2>
        <p className="text-muted-foreground">
          Download official SDKs to integrate with the API faster
        </p>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {languages.map((lang) => (
          <SdkCard
            key={lang.language}
            language={lang}
            isCopied={copiedCommand === lang.language}
            onCopyCommand={(cmd) => handleCopyCommand(lang.language, cmd)}
            onDownload={() => onDownload(lang.language)}
            onViewVersions={() => onViewVersions(lang.language)}
          />
        ))}
      </div>

      {/* SDK Features */}
      <div className="mt-8 p-6 bg-gray-50 rounded-lg">
        <h3 className="text-lg font-semibold mb-4">SDK Features</h3>
        <div className="grid md:grid-cols-3 gap-6">
          <FeatureItem
            icon={
              <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                />
              </svg>
            }
            title="Type Safe"
            description="Full type definitions for all API endpoints and models"
          />
          <FeatureItem
            icon={
              <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                />
              </svg>
            }
            title="Auto Retry"
            description="Built-in retry logic with exponential backoff"
          />
          <FeatureItem
            icon={
              <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
                />
              </svg>
            }
            title="Configurable"
            description="Easy configuration for timeouts, base URLs, and headers"
          />
        </div>
      </div>

      {/* Code Examples */}
      <div className="mt-8">
        <h3 className="text-lg font-semibold mb-4">Quick Start Example</h3>
        <div className="bg-gray-900 rounded-lg overflow-hidden">
          <div className="flex border-b border-gray-700">
            <TabButton active>TypeScript</TabButton>
            <TabButton>Python</TabButton>
            <TabButton>Go</TabButton>
          </div>
          <pre className="p-4 text-sm text-gray-100 overflow-x-auto">
            <code>
{`import { PptClient } from '@ppt/api-client';

// Initialize the client
const client = new PptClient({
  apiKey: 'your-api-key',
  baseUrl: 'https://api.ppt.example.com/v1',
});

// List buildings
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
});

console.log('Created fault:', fault.id);`}
            </code>
          </pre>
        </div>
      </div>
    </div>
  );
}

// ==================== Helper Components ====================

interface SdkCardProps {
  language: SdkLanguageInfo;
  isCopied: boolean;
  onCopyCommand: (command: string) => void;
  onDownload: () => void;
  onViewVersions: () => void;
}

function SdkCard({ language, isCopied, onCopyCommand, onDownload, onViewVersions }: SdkCardProps) {
  const getLanguageIcon = (lang: string) => {
    switch (lang) {
      case 'typescript':
        return (
          <svg className="w-8 h-8" viewBox="0 0 128 128">
            <path fill="#007acc" d="M2 63.91v62.5h125v-125H2zm100.73-5a15.56 15.56 0 017.82 4.5 20.58 20.58 0 013 4.22c.04.08-5.24 3.73-8.44 5.86-.12.08-.6-.46-1.08-1.22a6.47 6.47 0 00-5.36-3.28c-3.44-.24-5.66 1.58-5.64 4.6a4.15 4.15 0 00.48 2.14c.72 1.46 2.1 2.36 6.64 4.34 8.38 3.68 11.96 6.08 14.04 9.44 2.3 3.72 2.82 9.68 1.24 14a15.94 15.94 0 01-13.62 9.02c-1.48.08-5.18-.08-6.76-.3a19.77 19.77 0 01-11.96-7c-1.38-1.74-4.06-6.26-3.9-6.56.06-.1.56-.4 1.12-.72l4.54-2.62 3.52-2.04.74 1.08a16.14 16.14 0 005.04 4.74c2.34 1.24 7.68 1.08 9.46-.28a4.96 4.96 0 001.86-4.2 4.6 4.6 0 00-.68-2.68c-.92-1.52-2.82-2.66-7.92-4.78-5.84-2.42-8.36-3.92-10.52-6.22-1.26-1.32-2.44-3.42-2.94-5.2-.4-1.44-.5-5.06-.16-6.54 1.12-5 5-8.92 10.26-10.44 1.72-.5 6.82-.58 8.62-.14zM73.37 63.71l.04 5.2h-16.5v47h-10.6v-47h-16.5v-5.08c0-2.82.08-5.2.2-5.28s9.86-.12 21.66-.1l21.46.08z"/>
          </svg>
        );
      case 'python':
        return (
          <svg className="w-8 h-8" viewBox="0 0 128 128">
            <path fill="#3776AB" d="M63.391 1.988c-4.222.02-8.252.379-11.8 1.007-10.45 1.846-12.346 5.71-12.346 12.837v9.411h24.693v3.137H29.977c-7.176 0-13.46 4.313-15.426 12.521-2.268 9.405-2.368 15.275 0 25.096 1.755 7.311 5.947 12.519 13.124 12.519h8.491V67.234c0-8.151 7.051-15.34 15.426-15.34h24.665c6.866 0 12.346-5.654 12.346-12.548V15.833c0-6.693-5.646-11.72-12.346-12.837-4.244-.706-8.645-1.027-12.866-1.008zM50.037 9.557c2.55 0 4.634 2.117 4.634 4.721 0 2.593-2.083 4.69-4.634 4.69-2.56 0-4.633-2.097-4.633-4.69-.001-2.604 2.073-4.721 4.633-4.721z"/>
            <path fill="#FFC331" d="M91.682 28.38v10.966c0 8.5-7.208 15.655-15.426 15.655H51.591c-6.756 0-12.346 5.783-12.346 12.549v23.515c0 6.691 5.818 10.628 12.346 12.547 7.816 2.297 15.312 2.713 24.665 0 6.216-1.801 12.346-5.423 12.346-12.547v-9.412H63.938v-3.138h37.012c7.176 0 9.852-5.005 12.348-12.519 2.578-7.735 2.467-15.174 0-25.096-1.774-7.145-5.161-12.521-12.348-12.521h-9.268zM77.809 87.927c2.561 0 4.634 2.097 4.634 4.692 0 2.602-2.074 4.719-4.634 4.719-2.55 0-4.633-2.117-4.633-4.719 0-2.595 2.083-4.692 4.633-4.692z"/>
          </svg>
        );
      case 'go':
        return (
          <svg className="w-8 h-8" viewBox="0 0 128 128">
            <path fill="#00acd7" d="M101.83 55.69c-.19 0-.31.07-.44.25l-2.56 3.31c-.13.19-.13.37 0 .5l7.62 6.12c.19.19.44.19.63 0l2.56-3.31c.19-.19.19-.37 0-.5l-7.56-6.12a.562.562 0 00-.25-.25zM84.96 55c-.19 0-.37.07-.5.19l-6.12 4.81c-.19.13-.25.37-.13.56l4.5 9.25c.07.13.19.25.37.25h3.81c.25 0 .44-.13.5-.37l3.25-10.75c.07-.25-.06-.5-.31-.56l-5.19-3.19c-.06-.07-.12-.19-.18-.19zm-46.21.12c-.19 0-.37.07-.5.19L27.25 63c-.19.13-.19.37-.06.56l8.81 10.12c.13.13.31.19.5.19h4.25c.25 0 .44-.13.5-.37l2.5-9.94c.07-.19-.06-.44-.25-.56l-4.56-7.62c-.08-.13-.19-.26-.19-.26zm25.44.13c-.25 0-.44.19-.44.44v13.44c0 .25.19.44.44.44h3.37c.25 0 .44-.19.44-.44v-4.62l3.56 4.87c.13.13.25.19.44.19h4c.25 0 .37-.25.19-.44l-4.62-5.87 4.25-5.5c.13-.19.06-.44-.19-.44h-3.87c-.19 0-.37.07-.44.19l-3.25 4.37v-6c0-.25-.19-.44-.44-.44h-3.44zm-34.13.12c-.25 0-.44.19-.44.44v13.44c0 .25.19.44.44.44h3.37c.25 0 .44-.19.44-.44V60.5l4.12 8.75c.07.13.19.19.37.19h.69c.19 0 .31-.06.37-.19l4.12-8.75v8.75c0 .25.19.44.44.44h3.37c.25 0 .44-.19.44-.44V55.81c0-.25-.19-.44-.44-.44H43.9c-.25 0-.37.13-.44.37l-3.5 7.44-3.5-7.44a.53.53 0 00-.44-.37l-6-.12z"/>
          </svg>
        );
      default:
        return (
          <svg className="w-8 h-8 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
          </svg>
        );
    }
  };

  return (
    <div className="p-6 bg-white rounded-lg border shadow-sm">
      <div className="flex items-center gap-4 mb-4">
        {getLanguageIcon(language.language)}
        <div>
          <h3 className="font-semibold">{language.displayName}</h3>
          <p className="text-sm text-muted-foreground">
            via {language.packageManager}
          </p>
        </div>
      </div>

      {language.latestVersion && (
        <div className="mb-4">
          <span className="inline-flex items-center px-2 py-1 text-xs bg-green-100 text-green-800 rounded-full">
            v{language.latestVersion}
          </span>
        </div>
      )}

      {/* Installation Command */}
      <div className="mb-4">
        <label className="block text-xs font-medium text-muted-foreground mb-1">
          Installation
        </label>
        <div className="flex gap-2">
          <code className="flex-1 px-3 py-2 bg-gray-100 text-sm font-mono rounded-md truncate">
            {language.installationCommand}
          </code>
          <button
            type="button"
            onClick={() => onCopyCommand(language.installationCommand)}
            className={`px-3 py-2 rounded-md transition-colors ${
              isCopied
                ? 'bg-green-600 text-white'
                : 'bg-gray-100 hover:bg-gray-200 text-gray-700'
            }`}
          >
            {isCopied ? (
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
            ) : (
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3"
                />
              </svg>
            )}
          </button>
        </div>
      </div>

      {/* Actions */}
      <div className="flex gap-2">
        <button
          type="button"
          onClick={onDownload}
          className="flex-1 px-4 py-2 bg-blue-600 text-white text-sm rounded-md hover:bg-blue-700 transition-colors"
        >
          Download
        </button>
        <button
          type="button"
          onClick={onViewVersions}
          className="px-4 py-2 border text-sm rounded-md hover:bg-gray-50 transition-colors"
        >
          Versions
        </button>
      </div>

      {language.documentationUrl && (
        <a
          href={language.documentationUrl}
          target="_blank"
          rel="noopener noreferrer"
          className="mt-3 block text-center text-sm text-blue-600 hover:underline"
        >
          View Documentation
        </a>
      )}
    </div>
  );
}

interface FeatureItemProps {
  icon: React.ReactNode;
  title: string;
  description: string;
}

function FeatureItem({ icon, title, description }: FeatureItemProps) {
  return (
    <div className="flex gap-3">
      <div className="p-2 bg-blue-100 text-blue-600 rounded-lg h-fit">{icon}</div>
      <div>
        <h4 className="font-medium">{title}</h4>
        <p className="text-sm text-muted-foreground">{description}</p>
      </div>
    </div>
  );
}

function TabButton({ active, children }: { active?: boolean; children: React.ReactNode }) {
  return (
    <button
      type="button"
      className={`px-4 py-2 text-sm ${
        active
          ? 'bg-gray-800 text-white'
          : 'text-gray-400 hover:text-gray-200'
      }`}
    >
      {children}
    </button>
  );
}
