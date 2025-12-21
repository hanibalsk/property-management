/**
 * Announcement Types
 *
 * Type definitions for the Announcements API (UC-06).
 */

export type AnnouncementStatus = 'draft' | 'scheduled' | 'published' | 'archived';
export type AnnouncementTargetType = 'all' | 'building' | 'units' | 'roles';

export interface Announcement {
  id: string;
  organizationId: string;
  authorId: string;
  title: string;
  content: string;
  targetType: AnnouncementTargetType;
  targetIds: string[];
  status: AnnouncementStatus;
  scheduledAt?: string;
  publishedAt?: string;
  pinned: boolean;
  pinnedAt?: string;
  pinnedBy?: string;
  commentsEnabled: boolean;
  acknowledgmentRequired: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface AnnouncementSummary {
  id: string;
  title: string;
  status: AnnouncementStatus;
  targetType: AnnouncementTargetType;
  publishedAt?: string;
  pinned: boolean;
  commentsEnabled: boolean;
  acknowledgmentRequired: boolean;
}

export interface AnnouncementWithDetails extends Announcement {
  authorName: string;
  readCount: number;
  acknowledgedCount: number;
  commentCount: number;
  attachmentCount: number;
}

export interface AnnouncementAttachment {
  id: string;
  announcementId: string;
  fileKey: string;
  fileName: string;
  fileType: string;
  fileSize: number;
  createdAt: string;
}

export interface AnnouncementStatistics {
  total: number;
  published: number;
  draft: number;
  scheduled: number;
  archived: number;
}

export interface CreateAnnouncementRequest {
  title: string;
  content: string;
  targetType: AnnouncementTargetType;
  targetIds?: string[];
  scheduledAt?: string;
  commentsEnabled?: boolean;
  acknowledgmentRequired?: boolean;
}

export interface UpdateAnnouncementRequest {
  title?: string;
  content?: string;
  targetType?: AnnouncementTargetType;
  targetIds?: string[];
  scheduledAt?: string;
  commentsEnabled?: boolean;
  acknowledgmentRequired?: boolean;
}

export interface ScheduleAnnouncementRequest {
  scheduledAt: string;
}

export interface PinAnnouncementRequest {
  pinned: boolean;
}

export interface AddAttachmentRequest {
  fileKey: string;
  fileName: string;
  fileType: string;
  fileSize: number;
}

export interface ListAnnouncementsParams {
  page?: number;
  pageSize?: number;
  status?: AnnouncementStatus;
  targetType?: AnnouncementTargetType;
  authorId?: string;
  pinned?: boolean;
  fromDate?: string;
  toDate?: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
}

// Story 6.2: Announcement Viewing & Acknowledgment

export interface AnnouncementRead {
  id: string;
  announcementId: string;
  userId: string;
  readAt: string;
  acknowledgedAt?: string;
}

export interface UnreadCountResponse {
  unreadCount: number;
}

export interface AcknowledgmentStats {
  announcementId: string;
  totalTargeted: number;
  readCount: number;
  acknowledgedCount: number;
  pendingCount: number;
}

export interface UserAcknowledgmentStatus {
  userId: string;
  userName: string;
  readAt?: string;
  acknowledgedAt?: string;
}

export interface AcknowledgmentStatsResponse {
  stats: AcknowledgmentStats;
}

export interface AcknowledgmentListResponse {
  users: UserAcknowledgmentStatus[];
  count: number;
}

// Story 6.3: Announcement Comments & Discussion

export interface AnnouncementComment {
  id: string;
  announcementId: string;
  userId: string;
  parentId?: string;
  content: string;
  aiTrainingConsent: boolean;
  deletedAt?: string;
  deletedBy?: string;
  deletionReason?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CommentWithAuthor {
  id: string;
  announcementId: string;
  userId: string;
  parentId?: string;
  content: string;
  authorName: string;
  isDeleted: boolean;
  createdAt: string;
  updatedAt: string;
  replies?: CommentWithAuthor[];
}

export interface CreateCommentRequest {
  content: string;
  parentId?: string;
  aiTrainingConsent?: boolean;
}

export interface DeleteCommentRequest {
  reason?: string;
}

export interface CommentsResponse {
  comments: CommentWithAuthor[];
  count: number;
  total: number;
}

export interface ListCommentsParams {
  limit?: number;
  offset?: number;
}
