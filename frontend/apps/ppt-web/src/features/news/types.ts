/**
 * News & Media Management types (Epic 59)
 */

export type ArticleStatus = 'draft' | 'published' | 'archived';

export type ReactionType = 'like' | 'love' | 'surprised' | 'sad' | 'angry';

export interface NewsArticle {
  id: string;
  organizationId: string;
  authorId: string;
  title: string;
  content: string;
  excerpt: string | null;
  coverImageUrl: string | null;
  buildingIds: string[];
  status: ArticleStatus;
  publishedAt: string | null;
  archivedAt: string | null;
  pinned: boolean;
  pinnedAt: string | null;
  pinnedBy: string | null;
  commentsEnabled: boolean;
  reactionsEnabled: boolean;
  viewCount: number;
  reactionCount: number;
  commentCount: number;
  shareCount: number;
  createdAt: string;
  updatedAt: string;
}

export interface ArticleSummary {
  id: string;
  title: string;
  excerpt: string | null;
  coverImageUrl: string | null;
  authorId: string;
  status: ArticleStatus;
  publishedAt: string | null;
  pinned: boolean;
  viewCount: number;
  reactionCount: number;
  commentCount: number;
  createdAt: string;
}

export interface ArticleMedia {
  id: string;
  articleId: string;
  mediaType: string;
  fileKey: string | null;
  fileName: string | null;
  fileSize: number | null;
  mimeType: string | null;
  embedUrl: string | null;
  embedHtml: string | null;
  width: number | null;
  height: number | null;
  altText: string | null;
  caption: string | null;
  displayOrder: number;
  createdAt: string;
}

export interface ArticleReaction {
  id: string;
  articleId: string;
  userId: string;
  reaction: ReactionType;
  createdAt: string;
}

export interface ReactionCounts {
  like: number;
  love: number;
  surprised: number;
  sad: number;
  angry: number;
  total: number;
}

export interface ArticleComment {
  id: string;
  articleId: string;
  userId: string;
  parentId: string | null;
  content: string;
  isModerated: boolean;
  moderatedAt: string | null;
  moderatedBy: string | null;
  moderationReason: string | null;
  deletedAt: string | null;
  deletedBy: string | null;
  likeCount: number;
  createdAt: string;
  updatedAt: string;
}

export interface ArticleCommentWithAuthor extends ArticleComment {
  authorName: string;
  authorAvatarUrl: string | null;
  replyCount: number;
}

// Alias for backend naming consistency
export type CommentWithAuthor = ArticleCommentWithAuthor;

export interface CreateArticleRequest {
  title: string;
  content: string;
  excerpt?: string;
  coverImageUrl?: string;
  buildingIds?: string[];
  status?: ArticleStatus;
  publishedAt?: string;
  commentsEnabled?: boolean;
  reactionsEnabled?: boolean;
}

export interface UpdateArticleRequest {
  title?: string;
  content?: string;
  excerpt?: string;
  coverImageUrl?: string;
  buildingIds?: string[];
  status?: ArticleStatus;
  commentsEnabled?: boolean;
  reactionsEnabled?: boolean;
}

export interface ArticleListQuery {
  status?: ArticleStatus;
  buildingId?: string;
  pinnedOnly?: boolean;
  limit?: number;
  offset?: number;
}

export interface ArticleStatistics {
  totalArticles: number;
  publishedArticles: number;
  draftArticles: number;
  archivedArticles: number;
  totalViews: number;
  totalReactions: number;
  totalComments: number;
}
