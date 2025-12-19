namespace PropertyManagement.Shared;

using TypeSpec.Http;

@doc("Standard error response")
model ErrorResponse {
  @doc("Error code for programmatic handling")
  code: string;

  @doc("Human-readable error message")
  message: string;

  @doc("Request ID for support/debugging")
  requestId?: string;

  @doc("Detailed validation errors")
  details?: ValidationError[];

  @doc("ISO 8601 timestamp")
  timestamp: dateTime;
}

@doc("Validation error detail")
model ValidationError {
  @doc("Field path (e.g., 'address.city')")
  field: string;

  @doc("Error message for this field")
  message: string;

  @doc("Error code")
  code: string;
}

// Common error responses
@error
@doc("Bad Request - Invalid input")
model BadRequestError {
  @statusCode statusCode: 400;
  @body body: ErrorResponse;
}

@error
@doc("Unauthorized - Authentication required")
model UnauthorizedError {
  @statusCode statusCode: 401;
  @body body: ErrorResponse;
}

@error
@doc("Forbidden - Insufficient permissions")
model ForbiddenError {
  @statusCode statusCode: 403;
  @body body: ErrorResponse;
}

@error
@doc("Not Found - Resource does not exist")
model NotFoundError {
  @statusCode statusCode: 404;
  @body body: ErrorResponse;
}

@error
@doc("Conflict - Resource already exists or version mismatch")
model ConflictError {
  @statusCode statusCode: 409;
  @body body: ErrorResponse;
}

@error
@doc("Unprocessable Entity - Business rule violation")
model UnprocessableEntityError {
  @statusCode statusCode: 422;
  @body body: ErrorResponse;
}

@error
@doc("Too Many Requests - Rate limit exceeded")
model RateLimitError {
  @statusCode statusCode: 429;
  @header("Retry-After") retryAfter: int32;
  @body body: ErrorResponse;
}

@error
@doc("Internal Server Error")
model InternalServerError {
  @statusCode statusCode: 500;
  @body body: ErrorResponse;
}
