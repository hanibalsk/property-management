# Phase 2 Preparation Documentation

This directory contains architectural documentation and requirements for Phase 2 features that are currently stubbed or partially implemented.

## Stories

| Story | Document | Description |
|-------|----------|-------------|
| 87.1 | [oauth-integration.md](./oauth-integration.md) | OAuth Integration Requirements |
| 87.2 | [ai-assistant.md](./ai-assistant.md) | AI Assistant Architecture |
| 87.3 | [infrastructure-tracing.md](./infrastructure-tracing.md) | Infrastructure Tracing Strategy |

## Current Status

### OAuth (Story 87.1)

**Implemented:**
- OAuth 2.0 Authorization Server (api-server acts as OAuth Provider)
- Authorization Code flow with PKCE support (RFC 7636)
- Client registration, token management, introspection, revocation
- User grant management

**Needs Phase 2 Work:**
- External OAuth consumers (Google, Microsoft, Airbnb) - code exists but tokens are placeholders
- Voice assistant OAuth (Google Assistant, Alexa) - device linking stub
- SSO between api-server and reality-server

### AI Assistant (Story 87.2)

**Implemented:**
- Full API structure for AI chat, sentiment analysis, equipment maintenance, workflows
- LLM client with OpenAI, Anthropic, Azure OpenAI support
- Database schema for chat sessions, messages, feedback
- Lease generation, listing description, photo enhancement endpoints

**Needs Phase 2 Work:**
- Actual LLM API calls (currently returning placeholder responses)
- RAG implementation with document embeddings (pgvector)
- Workflow execution engine
- Photo enhancement service integration

### Infrastructure Tracing (Story 87.3)

**Implemented:**
- Basic `tracing` crate logging throughout the codebase
- Error logging patterns

**Needs Phase 2 Work:**
- OpenTelemetry integration
- Distributed tracing with span propagation
- Jaeger/Zipkin exporter configuration
- Metrics collection and dashboards
