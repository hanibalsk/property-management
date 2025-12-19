namespace PropertyManagement.Shared;

@doc("Pagination query parameters")
model PaginationQuery {
  @doc("Page number (1-indexed)")
  @query
  @minValue(1)
  page?: int32 = 1;

  @doc("Items per page")
  @query
  @minValue(1)
  @maxValue(100)
  limit?: int32 = 20;

  @doc("Sort field")
  @query
  sortBy?: string;

  @doc("Sort direction")
  @query
  sortOrder?: "asc" | "desc" = "asc";
}

@doc("Pagination metadata")
model PaginationMeta {
  @doc("Current page number")
  page: int32;

  @doc("Items per page")
  limit: int32;

  @doc("Total number of items")
  totalItems: int64;

  @doc("Total number of pages")
  totalPages: int32;

  @doc("Has next page")
  hasNext: boolean;

  @doc("Has previous page")
  hasPrevious: boolean;
}

@doc("Paginated response wrapper")
model PaginatedResponse<T> {
  @doc("Array of items")
  data: T[];

  @doc("Pagination metadata")
  pagination: PaginationMeta;
}

@doc("Search query parameters")
model SearchQuery {
  @doc("Search term")
  @query
  q?: string;

  @doc("Filter by status")
  @query
  status?: string;

  @doc("Filter by date from (ISO 8601)")
  @query
  from?: dateTime;

  @doc("Filter by date to (ISO 8601)")
  @query
  to?: dateTime;
}

@doc("Cursor-based pagination query")
model CursorPaginationQuery {
  @doc("Cursor for next page")
  @query
  cursor?: string;

  @doc("Items per page")
  @query
  @minValue(1)
  @maxValue(100)
  limit?: int32 = 20;
}

@doc("Cursor-based pagination metadata")
model CursorPaginationMeta {
  @doc("Cursor for next page (null if no more items)")
  nextCursor?: string;

  @doc("Has more items")
  hasMore: boolean;
}
