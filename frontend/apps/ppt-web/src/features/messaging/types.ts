/**
 * Messaging Feature Types
 *
 * TypeScript types for the messaging feature.
 */

export interface MessageAttachment {
  id: string;
  name: string;
  type: string;
  size: number;
  url: string;
}

export interface MessageThreadParticipant {
  id: string;
  userId: string;
  userName: string;
  userAvatar?: string;
  joinedAt: string;
  lastReadAt?: string;
}

export interface MessageThread {
  id: string;
  subject?: string;
  participantCount: number;
  messageCount: number;
  lastMessageAt?: string;
  lastMessagePreview?: string;
  lastMessageSenderId?: string;
  lastMessageSenderName?: string;
  unreadCount: number;
  createdAt: string;
  updatedAt: string;
  participants: MessageThreadParticipant[];
  isArchived?: boolean;
}

export interface Message {
  id: string;
  threadId: string;
  senderId: string;
  senderName: string;
  senderAvatar?: string;
  content: string;
  createdAt: string;
  updatedAt?: string;
  readBy: string[];
  attachments?: MessageAttachment[];
  replyToId?: string;
  replyToContent?: string;
  replyToSenderName?: string;
}

export interface ThreadWithMessages extends MessageThread {
  messages: Message[];
}

export interface CreateThreadRequest {
  recipientIds: string[];
  subject?: string;
  initialMessage: string;
}

export interface SendMessageRequest {
  content: string;
  attachments?: MessageAttachment[];
  replyToId?: string;
}

export interface BlockedUser {
  id: string;
  userId: string;
  blockedUserId: string;
  blockedUserName: string;
  blockedUserAvatar?: string;
  blockedAt: string;
  reason?: string;
}

export type ThreadFilterTab = 'all' | 'unread' | 'archived';

export interface ListThreadsParams {
  page?: number;
  pageSize?: number;
  search?: string;
  filter?: ThreadFilterTab;
}

export interface RecipientOption {
  id: string;
  name: string;
  avatar?: string;
  email?: string;
}
