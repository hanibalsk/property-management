//! LLM API client for AI capabilities (Epic 64: Advanced AI & LLM Capabilities).
//!
//! Provides unified interface for:
//! - OpenAI (GPT-4, GPT-4o)
//! - Anthropic (Claude)
//! - Azure OpenAI
//!
//! Features:
//! - Lease agreement generation (Story 64.1)
//! - Property listing descriptions (Story 64.2)
//! - Conversational AI with RAG (Story 64.3)
//! - Photo enhancement coordination (Story 64.4)

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

/// LLM API errors.
#[derive(Error, Debug)]
pub enum LlmError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Rate limited: retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Missing API key for provider: {0}")]
    MissingApiKey(String),

    #[error("Unsupported provider: {0}")]
    UnsupportedProvider(String),

    #[error("Context too long: {tokens} tokens exceeds limit of {limit}")]
    ContextTooLong { tokens: u32, limit: u32 },

    #[error("Content filtered: {reason}")]
    ContentFiltered { reason: String },

    #[error("Timeout")]
    Timeout,
}

/// LLM provider configuration.
#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub openai_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub anthropic_api_version: String,
    pub azure_openai_endpoint: Option<String>,
    pub azure_openai_api_key: Option<String>,
    pub azure_openai_deployment: Option<String>,
    pub default_timeout_secs: u64,
    pub max_retries: u32,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            openai_api_key: std::env::var("OPENAI_API_KEY").ok(),
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
            anthropic_api_version: std::env::var("ANTHROPIC_API_VERSION")
                .unwrap_or_else(|_| "2024-10-22".to_string()),
            azure_openai_endpoint: std::env::var("AZURE_OPENAI_ENDPOINT").ok(),
            azure_openai_api_key: std::env::var("AZURE_OPENAI_API_KEY").ok(),
            azure_openai_deployment: std::env::var("AZURE_OPENAI_DEPLOYMENT").ok(),
            default_timeout_secs: 120,
            max_retries: 3,
        }
    }
}

/// Unified LLM client.
#[derive(Debug, Clone)]
pub struct LlmClient {
    http_client: Client,
    config: LlmConfig,
}

impl LlmClient {
    /// Create a new LLM client with default configuration.
    pub fn new() -> Self {
        Self::with_config(LlmConfig::default())
    }

