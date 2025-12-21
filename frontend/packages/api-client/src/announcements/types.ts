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
