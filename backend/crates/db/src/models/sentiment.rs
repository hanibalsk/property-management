//! Sentiment analysis models (Epic 13, Story 13.2).

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Daily sentiment trend for a building or organization.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SentimentTrend {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub date: NaiveDate,
    pub avg_sentiment: f64,
    pub message_count: i32,
    pub negative_count: i32,
    pub neutral_count: i32,
    pub positive_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Alert when sentiment threshold is breached.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SentimentAlert {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub alert_type: String,
    pub threshold_breached: f64,
    pub current_sentiment: f64,
    pub previous_sentiment: Option<f64>,
    pub sample_message_ids: Vec<Uuid>,
    pub acknowledged: bool,
    pub acknowledged_by: Option<Uuid>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Organization sentiment thresholds configuration.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SentimentThresholds {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub negative_threshold: f64,
    pub alert_on_spike: bool,
    pub spike_threshold_change: f64,
    pub min_messages_for_alert: i32,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Alert type constants.
pub mod alert_type {
    pub const SPIKE_NEGATIVE: &str = "spike_negative";
    pub const SUSTAINED_DECLINE: &str = "sustained_decline";
    pub const ANOMALY: &str = "anomaly";
    pub const ALL: &[&str] = &[SPIKE_NEGATIVE, SUSTAINED_DECLINE, ANOMALY];
}

/// Request to update sentiment trend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertSentimentTrend {
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub date: NaiveDate,
    pub avg_sentiment: f64,
    pub message_count: i32,
    pub negative_count: i32,
    pub neutral_count: i32,
    pub positive_count: i32,
}

/// Request to create a sentiment alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSentimentAlert {
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub alert_type: String,
    pub threshold_breached: f64,
    pub current_sentiment: f64,
    pub previous_sentiment: Option<f64>,
    pub sample_message_ids: Vec<Uuid>,
}

/// Request to update sentiment thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSentimentThresholds {
    pub negative_threshold: Option<f64>,
    pub alert_on_spike: Option<bool>,
    pub spike_threshold_change: Option<f64>,
    pub min_messages_for_alert: Option<i32>,
    pub enabled: Option<bool>,
}

/// Sentiment dashboard data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentDashboard {
    pub organization_avg: f64,
    pub building_sentiments: Vec<BuildingSentiment>,
    pub trends: Vec<SentimentTrend>,
    pub recent_alerts: Vec<SentimentAlert>,
}

/// Building-level sentiment summary.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BuildingSentiment {
    pub building_id: Uuid,
    pub building_name: String,
    pub avg_sentiment: f64,
    pub message_count: i32,
    pub trend: String, // "improving", "declining", "stable"
}

/// Query parameters for sentiment trends.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SentimentTrendQuery {
    pub building_id: Option<Uuid>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub limit: Option<i64>,
}
