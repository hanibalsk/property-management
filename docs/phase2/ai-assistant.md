# Story 87.2: AI Assistant Architecture

## Overview

This document outlines the architecture for the AI Assistant features implemented in Epic 13 (AI Assistant & Automation) and Epic 64 (Advanced AI & LLM Capabilities). The infrastructure is fully scaffolded with placeholder responses.

## Current Implementation Status

### API Structure (Complete)

All AI endpoints are implemented and routed:

| Feature | Epic | Routes | Status |
|---------|------|--------|--------|
| AI Chat | 13.1 | `/api/v1/ai/chat/*` | Placeholder responses |
| Sentiment Analysis | 13.2 | `/api/v1/ai/sentiment/*` | Database ready, no ML |
| Equipment/Maintenance | 13.3 | `/api/v1/ai/equipment/*` | CRUD complete |
| Workflows | 13.6-7 | `/api/v1/ai/workflows/*` | CRUD complete, no execution |
| Lease Generation | 64.1 | `/api/v1/ai/llm/lease/*` | Placeholder |
| Listing Descriptions | 64.2 | `/api/v1/ai/llm/listing/*` | Placeholder |
| Enhanced Chat (RAG) | 64.3 | `/api/v1/ai/llm/chat/enhanced` | Placeholder |
| Photo Enhancement | 64.4 | `/api/v1/ai/llm/photos/*` | Stub |
| Voice Assistant | 64.5 | `/api/v1/ai/llm/voice/*` | Device linking only |

**Key Files:**
- `backend/servers/api-server/src/routes/ai.rs` (1824 lines)
- `backend/crates/integrations/src/llm.rs` (LLM client)
- `backend/crates/db/src/repositories/ai_chat.rs`
- `backend/crates/db/src/repositories/llm_document.rs`

### LLM Client (Scaffolded)

```rust
// backend/crates/integrations/src/llm.rs
pub struct LlmClient {
    http_client: Client,
    config: LlmConfig,
}

impl LlmClient {
    pub async fn openai_chat(&self, request: &ChatCompletionRequest) -> Result<...>
    pub async fn anthropic_chat(&self, request: &ChatCompletionRequest) -> Result<...>
    pub async fn azure_openai_chat(&self, request: &ChatCompletionRequest) -> Result<...>
}
```

**Supported Providers:**
- OpenAI (GPT-4, GPT-4o)
- Anthropic (Claude)
- Azure OpenAI

### Database Schema (Complete)

| Table | Purpose | Migration |
|-------|---------|-----------|
| `ai_chat_sessions` | Chat conversation sessions | 00042 |
| `ai_chat_messages` | Individual messages | 00042 |
| `ai_chat_feedback` | User feedback on responses | 00042 |
| `sentiment_analyses` | Sentiment scoring | 00043 |
| `sentiment_alerts` | Negative sentiment alerts | 00043 |
| `sentiment_thresholds` | Per-org thresholds | 00043 |
| `equipment` | Building equipment inventory | 00044 |
| `maintenance_predictions` | AI predictions | 00044 |
| `document_summaries` | LLM-generated summaries | 00045 |
| `smart_search_queries` | NLP search logs | 00046 |
| `workflows` | Automation workflows | 00047 |
| `workflow_executions` | Execution history | 00047 |
| `llm_generation_requests` | LLM request tracking | 00045+ |
| `listing_descriptions` | AI-generated descriptions | 00045+ |
| `photo_enhancements` | Photo processing queue | 00045+ |
| `voice_devices` | Linked voice assistants | 00045+ |

## Phase 2 Requirements

### 1. LLM Integration

**Current State:**
```rust
// routes/ai.rs:271
// TODO: Process with AI and add assistant response
// For now, return a placeholder response
let assistant_msg = state.ai_chat_repo.add_message(
    session_id,
    "assistant",
    "I'm the AI assistant. This is a placeholder response. Real AI integration coming soon!",
    // ...
);
```

**Required Work:**

1. **Connect LlmClient to routes:**
   ```rust
   // Replace placeholder with actual LLM call
   let response = state.llm_client.openai_chat(&ChatCompletionRequest {
       model: "gpt-4o",
       messages: vec![
           ChatMessage::system("You are a property management assistant..."),
           ChatMessage::user(&req.content),
       ],
       // ...
   }).await?;
   ```

2. **Implement context management:**
   - Load previous messages from session
   - Respect token limits (128K for GPT-4o)
   - Truncate or summarize old messages

3. **Add system prompts per tenant:**
   - Custom assistant personality
   - Building-specific knowledge
   - Language preferences

### 2. RAG Implementation (Story 64.3)

**Current State:**
- `document_embeddings` table exists (with `vector` column)
- `pgvector` extension referenced but not enabled
- Enhanced chat returns placeholder context

**Required Work:**

1. **Enable pgvector:**
   ```sql
   CREATE EXTENSION IF NOT EXISTS vector;

   ALTER TABLE document_embeddings
   ADD COLUMN embedding vector(1536);  -- OpenAI embedding size

   CREATE INDEX ON document_embeddings
   USING ivfflat (embedding vector_cosine_ops);
   ```

2. **Implement embedding generation:**
   ```rust
   async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, LlmError> {
       // Call OpenAI embedding API
       // Store in document_embeddings table
   }
   ```

3. **Implement context retrieval:**
   ```rust
   async fn find_relevant_context(
       &self,
       query: &str,
       org_id: Uuid,
       limit: usize
   ) -> Result<Vec<ContextChunk>, LlmError> {
       // Generate query embedding
       // Cosine similarity search
       // Return top-k chunks
   }
   ```

