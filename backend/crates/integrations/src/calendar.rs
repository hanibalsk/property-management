//! Calendar integration clients (Story 61.1).
//!
//! Supports Google Calendar and Microsoft Outlook via OAuth.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CalendarError {
    #[error("OAuth error: {0}")]
    OAuth(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Token expired")]
    TokenExpired,
    #[error("Invalid provider: {0}")]
    InvalidProvider(String),
}

/// OAuth configuration for calendar providers.
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

/// OAuth tokens received from the provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub token_type: String,
    pub scope: Option<String>,
}

/// Calendar event from external provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalCalendarEvent {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub all_day: bool,
    pub attendees: Vec<EventAttendee>,
    pub recurrence: Option<String>,
    pub updated_at: DateTime<Utc>,
}

/// Event attendee.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttendee {
    pub email: String,
    pub name: Option<String>,
    pub status: AttendeeStatus,
}

/// Attendee response status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttendeeStatus {
    Pending,
    Accepted,
    Declined,
    Tentative,
}

/// Calendar list item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarListItem {
    pub id: String,
    pub name: String,
    pub primary: bool,
    pub access_role: String,
}

/// Sync result from calendar provider.
#[derive(Debug, Clone, Default)]
pub struct SyncResult {
    pub events_created: Vec<ExternalCalendarEvent>,
    pub events_updated: Vec<ExternalCalendarEvent>,
    pub events_deleted: Vec<String>,
    pub sync_token: Option<String>,
}

// ============================================
// Google Calendar Client
// ============================================

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_CALENDAR_API: &str = "https://www.googleapis.com/calendar/v3";

/// Google Calendar API client.
pub struct GoogleCalendarClient {
    client: reqwest::Client,
    config: OAuthConfig,
}