    /// Create a new LLM client with custom configuration.
    pub fn with_config(config: LlmConfig) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.default_timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http_client,
            config,
        }
    }

    /// Complete a chat using OpenAI API.
    pub async fn openai_chat(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmError> {
        let api_key = self
            .config
            .openai_api_key
            .as_ref()
            .ok_or_else(|| LlmError::MissingApiKey("openai".to_string()))?;

        let response = self
            .http_client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&OpenAiRequest::from(request.clone()))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            if status == 429 {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60);
                return Err(LlmError::RateLimited { retry_after });
            }
            let error_body = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError {
                status,
                message: error_body,
            });
        }

        let openai_response: OpenAiResponse = response.json().await?;
        Ok(openai_response.into())
    }

    /// Complete a chat using Anthropic API.
    pub async fn anthropic_chat(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmError> {
        let api_key = self
            .config
            .anthropic_api_key
            .as_ref()
            .ok_or_else(|| LlmError::MissingApiKey("anthropic".to_string()))?;

        let anthropic_request = AnthropicRequest::from(request.clone());

        let response = self
            .http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", &self.config.anthropic_api_version)
            .header("Content-Type", "application/json")
            .json(&anthropic_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            if status == 429 {
                return Err(LlmError::RateLimited { retry_after: 60 });
            }
            let error_body = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError {
                status,
                message: error_body,
            });
        }

        let anthropic_response: AnthropicResponse = response.json().await?;
        Ok(anthropic_response.into())
    }

    /// Complete a chat using the specified provider.
    pub async fn chat(
        &self,
        provider: &str,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmError> {
        match provider {
            "openai" => self.openai_chat(request).await,
            "anthropic" => self.anthropic_chat(request).await,
            "azure_openai" => self.azure_openai_chat(request).await,
            _ => Err(LlmError::UnsupportedProvider(provider.to_string())),
        }
    }

    /// Complete a chat using Azure OpenAI API.
    pub async fn azure_openai_chat(
        &self,
        request: &ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, LlmError> {
        let endpoint = self
            .config
            .azure_openai_endpoint
            .as_ref()
            .ok_or_else(|| LlmError::MissingApiKey("azure_openai_endpoint".to_string()))?;
        let api_key = self
            .config
            .azure_openai_api_key
            .as_ref()
            .ok_or_else(|| LlmError::MissingApiKey("azure_openai".to_string()))?;
        let deployment = self
            .config
            .azure_openai_deployment
            .as_ref()
            .ok_or_else(|| LlmError::MissingApiKey("azure_openai_deployment".to_string()))?;

        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version=2024-02-15-preview",
            endpoint, deployment
        );

        let response = self
            .http_client
            .post(&url)
            .header("api-key", api_key)
            .header("Content-Type", "application/json")
            .json(&OpenAiRequest::from(request.clone()))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error_body = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError {
                status,
                message: error_body,
            });
        }

        let openai_response: OpenAiResponse = response.json().await?;
        Ok(openai_response.into())
    }

    /// Generate a lease agreement.
    pub async fn generate_lease(
        &self,
        provider: &str,
        model: &str,
        system_prompt: &str,
        lease_details: &LeaseGenerationInput,
        language: &str,
    ) -> Result<LeaseGenerationResult, LlmError> {
        let user_prompt = format!(
            r#"Generate a comprehensive lease agreement in {} language with the following details:

Landlord: {}
Landlord Address: {}
Tenant: {}
Tenant Email: {}
Tenant Phone: {}
Property: Unit {}
Start Date: {}
End Date: {}
Monthly Rent: {} {}
Security Deposit: {} {}
Additional Terms: {}
Include Pet Clause: {}
Include Parking: {}
Jurisdiction: {}

Please generate a legally appropriate lease agreement with all standard clauses for the specified jurisdiction. Format the output as follows:
1. Document HTML (wrapped in <document_html></document_html> tags)
2. Plain text version (wrapped in <document_text></document_text> tags)
3. List of clauses with their titles and whether they are mandatory
4. Any warnings or compliance notes"#,
            language,
            lease_details.landlord_name,
            lease_details.landlord_address.as_deref().unwrap_or("N/A"),
            lease_details.tenant_name,
            lease_details.tenant_email,
            lease_details.tenant_phone.as_deref().unwrap_or("N/A"),
            lease_details.unit_id,
            lease_details.start_date,
            lease_details.end_date,
            lease_details.monthly_rent,
            lease_details.currency,
            lease_details.security_deposit,
            lease_details.currency,
            lease_details
                .additional_terms
                .as_ref()
                .map(|t| t.join(", "))
                .unwrap_or_else(|| "None".to_string()),
            lease_details.include_pet_clause,
            lease_details.include_parking,
            lease_details.jurisdiction.as_deref().unwrap_or("Slovakia"),
        );

        let request = ChatCompletionRequest {
            model: model.to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            temperature: Some(0.3),
            max_tokens: Some(8000),
        };

        let response = self.chat(provider, &request).await?;
        let content = response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        // Parse the response
        let document_html = extract_between(&content, "<document_html>", "</document_html>")
            .unwrap_or_else(|| content.clone());
        let document_text = extract_between(&content, "<document_text>", "</document_text>")
            .unwrap_or_else(|| content.clone());

        Ok(LeaseGenerationResult {
            document_html,
            document_text,
            clauses: vec![],
            warnings: vec![],
            compliance_notes: None,
            tokens_used: response.usage.total_tokens,
        })
    }

    /// Generate a property listing description.
    pub async fn generate_listing_description(
        &self,
        provider: &str,
        model: &str,
        system_prompt: &str,
        input: &ListingDescriptionInput,
    ) -> Result<ListingDescriptionResult, LlmError> {
        let features_str = input.features.join(", ");
        let amenities_str = input
            .nearby_amenities
            .as_ref()
            .map(|a| a.join(", "))
            .unwrap_or_default();

        let user_prompt = format!(
            r#"Generate an attractive property listing description in {} language:

Property Type: {}
Transaction: {} (Price: {} {})
Size: {} sqm
Rooms: {} rooms, {} bathrooms
Floor: {} of {}
Location: {}, {}
Features: {}
Nearby: {}
Style: {}
Max Length: {} words

Generate:
1. An engaging description highlighting the property's best features
2. 3-5 key highlights as bullet points
3. A suggested title
4. SEO keywords (comma-separated)"#,
            input.language,
            input.property_type,
            input.transaction_type,
            input.price,
            input.currency,
            input.size_sqm.unwrap_or(0.0),
            input.rooms.unwrap_or(0),
            input.bathrooms.unwrap_or(0),
            input.floor.unwrap_or(0),
            input.total_floors.unwrap_or(0),
            input.city,
            input.district.as_deref().unwrap_or(""),
            features_str,
            amenities_str,
            input.style.as_deref().unwrap_or("professional"),
            input.max_length.unwrap_or(300),
        );

        let request = ChatCompletionRequest {
            model: model.to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            temperature: Some(0.7),
            max_tokens: Some(2000),
        };

        let response = self.chat(provider, &request).await?;
        let content = response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(ListingDescriptionResult {
            description: content,
            key_highlights: vec![],
            suggested_title: None,
            seo_keywords: None,
            tokens_used: response.usage.total_tokens,
        })
    }

    /// Enhanced chat with context for RAG.
    pub async fn enhanced_chat(
        &self,
        provider: &str,
        model: &str,
        system_prompt: &str,
        user_message: &str,
        context_chunks: &[ContextChunk],
        language: &str,
    ) -> Result<EnhancedChatResult, LlmError> {
        let context_text = if !context_chunks.is_empty() {
            let chunks: Vec<String> = context_chunks
                .iter()
                .map(|c| format!("[Source: {}]\n{}", c.source_title, c.text))
                .collect();
            format!(
                "Relevant context from building documents:\n\n{}\n\n",
                chunks.join("\n---\n")
            )
        } else {
            String::new()
        };

        let enhanced_system = format!(
            "{}\n\nIMPORTANT: Respond in {} language. If you cannot answer confidently (confidence below 80%), indicate that escalation to a human is needed.",
            system_prompt, language
        );

        let user_content = format!("{}{}", context_text, user_message);

        let request = ChatCompletionRequest {
            model: model.to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: enhanced_system,
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_content,
                },
            ],
            temperature: Some(0.5),
            max_tokens: Some(2000),
        };

        let response = self.chat(provider, &request).await?;
        let content = response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        // Simple escalation detection
        let escalated = content.to_lowercase().contains("escalat")
            || content.to_lowercase().contains("human")
            || content.to_lowercase().contains("cannot answer");

        Ok(EnhancedChatResult {
            response: content,
            confidence: if escalated { 0.5 } else { 0.9 },
            escalated,
            escalation_reason: if escalated {
                Some("Low confidence or complex query".to_string())
            } else {
                None
            },
            sources_used: context_chunks.iter().map(|c| c.source_id).collect(),
            tokens_used: response.usage.total_tokens,
        })
    }
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Request/Response Types
// =============================================================================