4. **RAG Chat Flow:**
   ```
   User Query → Generate Embedding → Find Similar Docs →
   Build Context → LLM Call with Context → Response
   ```

### 3. Workflow Execution Engine

**Current State:**
- Workflow CRUD is complete
- `trigger_workflow` creates execution record but doesn't run

**Required Work:**

1. **Implement action executor:**
   ```rust
   async fn execute_workflow(
       &self,
       execution_id: Uuid,
       workflow: &Workflow,
       context: serde_json::Value,
   ) -> Result<(), WorkflowError> {
       for action in workflow.actions {
           self.execute_action(execution_id, &action, &context).await?;
       }
   }
   ```

2. **Action types to implement:**
   - `send_email` - Use email service
   - `send_notification` - Push notification
   - `create_fault` - Auto-create fault report
   - `assign_task` - Create assignment
   - `webhook` - External HTTP call
   - `llm_response` - Generate AI response

3. **Background execution:**
   - Use Tokio tasks or job queue
   - Track step completion
   - Handle retries and failures

### 4. Photo Enhancement

**Current State:**
- Endpoint exists, creates database record
- No actual image processing

**Required Work:**

1. **Integration options:**
   - AWS Rekognition for auto-tagging
   - Stability AI for enhancement
   - OpenAI Vision for analysis

2. **Processing pipeline:**
   ```
   Upload → Queue Job → Process → Store Result → Update Record
   ```

3. **Enhancement types:**
   - `brightness` - Auto-adjust lighting
   - `hdr` - HDR effect
   - `remove_objects` - Remove furniture/clutter
   - `virtual_staging` - Add virtual furniture

### 5. Sentiment Analysis

**Current State:**
- Database schema complete
- No ML model integration

**Required Work:**

1. **Analyze messages:**
   ```rust
   async fn analyze_sentiment(&self, text: &str) -> Result<f64, SentimentError> {
       // Option 1: Local model (rust-bert)
       // Option 2: OpenAI with structured output
       // Option 3: AWS Comprehend
   }
   ```

2. **Trigger alerts:**
   - Check sentiment against org thresholds
   - Create alert if below threshold
   - Notify relevant staff

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         API Layer                                │
│  ┌─────────┐  ┌─────────┐  ┌──────────┐  ┌──────────────────┐ │
│  │ AI Chat │  │Sentiment│  │ Workflow │  │ LLM Generation   │ │
│  └────┬────┘  └────┬────┘  └────┬─────┘  └────────┬─────────┘ │
└───────┼────────────┼───────────┼─────────────────┼────────────┘
        │            │           │                 │
        v            v           v                 v
┌─────────────────────────────────────────────────────────────────┐
│                       Service Layer                              │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────────────┐ │
│  │ LlmClient   │  │ SentimentSvc │  │ WorkflowExecutor       │ │
│  │ - OpenAI    │  │ - Analyze    │  │ - Actions              │ │
│  │ - Anthropic │  │ - Alert      │  │ - Background Jobs      │ │
│  │ - Azure     │  └──────────────┘  └────────────────────────┘ │
│  └─────────────┘                                                │
└───────┼─────────────────────────────────────────────────────────┘
        │
        v
┌─────────────────────────────────────────────────────────────────┐
│                       Data Layer                                 │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────────────┐ │
│  │ Chat Repo   │  │ LLM Doc Repo │  │ Workflow Repo          │ │
│  │ - Sessions  │  │ - Embeddings │  │ - Definitions          │ │
│  │ - Messages  │  │ - Summaries  │  │ - Executions           │ │
│  │ - Feedback  │  │ - Templates  │  │ - Steps                │ │
│  └─────────────┘  └──────────────┘  └────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `OPENAI_API_KEY` | OpenAI API key | For GPT models |
| `ANTHROPIC_API_KEY` | Anthropic API key | For Claude |
| `ANTHROPIC_API_VERSION` | API version (default: 2024-10-22) | For Claude |
| `AZURE_OPENAI_ENDPOINT` | Azure OpenAI endpoint | For Azure |
| `AZURE_OPENAI_API_KEY` | Azure OpenAI key | For Azure |
| `AZURE_OPENAI_DEPLOYMENT` | Deployment name | For Azure |

## Cost Considerations

| Model | Input | Output | Use Case |
|-------|-------|--------|----------|
| GPT-4o | $2.50/1M | $10/1M | Complex chat, lease gen |
| GPT-4o-mini | $0.15/1M | $0.60/1M | Simple responses |
| Claude 3.5 Sonnet | $3/1M | $15/1M | Long context |
| text-embedding-3-small | $0.02/1M | - | Document embeddings |

**Cost Control:**
- Cache common responses
- Use cheaper models for simple tasks
- Implement token budgets per org
- Track usage in `llm_generation_requests`

## Testing Requirements

1. **Unit Tests:**
   - Prompt construction
   - Token counting
   - Response parsing

2. **Integration Tests:**
   - Mock LLM responses
   - Database operations
   - Workflow execution

3. **E2E Tests:**
   - Full chat conversation
   - RAG retrieval accuracy
   - Workflow triggers

## References

- [OpenAI API Reference](https://platform.openai.com/docs/api-reference)
- [Anthropic Claude API](https://docs.anthropic.com/claude/reference)
- [pgvector Documentation](https://github.com/pgvector/pgvector)
- [rust-bert](https://github.com/guillaume-be/rust-bert) (for local sentiment)
