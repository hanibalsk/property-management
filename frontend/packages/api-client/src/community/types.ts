/**
 * Community Types
 *
 * Type definitions for the Community API (Epic 42).
 * Includes Groups, Posts, Events, and Marketplace.
 */

// ============================================
// Story 42.1: Community Groups
// ============================================

export type GroupVisibility = 'public' | 'private' | 'hidden';
export type GroupMemberRole = 'owner' | 'admin' | 'moderator' | 'member';
export type GroupMemberStatus = 'pending' | 'active' | 'banned';

export interface CommunityGroup {
  id: string;
  buildingId: string;
  name: string;
  description: string;
  category: string;
  visibility: GroupVisibility;
  coverImageUrl?: string;
  memberCount: number;
  postCount: number;
  isOfficial: boolean;
  createdBy: string;
  createdAt: string;
  updatedAt: string;
}

export interface CommunityGroupSummary {
  id: string;
  name: string;
  description: string;
  category: string;
  visibility: GroupVisibility;
  coverImageUrl?: string;
  memberCount: number;
  isOfficial: boolean;
  isMember: boolean;
}

export interface GroupMember {
  id: string;
  groupId: string;
  userId: string;
  userName: string;
  userAvatar?: string;
  role: GroupMemberRole;
  status: GroupMemberStatus;
  joinedAt: string;
}

export interface CreateGroupRequest {
  name: string;
  description: string;
  category: string;
  visibility?: GroupVisibility;
  coverImageUrl?: string;
}

export interface UpdateGroupRequest {
  name?: string;
  description?: string;
  category?: string;
  visibility?: GroupVisibility;
  coverImageUrl?: string;
}

export interface ListGroupsParams {
  page?: number;
  pageSize?: number;
  category?: string;
  visibility?: GroupVisibility;
  search?: string;
  memberOnly?: boolean;
}

// ============================================
// Story 42.2: Community Feed / Posts
// ============================================

export type PostType = 'text' | 'image' | 'poll' | 'event' | 'item';
export type PostVisibility = 'group' | 'building' | 'public';

export interface CommunityPost {
  id: string;
  groupId: string;
  authorId: string;
  authorName: string;
  authorAvatar?: string;
  type: PostType;
  content: string;
  mediaUrls: string[];
  visibility: PostVisibility;
  likeCount: number;
  commentCount: number;
  shareCount: number;
  isPinned: boolean;
  isLikedByUser: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface PostComment {
  id: string;
  postId: string;
  authorId: string;
  authorName: string;
  authorAvatar?: string;
  parentId?: string;
  content: string;
  likeCount: number;
  isLikedByUser: boolean;
  createdAt: string;
  updatedAt: string;
  replies?: PostComment[];
}

export interface CreatePostRequest {
  groupId: string;
  type?: PostType;
  content: string;
  mediaUrls?: string[];
  visibility?: PostVisibility;
}

export interface UpdatePostRequest {
  content?: string;
  mediaUrls?: string[];
}

export interface CreatePostCommentRequest {
  content: string;
  parentId?: string;
}

export interface ListPostsParams {
  page?: number;
  pageSize?: number;
  groupId?: string;
  authorId?: string;
  type?: PostType;
  feed?: 'all' | 'my_groups' | 'building';
}

// ============================================
// Story 42.3: Community Events
// ============================================

export type EventStatus = 'draft' | 'published' | 'cancelled' | 'completed';
export type RsvpStatus = 'going' | 'maybe' | 'not_going';

export interface CommunityEvent {
  id: string;
  groupId: string;
  title: string;
  description: string;
  location: string;
  locationDetails?: string;
  startDate: string;
  endDate: string;
  allDay: boolean;
  coverImageUrl?: string;
  status: EventStatus;
  maxAttendees?: number;
  goingCount: number;
  maybeCount: number;
  notGoingCount: number;
  userRsvp?: RsvpStatus;
  createdBy: string;
  createdAt: string;
  updatedAt: string;
}

export interface EventAttendee {
  id: string;
  eventId: string;
  userId: string;
  userName: string;
  userAvatar?: string;
  rsvpStatus: RsvpStatus;
  rsvpAt: string;
}

export interface CreateEventRequest {
  groupId: string;
  title: string;
  description: string;
  location: string;
  locationDetails?: string;
  startDate: string;
  endDate: string;
  allDay?: boolean;
  coverImageUrl?: string;
  maxAttendees?: number;
}

export interface UpdateEventRequest {
  title?: string;
  description?: string;
  location?: string;
  locationDetails?: string;
  startDate?: string;
  endDate?: string;
  allDay?: boolean;
  coverImageUrl?: string;
  maxAttendees?: number;
  status?: EventStatus;
}

export interface RsvpRequest {
  status: RsvpStatus;
}

export interface ListEventsParams {
  page?: number;
  pageSize?: number;
  groupId?: string;
  status?: EventStatus;
  fromDate?: string;
  toDate?: string;
  userRsvp?: RsvpStatus;
}

// ============================================
// Story 42.4: Item Marketplace
// ============================================

export type ItemCondition = 'new' | 'like_new' | 'good' | 'fair' | 'poor';
export type ItemListingType = 'sale' | 'free' | 'wanted' | 'trade';
export type ItemStatus = 'active' | 'reserved' | 'sold' | 'expired' | 'removed';

export interface MarketplaceItem {
  id: string;
  sellerId: string;
  sellerName: string;
  sellerAvatar?: string;
  sellerUnit?: string;
  title: string;
  description: string;
  category: string;
  condition: ItemCondition;
  listingType: ItemListingType;
  price?: number;
  currency?: string;
  imageUrls: string[];
  status: ItemStatus;
  viewCount: number;
  messageCount: number;
  createdAt: string;
  updatedAt: string;
  expiresAt?: string;
}

export interface CreateItemRequest {
  title: string;
  description: string;
  category: string;
  condition: ItemCondition;
  listingType: ItemListingType;
  price?: number;
  currency?: string;
  imageUrls?: string[];
}

export interface UpdateItemRequest {
  title?: string;
  description?: string;
  category?: string;
  condition?: ItemCondition;
  listingType?: ItemListingType;
  price?: number;
  currency?: string;
  imageUrls?: string[];
  status?: ItemStatus;
}

export interface ListItemsParams {
  page?: number;
  pageSize?: number;
  category?: string;
  condition?: ItemCondition;
  listingType?: ItemListingType;
  status?: ItemStatus;
  sellerId?: string;
  minPrice?: number;
  maxPrice?: number;
  search?: string;
}

export interface ContactSellerRequest {
  message: string;
}

// ============================================
// Shared Types
// ============================================

// Note: PaginatedResponse is re-exported from ../announcements
// to avoid duplicate exports

export interface CommunityStats {
  totalGroups: number;
  totalMembers: number;
  totalPosts: number;
  totalEvents: number;
  totalItems: number;
  activeListings: number;
  upcomingEvents: number;
}
