/**
 * Voice assistant type definitions.
 *
 * Epic 49 - Story 49.2: Voice Assistant Integration
 */

/**
 * Voice command intent types.
 */
export type VoiceIntent =
  | 'report_fault'
  | 'view_announcements'
  | 'cast_vote'
  | 'view_documents'
  | 'check_notifications'
  | 'view_dashboard'
  | 'open_settings'
  | 'unknown';

/**
 * Parsed voice command with intent and parameters.
 */
export interface ParsedVoiceCommand {
  /** Recognized intent */
  intent: VoiceIntent;
  /** Confidence score (0-1) */
  confidence: number;
  /** Extracted parameters */
  params: VoiceCommandParams;
  /** Original transcript */
  transcript: string;
}

/**
 * Parameters extracted from voice commands.
 */
export interface VoiceCommandParams {
  /** Fault category (if reporting fault) */
  faultCategory?: string;
  /** Location description */
  location?: string;
  /** Priority level */
  priority?: 'low' | 'medium' | 'high' | 'urgent';
  /** Building name or ID */
  building?: string;
  /** Document type */
  documentType?: string;
  /** Vote ID or title */
  voteId?: string;
  /** Generic search query */
  query?: string;
}

/**
 * Voice assistant state.
 */
export type VoiceAssistantState = 'idle' | 'listening' | 'processing' | 'speaking' | 'error';

/**
 * Voice recognition result.
 */
export interface VoiceRecognitionResult {
  /** Recognized transcript */
  transcript: string;
  /** Confidence score */
  confidence: number;
  /** Whether this is a final result */
  isFinal: boolean;
}

/**
 * Voice assistant configuration.
 */
export interface VoiceAssistantConfig {
  /** Language for recognition */
  language: string;
  /** Enable continuous listening */
  continuous: boolean;
  /** Enable haptic feedback */
  hapticFeedback: boolean;
  /** Voice confirmation after actions */
  voiceConfirmation: boolean;
}

/**
 * Voice action result.
 */
export interface VoiceActionResult {
  /** Whether action was successful */
  success: boolean;
  /** Action taken */
  action: string;
  /** Navigation target (if navigating) */
  navigation?: string;
  /** Prefilled data for forms */
  prefilledData?: Record<string, string>;
  /** Confirmation message to speak */
  confirmationMessage?: string;
  /** Error message if failed */
  errorMessage?: string;
}

/**
 * Siri shortcut configuration (iOS).
 */
export interface SiriShortcut {
  /** Shortcut identifier */
  id: string;
  /** User-visible title */
  title: string;
  /** Invocation phrase */
  phrase: string;
  /** Activity type */
  activityType: string;
  /** Associated intent */
  intent: VoiceIntent;
}

/**
 * Google Assistant App Action (Android).
 */
export interface AppAction {
  /** Action identifier */
  id: string;
  /** Action name */
  name: string;
  /** Intent filter action */
  intentAction: string;
  /** BII (Built-in Intent) type */
  builtInIntent: string;
  /** Parameter mappings */
  parameters: Record<string, string>;
}
