//! Voice command processing service (Epic 93, Story 93.2).
//!
//! Maps voice intents to API actions and returns voice-friendly responses.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Voice command processor.
#[derive(Clone)]
pub struct VoiceCommandProcessor {
    // Dependencies will be added as the feature is implemented
}

/// Voice command request from assistant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommand {
    /// The intent recognized from voice input
    pub intent: String,
    /// Parsed entities/slots from the voice input
    pub entities: serde_json::Value,
    /// User ID from OAuth token
    pub user_id: Uuid,
    /// Organization ID
    pub organization_id: Uuid,
}

/// Voice command response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResponse {
    /// Text to speak back to the user
    pub speech: String,
    /// Whether to end the session
    pub end_session: bool,
    /// Optional card/display data
    pub card: Option<VoiceCard>,
}

/// Display card for voice platforms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCard {
    pub title: String,
    pub content: String,
}

impl VoiceCommandProcessor {
    /// Create a new voice command processor.
    pub fn new() -> Self {
        Self {}
    }

    /// Process a voice command and return a response.
    pub async fn process(&self, command: VoiceCommand) -> VoiceResponse {
        // TODO: Implement in Epic 93, Story 93.2
        // For now, return a placeholder response
        match command.intent.as_str() {
            "GetBalance" => VoiceResponse {
                speech: "I'm sorry, I can't check your balance right now. This feature is coming soon.".to_string(),
                end_session: true,
                card: None,
            },
            "ReportFault" => VoiceResponse {
                speech: "To report a fault, please describe the issue.".to_string(),
                end_session: false,
                card: None,
            },
            "Help" => VoiceResponse {
                speech: "I can help you check your balance, report faults, or get building information. What would you like to do?".to_string(),
                end_session: false,
                card: Some(VoiceCard {
                    title: "Available Commands".to_string(),
                    content: "- Check balance\n- Report a fault\n- Building info".to_string(),
                }),
            },
            _ => VoiceResponse {
                speech: "I'm sorry, I didn't understand that. Try asking for help.".to_string(),
                end_session: false,
                card: None,
            },
        }
    }
}

impl Default for VoiceCommandProcessor {
    fn default() -> Self {
        Self::new()
    }
}
