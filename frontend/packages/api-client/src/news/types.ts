/**
 * News & Media Management types (Epic 59)
 */

export type ArticleStatus = 'draft' | 'published' | 'archived';

export type ReactionType = 'like' | 'love' | 'surprised' | 'sad' | 'angry';

export interface NewsArticle {
  id: string;
  organization_id: string;
  author_id: string;
  title: string;
  content: string;
  excerpt: string | null;
  cover_image_url: string | null;
  building_ids: string[];
  status: ArticleStatus;
  published_at: string | null;
  archived_at: string | null;
  pinned: boolean;
  pinned_at: string | null;
  pinned_by: string | null;
  comments_enabled: boolean;
  reactions_enabled: boolean;
  view_count: number;
  reaction_count: number;
  comment_count: number;
  share_count: number;
  created_at: string;
  updated_at: string;
}

export interface ArticleSummary {
  id: string;
  title: string;
  excerpt: string | null;
  cover_image_url: string | null;
  author_id: string;
  author_name?: string;
  status: ArticleStatus;
  published_at: string | null;
  pinned: boolean;
  view_count: number;
  reaction_count: number;
  comment_count: number;
  created_at: string;
}

export interface ArticleWithDetails extends NewsArticle {
  author_name: string;
  author_avatar_url: string | null;
}

export interface ArticleMedia {
  id: string;
  article_id: string;
  media_type: string;
  file_key: string | null;
  file_name: string | null;
  file_size: number | null;
  mime_type: string | null;
  embed_url: string | null;
  embed_html: string | null;
  width: number | null;
  height: number | null;
  alt_text: string | null;
  caption: string | null;
  display_order: number;
  created_at: string;
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
  article_id: string;
  user_id: string;
  parent_id: string | null;
  content: string;
  is_moderated: boolean;
  moderated_at: string | null;
  moderated_by: string | null;
  moderation_reason: string | null;
  deleted_at: string | null;
  deleted_by: string | null;
  like_count: number;
  created_at: string;
  updated_at: string;
}

export interface NewsCommentWithAuthor extends ArticleComment {
  author_name: string;
  author_avatar_url: string | null;
  reply_count: number;
}

export interface CreateArticleRequest {
  title: string;
  content: string;
  excerpt?: string;
  cover_image_url?: string;
  building_ids?: string[];
  status?: ArticleStatus;
  comments_enabled?: boolean;
  reactions_enabled?: boolean;
}

export interface UpdateArticleRequest {
  title?: string;
  content?: string;
  excerpt?: string;
  cover_image_url?: string;
  building_ids?: string[];
  comments_enabled?: boolean;
  reactions_enabled?: boolean;
}

export interface ArticleListQuery {
  organization_id: string;
  status?: ArticleStatus;
  building_id?: string;
  pinned_only?: boolean;
  limit?: number;
  offset?: number;
}

export interface ArticleStatistics {
  total_articles: number;
  published_articles: number;
  draft_articles: number;
  archived_articles: number;
  total_views: number;
  total_reactions: number;
  total_comments: number;
}

export interface CreateNewsCommentRequest {
  content: string;
  parent_id?: string;
}

export interface ToggleReactionRequest {
  reaction_type: ReactionType;
}

export interface CreateMediaRequest {
  media_type: string;
  file_key?: string;
  file_name?: string;
  file_size?: number;
  mime_type?: string;
  embed_url?: string;
  embed_html?: string;
  width?: number;
  height?: number;
  alt_text?: string;
  caption?: string;
  display_order?: number;
}