impl GoogleCalendarClient {
    /// Create a new Google Calendar client.
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    /// Get the OAuth authorization URL.
    pub fn get_auth_url(&self, state: &str) -> String {
        let scopes = [
            "https://www.googleapis.com/auth/calendar",
            "https://www.googleapis.com/auth/calendar.events",
        ]
        .join(" ");

        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent&state={}",
            GOOGLE_AUTH_URL,
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&scopes),
            urlencoding::encode(state)
        )
    }

    /// Exchange authorization code for tokens.
    pub async fn exchange_code(&self, code: &str) -> Result<OAuthTokens, CalendarError> {
        #[derive(Serialize)]
        struct TokenRequest<'a> {
            code: &'a str,
            client_id: &'a str,
            client_secret: &'a str,
            redirect_uri: &'a str,
            grant_type: &'a str,
        }

        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: String,
            refresh_token: Option<String>,
            expires_in: Option<i64>,
            token_type: String,
            scope: Option<String>,
        }

        let response = self
            .client
            .post(GOOGLE_TOKEN_URL)
            .form(&TokenRequest {
                code,
                client_id: &self.config.client_id,
                client_secret: &self.config.client_secret,
                redirect_uri: &self.config.redirect_uri,
                grant_type: "authorization_code",
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::OAuth(error));
        }

        let token_response: TokenResponse = response.json().await?;

        let expires_at = token_response
            .expires_in
            .map(|secs| Utc::now() + chrono::Duration::seconds(secs));

        Ok(OAuthTokens {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_at,
            token_type: token_response.token_type,
            scope: token_response.scope,
        })
    }

    /// Refresh the access token.
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthTokens, CalendarError> {
        #[derive(Serialize)]
        struct RefreshRequest<'a> {
            refresh_token: &'a str,
            client_id: &'a str,
            client_secret: &'a str,
            grant_type: &'a str,
        }

        #[derive(Deserialize)]
        struct RefreshResponse {
            access_token: String,
            expires_in: Option<i64>,
            token_type: String,
            scope: Option<String>,
        }

        let response = self
            .client
            .post(GOOGLE_TOKEN_URL)
            .form(&RefreshRequest {
                refresh_token,
                client_id: &self.config.client_id,
                client_secret: &self.config.client_secret,
                grant_type: "refresh_token",
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::OAuth(error));
        }

        let refresh_response: RefreshResponse = response.json().await?;

        let expires_at = refresh_response
            .expires_in
            .map(|secs| Utc::now() + chrono::Duration::seconds(secs));

        Ok(OAuthTokens {
            access_token: refresh_response.access_token,
            refresh_token: Some(refresh_token.to_string()),
            expires_at,
            token_type: refresh_response.token_type,
            scope: refresh_response.scope,
        })
    }

    /// List calendars for the authenticated user.
    pub async fn list_calendars(
        &self,
        access_token: &str,
    ) -> Result<Vec<CalendarListItem>, CalendarError> {
        #[derive(Deserialize)]
        struct CalendarList {
            items: Vec<CalendarEntry>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct CalendarEntry {
            id: String,
            summary: String,
            primary: Option<bool>,
            access_role: String,
        }

        let response = self
            .client
            .get(format!("{}/users/me/calendarList", GOOGLE_CALENDAR_API))
            .bearer_auth(access_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CalendarError::TokenExpired);
        }

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::Api(error));
        }

        let calendar_list: CalendarList = response.json().await?;

        Ok(calendar_list
            .items
            .into_iter()
            .map(|c| CalendarListItem {
                id: c.id,
                name: c.summary,
                primary: c.primary.unwrap_or(false),
                access_role: c.access_role,
            })
            .collect())
    }

    /// Fetch events from a calendar.
    pub async fn fetch_events(
        &self,
        access_token: &str,
        calendar_id: &str,
        time_min: DateTime<Utc>,
        time_max: DateTime<Utc>,
        sync_token: Option<&str>,
    ) -> Result<SyncResult, CalendarError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct EventsList {
            items: Option<Vec<GoogleEvent>>,
            next_sync_token: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct GoogleEvent {
            id: String,
            summary: Option<String>,
            description: Option<String>,
            location: Option<String>,
            start: Option<EventDateTime>,
            end: Option<EventDateTime>,
            attendees: Option<Vec<GoogleAttendee>>,
            recurrence: Option<Vec<String>>,
            updated: Option<String>,
            status: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct EventDateTime {
            date_time: Option<String>,
            date: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct GoogleAttendee {
            email: String,
            display_name: Option<String>,
            response_status: Option<String>,
        }

        let mut url = format!(
            "{}/calendars/{}/events",
            GOOGLE_CALENDAR_API,
            urlencoding::encode(calendar_id)
        );

        if let Some(token) = sync_token {
            url.push_str(&format!("?syncToken={}", urlencoding::encode(token)));
        } else {
            url.push_str(&format!(
                "?timeMin={}&timeMax={}&singleEvents=true&orderBy=startTime",
                urlencoding::encode(&time_min.to_rfc3339()),
                urlencoding::encode(&time_max.to_rfc3339())
            ));
        }

        let response = self
            .client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CalendarError::TokenExpired);
        }

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::Api(error));
        }

        let events_list: EventsList = response.json().await?;

        let mut result = SyncResult {
            sync_token: events_list.next_sync_token,
            ..Default::default()
        };

        for event in events_list.items.unwrap_or_default() {
            if event.status.as_deref() == Some("cancelled") {
                result.events_deleted.push(event.id);
                continue;
            }

            let (start_time, end_time, all_day) = match (&event.start, &event.end) {
                (Some(start), Some(end)) => {
                    if let (Some(start_dt), Some(end_dt)) = (&start.date_time, &end.date_time) {
                        (
                            DateTime::parse_from_rfc3339(start_dt)
                                .map(|dt| dt.with_timezone(&Utc))
                                .unwrap_or_else(|_| Utc::now()),
                            DateTime::parse_from_rfc3339(end_dt)
                                .map(|dt| dt.with_timezone(&Utc))
                                .unwrap_or_else(|_| Utc::now()),
                            false,
                        )
                    } else if let (Some(start_d), Some(end_d)) = (&start.date, &end.date) {
                        (
                            chrono::NaiveDate::parse_from_str(start_d, "%Y-%m-%d")
                                .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc())
                                .unwrap_or_else(|_| Utc::now()),
                            chrono::NaiveDate::parse_from_str(end_d, "%Y-%m-%d")
                                .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc())
                                .unwrap_or_else(|_| Utc::now()),
                            true,
                        )
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };

            let attendees = event
                .attendees
                .unwrap_or_default()
                .into_iter()
                .map(|a| EventAttendee {
                    email: a.email,
                    name: a.display_name,
                    status: match a.response_status.as_deref() {
                        Some("accepted") => AttendeeStatus::Accepted,
                        Some("declined") => AttendeeStatus::Declined,
                        Some("tentative") => AttendeeStatus::Tentative,
                        _ => AttendeeStatus::Pending,
                    },
                })
                .collect();

            let external_event = ExternalCalendarEvent {
                id: event.id,
                title: event.summary.unwrap_or_else(|| "(No title)".to_string()),
                description: event.description,
                location: event.location,
                start_time,
                end_time,
                all_day,
                attendees,
                recurrence: event.recurrence.map(|r| r.join(";")),
                updated_at: event
                    .updated
                    .and_then(|u| DateTime::parse_from_rfc3339(&u).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
            };

            // For incremental sync, all events are considered updated
            if sync_token.is_some() {
                result.events_updated.push(external_event);
            } else {
                result.events_created.push(external_event);
            }
        }

        Ok(result)
    }

    /// Create an event in the calendar.
    pub async fn create_event(
        &self,
        access_token: &str,
        calendar_id: &str,
        event: &ExternalCalendarEvent,
    ) -> Result<String, CalendarError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct CreateEventRequest<'a> {
            summary: &'a str,
            description: Option<&'a str>,
            location: Option<&'a str>,
            start: EventDateTime,
            end: EventDateTime,
            attendees: Option<Vec<Attendee<'a>>>,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct EventDateTime {
            date_time: Option<String>,
            date: Option<String>,
            time_zone: Option<String>,
        }

        #[derive(Serialize)]
        struct Attendee<'a> {
            email: &'a str,
        }

        #[derive(Deserialize)]
        struct CreateEventResponse {
            id: String,
        }

        let (start, end) = if event.all_day {
            (
                EventDateTime {
                    date_time: None,
                    date: Some(event.start_time.format("%Y-%m-%d").to_string()),
                    time_zone: None,
                },
                EventDateTime {
                    date_time: None,
                    date: Some(event.end_time.format("%Y-%m-%d").to_string()),
                    time_zone: None,
                },
            )
        } else {
            (
                EventDateTime {
                    date_time: Some(event.start_time.to_rfc3339()),
                    date: None,
                    time_zone: Some("UTC".to_string()),
                },
                EventDateTime {
                    date_time: Some(event.end_time.to_rfc3339()),
                    date: None,
                    time_zone: Some("UTC".to_string()),
                },
            )
        };

        let attendees: Option<Vec<Attendee>> = if event.attendees.is_empty() {
            None
        } else {
            Some(
                event
                    .attendees
                    .iter()
                    .map(|a| Attendee { email: &a.email })
                    .collect(),
            )
        };

        let request = CreateEventRequest {
            summary: &event.title,
            description: event.description.as_deref(),
            location: event.location.as_deref(),
            start,
            end,
            attendees,
        };

        let response = self
            .client
            .post(format!(
                "{}/calendars/{}/events",
                GOOGLE_CALENDAR_API,
                urlencoding::encode(calendar_id)
            ))
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CalendarError::TokenExpired);
        }

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::Api(error));
        }

        let event_response: CreateEventResponse = response.json().await?;
        Ok(event_response.id)
    }

    /// Delete an event from the calendar.
    pub async fn delete_event(
        &self,
        access_token: &str,
        calendar_id: &str,
        event_id: &str,
    ) -> Result<(), CalendarError> {
        let response = self
            .client
            .delete(format!(
                "{}/calendars/{}/events/{}",
                GOOGLE_CALENDAR_API,
                urlencoding::encode(calendar_id),
                urlencoding::encode(event_id)
            ))
            .bearer_auth(access_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CalendarError::TokenExpired);
        }

        if !response.status().is_success() && response.status() != reqwest::StatusCode::NOT_FOUND {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::Api(error));
        }

        Ok(())
    }
}