/// Chat completion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
}

/// Chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<ChatChoice>,
    pub usage: TokenUsage,
}

/// Chat choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: i32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

/// Token usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

// =============================================================================
// OpenAI Types
// =============================================================================

#[derive(Debug, Clone, Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,
}

impl From<ChatCompletionRequest> for OpenAiRequest {
    fn from(req: ChatCompletionRequest) -> Self {
        Self {
            model: req.model,
            messages: req.messages,
            temperature: req.temperature,
            max_tokens: req.max_tokens,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiResponse {
    id: String,
    model: String,
    choices: Vec<OpenAiChoice>,
    usage: OpenAiUsage,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiChoice {
    index: i32,
    message: ChatMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

impl From<OpenAiResponse> for ChatCompletionResponse {
    fn from(resp: OpenAiResponse) -> Self {
        Self {
            id: resp.id,
            model: resp.model,
            choices: resp
                .choices
                .into_iter()
                .map(|c| ChatChoice {
                    index: c.index,
                    message: c.message,
                    finish_reason: c.finish_reason,
                })
                .collect(),
            usage: TokenUsage {
                prompt_tokens: resp.usage.prompt_tokens,
                completion_tokens: resp.usage.completion_tokens,
                total_tokens: resp.usage.total_tokens,
            },
        }
    }
}

// =============================================================================
// Anthropic Types
// =============================================================================

#[derive(Debug, Clone, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    max_tokens: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

impl From<ChatCompletionRequest> for AnthropicRequest {
    fn from(req: ChatCompletionRequest) -> Self {
        let system = req
            .messages
            .iter()
            .find(|m| m.role == "system")
            .map(|m| m.content.clone());

        let messages: Vec<AnthropicMessage> = req
            .messages
            .into_iter()
            .filter(|m| m.role != "system")
            .map(|m| AnthropicMessage {
                role: m.role,
                content: m.content,
            })
            .collect();

        Self {
            model: req.model,
            messages,
            system,
            max_tokens: req.max_tokens.unwrap_or(4096),
            temperature: req.temperature,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct AnthropicResponse {
    id: String,
    model: String,
    content: Vec<AnthropicContent>,
    usage: AnthropicUsage,
    stop_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    content_type: String,
    text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct AnthropicUsage {
    input_tokens: i32,
    output_tokens: i32,
}

impl From<AnthropicResponse> for ChatCompletionResponse {
    fn from(resp: AnthropicResponse) -> Self {
        let content = resp
            .content
            .into_iter()
            .filter_map(|c| c.text)
            .collect::<Vec<_>>()
            .join("");

        Self {
            id: resp.id,
            model: resp.model,
            choices: vec![ChatChoice {
                index: 0,
                message: ChatMessage {
                    role: "assistant".to_string(),
                    content,
                },
                finish_reason: resp.stop_reason,
            }],
            usage: TokenUsage {
                prompt_tokens: resp.usage.input_tokens,
                completion_tokens: resp.usage.output_tokens,
                total_tokens: resp.usage.input_tokens + resp.usage.output_tokens,
            },
        }
    }
}

// =============================================================================
// Domain-Specific Types
// =============================================================================

/// Lease generation input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseGenerationInput {
    pub unit_id: Uuid,
    pub landlord_name: String,
    pub landlord_address: Option<String>,
    pub tenant_name: String,
    pub tenant_email: String,
    pub tenant_phone: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub monthly_rent: f64,
    pub security_deposit: f64,
    pub currency: String,
    pub additional_terms: Option<Vec<String>>,
    pub include_pet_clause: bool,
    pub include_parking: bool,
    pub jurisdiction: Option<String>,
}

/// Lease generation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseGenerationResult {
    pub document_html: String,
    pub document_text: String,
    pub clauses: Vec<LeaseClause>,
    pub warnings: Vec<String>,
    pub compliance_notes: Option<String>,
    pub tokens_used: i32,
}

/// Lease clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseClause {
    pub title: String,
    pub content: String,
    pub is_mandatory: bool,
}

/// Listing description input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingDescriptionInput {
    pub property_type: String,
    pub transaction_type: String,
    pub size_sqm: Option<f64>,
    pub rooms: Option<i32>,
    pub bathrooms: Option<i32>,
    pub floor: Option<i32>,
    pub total_floors: Option<i32>,
    pub features: Vec<String>,
    pub city: String,
    pub district: Option<String>,
    pub nearby_amenities: Option<Vec<String>>,
    pub price: f64,
    pub currency: String,
    pub language: String,
    pub style: Option<String>,
    pub max_length: Option<i32>,
}

/// Listing description result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingDescriptionResult {
    pub description: String,
    pub key_highlights: Vec<String>,
    pub suggested_title: Option<String>,
    pub seo_keywords: Option<Vec<String>>,
    pub tokens_used: i32,
}

/// Context chunk for RAG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextChunk {
    pub source_id: Uuid,
    pub source_title: String,
    pub text: String,
    pub relevance_score: f64,
}

/// Enhanced chat result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedChatResult {
    pub response: String,
    pub confidence: f64,
    pub escalated: bool,
    pub escalation_reason: Option<String>,
    pub sources_used: Vec<Uuid>,
    pub tokens_used: i32,
}

// =============================================================================
// Helper Functions
// =============================================================================

fn extract_between(text: &str, start_tag: &str, end_tag: &str) -> Option<String> {
    let start = text.find(start_tag)?;
    let end = text.find(end_tag)?;
    if start < end {
        Some(text[start + start_tag.len()..end].trim().to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_between() {
        let text = "<doc>Hello World</doc>";
        assert_eq!(
            extract_between(text, "<doc>", "</doc>"),
            Some("Hello World".to_string())
        );

        let text_no_match = "Hello World";
        assert_eq!(extract_between(text_no_match, "<doc>", "</doc>"), None);
    }

    #[test]
    fn test_openai_request_from_chat_request() {
        let request = ChatCompletionRequest {
            model: "gpt-4".to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
            temperature: Some(0.7),
            max_tokens: Some(1000),
        };

        let openai_request = OpenAiRequest::from(request);
        assert_eq!(openai_request.model, "gpt-4");
        assert_eq!(openai_request.messages.len(), 1);
    }

    #[test]
    fn test_anthropic_request_extracts_system() {
        let request = ChatCompletionRequest {
            model: "claude-3-opus".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are helpful".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: "Hello".to_string(),
                },
            ],
            temperature: Some(0.5),
            max_tokens: Some(2000),
        };

        let anthropic_request = AnthropicRequest::from(request);
        assert_eq!(
            anthropic_request.system,
            Some("You are helpful".to_string())
        );
        assert_eq!(anthropic_request.messages.len(), 1);
    }
}
