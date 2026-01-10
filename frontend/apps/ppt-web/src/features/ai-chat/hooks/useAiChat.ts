/**
 * AI Chat Hooks
 * Epic 127: AI Chatbot Interface
 *
 * TanStack Query hooks for AI chat feature.
 */

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import type {
  ChatMessage,
  ChatSession,
  ChatSessionSummary,
  CreateSessionRequest,
  MessageFeedback,
  SendMessageRequest,
  SendMessageResponse,
} from '../types';

const API_BASE = '/api/v1/ai/chat';

/** Query keys for AI chat */
export const aiChatKeys = {
  all: ['ai-chat'] as const,
  sessions: () => [...aiChatKeys.all, 'sessions'] as const,
  session: (id: string) => [...aiChatKeys.all, 'session', id] as const,
  messages: (sessionId: string) => [...aiChatKeys.all, 'messages', sessionId] as const,
  escalated: () => [...aiChatKeys.all, 'escalated'] as const,
};

/** API helper to make fetch requests */
async function apiFetch<T>(url: string, options?: RequestInit): Promise<T> {
  const response = await fetch(url, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'Request failed' }));
    throw new Error(error.message || `HTTP error ${response.status}`);
  }

  return response.json();
}

/** Fetch user's chat sessions */
export function useAiChatSessions(limit = 50, offset = 0) {
  return useQuery({
    queryKey: aiChatKeys.sessions(),
    queryFn: async () => {
      const result = await apiFetch<{ sessions: ChatSessionSummary[] }>(
        `${API_BASE}/sessions?limit=${limit}&offset=${offset}`
      );
      return result.sessions;
    },
    staleTime: 30 * 1000, // 30 seconds
  });
}

/** Fetch a single session with details */
export function useAiChatSession(sessionId: string | null) {
  return useQuery({
    queryKey: aiChatKeys.session(sessionId ?? ''),
    queryFn: async () => {
      if (!sessionId) return null;
      return apiFetch<ChatSession>(`${API_BASE}/sessions/${sessionId}`);
    },
    enabled: !!sessionId,
  });
}

/** Fetch messages for a session */
export function useAiChatMessages(sessionId: string | null, limit = 100, offset = 0) {
  return useQuery({
    queryKey: aiChatKeys.messages(sessionId ?? ''),
    queryFn: async () => {
      if (!sessionId) return [];
      const result = await apiFetch<{ messages: ChatMessage[] }>(
        `${API_BASE}/sessions/${sessionId}/messages?limit=${limit}&offset=${offset}`
      );
      return result.messages;
    },
    enabled: !!sessionId,
    staleTime: 10 * 1000, // 10 seconds
  });
}

/** Create a new chat session */
export function useCreateSession() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (request: CreateSessionRequest) => {
      return apiFetch<ChatSession>(`${API_BASE}/sessions`, {
        method: 'POST',
        body: JSON.stringify(request),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: aiChatKeys.sessions() });
    },
  });
}

/** Send a message in a session */
export function useSendMessage(sessionId: string) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (request: SendMessageRequest) => {
      return apiFetch<SendMessageResponse>(`${API_BASE}/sessions/${sessionId}/messages`, {
        method: 'POST',
        body: JSON.stringify(request),
      });
    },
    onSuccess: (data) => {
      // Update messages cache with new messages
      queryClient.setQueryData<ChatMessage[]>(aiChatKeys.messages(sessionId), (old) => {
        if (!old) return [data.userMessage, data.assistantMessage];
        return [...old, data.userMessage, data.assistantMessage];
      });
      // Also invalidate to ensure we have latest
      queryClient.invalidateQueries({ queryKey: aiChatKeys.sessions() });
    },
  });
}

/** Delete a chat session */
export function useDeleteSession() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (sessionId: string) => {
      await fetch(`${API_BASE}/sessions/${sessionId}`, { method: 'DELETE' });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: aiChatKeys.sessions() });
    },
  });
}

/** Provide feedback on a message */
export function useMessageFeedback() {
  return useMutation({
    mutationFn: async (feedback: MessageFeedback) => {
      return apiFetch<void>(`${API_BASE}/messages/${feedback.messageId}/feedback`, {
        method: 'POST',
        body: JSON.stringify({
          rating: feedback.rating,
          helpful: feedback.helpful,
          feedback_text: feedback.feedbackText,
        }),
      });
    },
  });
}

/** Fetch escalated messages */
export function useEscalatedMessages(limit = 50, offset = 0) {
  return useQuery({
    queryKey: aiChatKeys.escalated(),
    queryFn: async () => {
      const result = await apiFetch<{ messages: ChatMessage[] }>(
        `${API_BASE}/escalated?limit=${limit}&offset=${offset}`
      );
      return result.messages;
    },
    staleTime: 60 * 1000, // 1 minute
  });
}
