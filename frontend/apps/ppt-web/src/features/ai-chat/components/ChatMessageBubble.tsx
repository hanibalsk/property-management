/**
 * ChatMessageBubble component
 * Epic 127: AI Chatbot Interface
 *
 * Displays a single chat message with role-specific styling,
 * confidence indicators, and source references.
 */

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { ChatMessage, ConfidenceLevel } from '../types';

interface ChatMessageBubbleProps {
  message: ChatMessage;
  onFeedback?: (messageId: string, helpful: boolean) => void;
  showFeedback?: boolean;
}

function getConfidenceLevel(confidence: number | undefined): ConfidenceLevel {
  if (confidence === undefined) return 'medium';
  if (confidence >= 0.8) return 'high';
  if (confidence >= 0.5) return 'medium';
  return 'low';
}

function formatTime(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

export function ChatMessageBubble({
  message,
  onFeedback,
  showFeedback = true,
}: ChatMessageBubbleProps) {
  const { t } = useTranslation();
  const [showSources, setShowSources] = useState(false);
  const [feedbackGiven, setFeedbackGiven] = useState<boolean | null>(null);

  const isUser = message.role === 'user';
  const isAssistant = message.role === 'assistant';
  const confidenceLevel = getConfidenceLevel(message.confidence);

  const handleFeedback = (helpful: boolean) => {
    if (onFeedback && feedbackGiven === null) {
      onFeedback(message.id, helpful);
      setFeedbackGiven(helpful);
    }
  };

  const confidenceColors = {
    high: 'bg-green-100 text-green-800',
    medium: 'bg-yellow-100 text-yellow-800',
    low: 'bg-red-100 text-red-800',
  };

  return (
    <div
      className={`flex ${isUser ? 'justify-end' : 'justify-start'} mb-4`}
      role="article"
      aria-label={t('aiChat.messageFrom', { role: message.role })}
    >
      <div
        className={`max-w-[80%] rounded-lg p-4 ${
          isUser ? 'bg-blue-600 text-white' : 'bg-gray-100 text-gray-900'
        }`}
      >
        {/* Message content */}
        <div className="whitespace-pre-wrap break-words">{message.content}</div>

        {/* Assistant-specific metadata */}
        {isAssistant && (
          <div className="mt-3 pt-3 border-t border-gray-200 space-y-2">
            {/* Confidence indicator */}
            {message.confidence !== undefined && (
              <div className="flex items-center gap-2 text-sm">
                <span
                  className={`px-2 py-0.5 rounded-full text-xs font-medium ${confidenceColors[confidenceLevel]}`}
                >
                  {t(`aiChat.confidence.${confidenceLevel}`)}
                </span>
                <span className="text-gray-500">{Math.round(message.confidence * 100)}%</span>
              </div>
            )}

            {/* Escalation warning */}
            {message.escalated && (
              <div
                className="flex items-center gap-2 text-amber-700 bg-amber-50 px-3 py-2 rounded text-sm"
                role="alert"
              >
                <svg
                  className="w-4 h-4 flex-shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-2.694-.833-3.464 0L3.34 16.5c-.77.833.192 2.5 1.732 2.5z"
                  />
                </svg>
                <span>{message.escalationReason || t('aiChat.escalated')}</span>
              </div>
            )}

            {/* Sources toggle */}
            {message.sources.length > 0 && (
              <div>
                <button
                  type="button"
                  onClick={() => setShowSources(!showSources)}
                  className="flex items-center gap-1 text-sm text-gray-600 hover:text-gray-800"
                  aria-expanded={showSources}
                  aria-controls={`sources-${message.id}`}
                >
                  <svg
                    className={`w-4 h-4 transition-transform ${showSources ? 'rotate-90' : ''}`}
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    aria-hidden="true"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M9 5l7 7-7 7"
                    />
                  </svg>
                  {t('aiChat.sources', { count: message.sources.length })}
                </button>

                {/* Sources list */}
                {showSources && (
                  <ul id={`sources-${message.id}`} className="mt-2 space-y-1 pl-4">
                    {message.sources.map((source, idx) => (
                      <li key={`${source.sourceId}-${idx}`} className="text-sm text-gray-600">
                        <span className="font-medium">{source.title}</span>
                        {source.snippet && (
                          <p className="text-gray-500 text-xs mt-0.5 line-clamp-2">
                            {source.snippet}
                          </p>
                        )}
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            )}

            {/* Feedback buttons */}
            {showFeedback && onFeedback && (
              <div className="flex items-center gap-3 text-sm">
                <span className="text-gray-500">{t('aiChat.wasHelpful')}</span>
                <div className="flex gap-1">
                  <button
                    type="button"
                    onClick={() => handleFeedback(true)}
                    disabled={feedbackGiven !== null}
                    className={`p-1.5 rounded transition-colors ${
                      feedbackGiven === true
                        ? 'bg-green-100 text-green-700'
                        : feedbackGiven === null
                          ? 'hover:bg-gray-200 text-gray-600'
                          : 'text-gray-300 cursor-not-allowed'
                    }`}
                    aria-label={t('aiChat.helpful')}
                    aria-pressed={feedbackGiven === true}
                  >
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      aria-hidden="true"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5"
                      />
                    </svg>
                  </button>
                  <button
                    type="button"
                    onClick={() => handleFeedback(false)}
                    disabled={feedbackGiven !== null}
                    className={`p-1.5 rounded transition-colors ${
                      feedbackGiven === false
                        ? 'bg-red-100 text-red-700'
                        : feedbackGiven === null
                          ? 'hover:bg-gray-200 text-gray-600'
                          : 'text-gray-300 cursor-not-allowed'
                    }`}
                    aria-label={t('aiChat.notHelpful')}
                    aria-pressed={feedbackGiven === false}
                  >
                    <svg
                      className="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                      aria-hidden="true"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M10 14H5.236a2 2 0 01-1.789-2.894l3.5-7A2 2 0 018.736 3h4.018a2 2 0 01.485.06l3.76.94m-7 10v5a2 2 0 002 2h.096c.5 0 .905-.405.905-.904 0-.715.211-1.413.608-2.008L17 13V4m-7 10h2m5-10h2a2 2 0 012 2v6a2 2 0 01-2 2h-2.5"
                      />
                    </svg>
                  </button>
                </div>
              </div>
            )}

            {/* Latency info (for debugging, could be hidden in production) */}
            {message.latencyMs && (
              <div className="text-xs text-gray-400">
                {t('aiChat.responseTime', { ms: message.latencyMs })}
              </div>
            )}
          </div>
        )}

        {/* Timestamp */}
        <div className={`text-xs mt-2 ${isUser ? 'text-blue-200' : 'text-gray-400'}`}>
          {formatTime(message.createdAt)}
        </div>
      </div>
    </div>
  );
}