// ============================================
// Microsoft Graph (Outlook) Client
// ============================================

const MICROSOFT_AUTH_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize";
const MICROSOFT_TOKEN_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";
const MICROSOFT_GRAPH_API: &str = "https://graph.microsoft.com/v1.0";

/// Microsoft Graph (Outlook) Calendar API client.
pub struct MicrosoftCalendarClient {
    client: reqwest::Client,
    config: OAuthConfig,
}

impl MicrosoftCalendarClient {
    /// Create a new Microsoft Calendar client.
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    /// Get the OAuth authorization URL.
    pub fn get_auth_url(&self, state: &str) -> String {
        let scopes = ["Calendars.ReadWrite", "offline_access", "User.Read"].join(" ");

        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            MICROSOFT_AUTH_URL,
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.redirect_uri),
            urlencoding::encode(&scopes),
            urlencoding::encode(state)
        )
    }

    /// Exchange authorization code for tokens.
    pub async fn exchange_code(&self, code: &str) -> Result<OAuthTokens, CalendarError> {
        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: String,
            refresh_token: Option<String>,
            expires_in: Option<i64>,
            token_type: String,
            scope: Option<String>,
        }

        let response = self
            .client
            .post(MICROSOFT_TOKEN_URL)
            .form(&[
                ("code", code),
                ("client_id", &self.config.client_id),
                ("client_secret", &self.config.client_secret),
                ("redirect_uri", &self.config.redirect_uri),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::OAuth(error));
        }

        let token_response: TokenResponse = response.json().await?;

        let expires_at = token_response
            .expires_in
            .map(|secs| Utc::now() + chrono::Duration::seconds(secs));

        Ok(OAuthTokens {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_at,
            token_type: token_response.token_type,
            scope: token_response.scope,
        })
    }

    /// Refresh the access token.
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthTokens, CalendarError> {
        #[derive(Deserialize)]
        struct RefreshResponse {
            access_token: String,
            refresh_token: Option<String>,
            expires_in: Option<i64>,
            token_type: String,
            scope: Option<String>,
        }

        let response = self
            .client
            .post(MICROSOFT_TOKEN_URL)
            .form(&[
                ("refresh_token", refresh_token),
                ("client_id", &self.config.client_id),
                ("client_secret", &self.config.client_secret),
                ("grant_type", "refresh_token"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::OAuth(error));
        }

        let refresh_response: RefreshResponse = response.json().await?;

        let expires_at = refresh_response
            .expires_in
            .map(|secs| Utc::now() + chrono::Duration::seconds(secs));

        Ok(OAuthTokens {
            access_token: refresh_response.access_token,
            refresh_token: refresh_response
                .refresh_token
                .or_else(|| Some(refresh_token.to_string())),
            expires_at,
            token_type: refresh_response.token_type,
            scope: refresh_response.scope,
        })
    }

    /// List calendars for the authenticated user.
    pub async fn list_calendars(
        &self,
        access_token: &str,
    ) -> Result<Vec<CalendarListItem>, CalendarError> {
        #[derive(Deserialize)]
        struct CalendarList {
            value: Vec<OutlookCalendar>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct OutlookCalendar {
            id: String,
            name: String,
            is_default_calendar: Option<bool>,
            can_edit: bool,
        }

        let response = self
            .client
            .get(format!("{}/me/calendars", MICROSOFT_GRAPH_API))
            .bearer_auth(access_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CalendarError::TokenExpired);
        }

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::Api(error));
        }

        let calendar_list: CalendarList = response.json().await?;

        Ok(calendar_list
            .value
            .into_iter()
            .map(|c| CalendarListItem {
                id: c.id,
                name: c.name,
                primary: c.is_default_calendar.unwrap_or(false),
                access_role: if c.can_edit {
                    "writer".to_string()
                } else {
                    "reader".to_string()
                },
            })
            .collect())
    }

    /// Fetch events from a calendar.
    pub async fn fetch_events(
        &self,
        access_token: &str,
        calendar_id: &str,
        time_min: DateTime<Utc>,
        time_max: DateTime<Utc>,
        _delta_link: Option<&str>,
    ) -> Result<SyncResult, CalendarError> {
        #[derive(Deserialize)]
        struct EventsList {
            value: Vec<OutlookEvent>,
            #[serde(rename = "@odata.deltaLink")]
            delta_link: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct OutlookEvent {
            id: String,
            subject: Option<String>,
            body_preview: Option<String>,
            location: Option<OutlookLocation>,
            start: OutlookDateTime,
            end: OutlookDateTime,
            is_all_day: Option<bool>,
            attendees: Option<Vec<OutlookAttendee>>,
            recurrence: Option<serde_json::Value>,
            last_modified_date_time: Option<String>,
            is_cancelled: Option<bool>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct OutlookLocation {
            display_name: Option<String>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct OutlookDateTime {
            date_time: String,
            #[allow(dead_code)]
            time_zone: String,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct OutlookAttendee {
            email_address: EmailAddress,
            status: Option<AttendeeResponseStatus>,
        }

        #[derive(Deserialize)]
        struct EmailAddress {
            address: String,
            name: Option<String>,
        }

        #[derive(Deserialize)]
        struct AttendeeResponseStatus {
            response: Option<String>,
        }

        let url = format!(
            "{}/me/calendars/{}/calendarView?startDateTime={}&endDateTime={}",
            MICROSOFT_GRAPH_API,
            urlencoding::encode(calendar_id),
            urlencoding::encode(&time_min.to_rfc3339()),
            urlencoding::encode(&time_max.to_rfc3339())
        );

        let response = self
            .client
            .get(&url)
            .bearer_auth(access_token)
            .header("Prefer", "odata.maxpagesize=100")
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CalendarError::TokenExpired);
        }

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::Api(error));
        }

        let events_list: EventsList = response.json().await?;

        let mut result = SyncResult {
            sync_token: events_list.delta_link,
            ..Default::default()
        };

        for event in events_list.value {
            if event.is_cancelled.unwrap_or(false) {
                result.events_deleted.push(event.id);
                continue;
            }

            let parse_outlook_datetime = |dt: &OutlookDateTime| -> DateTime<Utc> {
                // Outlook returns datetime without timezone offset in the string
                // The timezone is provided separately
                chrono::NaiveDateTime::parse_from_str(&dt.date_time, "%Y-%m-%dT%H:%M:%S%.f")
                    .or_else(|_| {
                        chrono::NaiveDateTime::parse_from_str(&dt.date_time, "%Y-%m-%dT%H:%M:%S")
                    })
                    .map(|ndt| ndt.and_utc())
                    .unwrap_or_else(|_| Utc::now())
            };

            let attendees = event
                .attendees
                .unwrap_or_default()
                .into_iter()
                .map(|a| EventAttendee {
                    email: a.email_address.address,
                    name: a.email_address.name,
                    status: match a.status.and_then(|s| s.response).as_deref() {
                        Some("accepted") => AttendeeStatus::Accepted,
                        Some("declined") => AttendeeStatus::Declined,
                        Some("tentativelyAccepted") => AttendeeStatus::Tentative,
                        _ => AttendeeStatus::Pending,
                    },
                })
                .collect();

            let external_event = ExternalCalendarEvent {
                id: event.id,
                title: event.subject.unwrap_or_else(|| "(No subject)".to_string()),
                description: event.body_preview,
                location: event.location.and_then(|l| l.display_name),
                start_time: parse_outlook_datetime(&event.start),
                end_time: parse_outlook_datetime(&event.end),
                all_day: event.is_all_day.unwrap_or(false),
                attendees,
                recurrence: event.recurrence.map(|r| r.to_string()),
                updated_at: event
                    .last_modified_date_time
                    .and_then(|u| DateTime::parse_from_rfc3339(&u).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
            };

            result.events_created.push(external_event);
        }

        Ok(result)
    }

    /// Create an event in the calendar.
    pub async fn create_event(
        &self,
        access_token: &str,
        calendar_id: &str,
        event: &ExternalCalendarEvent,
    ) -> Result<String, CalendarError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct CreateEventRequest<'a> {
            subject: &'a str,
            body: Option<Body<'a>>,
            location: Option<Location<'a>>,
            start: DateTimeZone,
            end: DateTimeZone,
            is_all_day: bool,
            attendees: Vec<Attendee<'a>>,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Body<'a> {
            content_type: &'a str,
            content: &'a str,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Location<'a> {
            display_name: &'a str,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct DateTimeZone {
            date_time: String,
            time_zone: String,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Attendee<'a> {
            email_address: EmailAddress<'a>,
            r#type: &'a str,
        }

        #[derive(Serialize)]
        struct EmailAddress<'a> {
            address: &'a str,
        }

        #[derive(Deserialize)]
        struct CreateEventResponse {
            id: String,
        }

        let request = CreateEventRequest {
            subject: &event.title,
            body: event.description.as_ref().map(|d| Body {
                content_type: "text",
                content: d,
            }),
            location: event
                .location
                .as_ref()
                .map(|l| Location { display_name: l }),
            start: DateTimeZone {
                date_time: event.start_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
                time_zone: "UTC".to_string(),
            },
            end: DateTimeZone {
                date_time: event.end_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
                time_zone: "UTC".to_string(),
            },
            is_all_day: event.all_day,
            attendees: event
                .attendees
                .iter()
                .map(|a| Attendee {
                    email_address: EmailAddress { address: &a.email },
                    r#type: "required",
                })
                .collect(),
        };

        let response = self
            .client
            .post(format!(
                "{}/me/calendars/{}/events",
                MICROSOFT_GRAPH_API,
                urlencoding::encode(calendar_id)
            ))
            .bearer_auth(access_token)
            .json(&request)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CalendarError::TokenExpired);
        }

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::Api(error));
        }

        let event_response: CreateEventResponse = response.json().await?;
        Ok(event_response.id)
    }

    /// Delete an event from the calendar.
    pub async fn delete_event(
        &self,
        access_token: &str,
        calendar_id: &str,
        event_id: &str,
    ) -> Result<(), CalendarError> {
        let response = self
            .client
            .delete(format!(
                "{}/me/calendars/{}/events/{}",
                MICROSOFT_GRAPH_API,
                urlencoding::encode(calendar_id),
                urlencoding::encode(event_id)
            ))
            .bearer_auth(access_token)
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(CalendarError::TokenExpired);
        }

        if !response.status().is_success() && response.status() != reqwest::StatusCode::NOT_FOUND {
            let error = response.text().await.unwrap_or_default();
            return Err(CalendarError::Api(error));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_auth_url() {
        let client = GoogleCalendarClient::new(OAuthConfig {
            client_id: "test_client_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:8080/callback".to_string(),
        });

        let auth_url = client.get_auth_url("test_state");
        assert!(auth_url.contains("accounts.google.com"));
        assert!(auth_url.contains("test_client_id"));
        assert!(auth_url.contains("test_state"));
    }

    #[test]
    fn test_microsoft_auth_url() {
        let client = MicrosoftCalendarClient::new(OAuthConfig {
            client_id: "test_client_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:8080/callback".to_string(),
        });

        let auth_url = client.get_auth_url("test_state");
        assert!(auth_url.contains("login.microsoftonline.com"));
        assert!(auth_url.contains("test_client_id"));
        assert!(auth_url.contains("test_state"));
    }
}
