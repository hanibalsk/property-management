//! Epic 143: Board Meeting Management Models
//!
//! This module provides models for HOA/Condo board meeting management including:
//! - Board member management and roles
//! - Meeting scheduling and configuration
//! - Agenda item management
//! - Motion handling and voting
//! - Attendance tracking
//! - Meeting minutes with approval workflow
//! - Action item tracking

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

// ============================================================================
// ENUM TYPES
// ============================================================================

/// Types of board meetings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "meeting_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MeetingType {
    Regular,
    Special,
    Annual,
    Emergency,
    Committee,
    ExecutiveSession,
}

/// Status of a board meeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "meeting_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MeetingStatus {
    Draft,
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    Postponed,
}

/// Status of an agenda item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "agenda_item_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AgendaItemStatus {
    Pending,
    InDiscussion,
    Tabled,
    Completed,
    Withdrawn,
}

/// Status of a motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "motion_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MotionStatus {
    Proposed,
    Seconded,
    Discussion,
    Voting,
    Passed,
    Failed,
    Tabled,
    Withdrawn,
    Amended,
}

/// Role of a board member.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "board_role", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BoardRole {
    President,
    VicePresident,
    Secretary,
    Treasurer,
    Director,
    CommitteeChair,
    MemberAtLarge,
}

/// Attendance status for a meeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "attendance_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AttendanceStatus {
    Present,
    Absent,
    Excused,
    Late,
    Remote,
}

/// Vote choice for a motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "vote_choice", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VoteChoice {
    InFavor,
    Opposed,
    Abstain,
    Recused,
}

// ============================================================================
// ENTITY MODELS
// ============================================================================

/// A board member record.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BoardMember {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub role: BoardRole,
    pub title: Option<String>,
    pub term_start: NaiveDate,
    pub term_end: Option<NaiveDate>,
    pub is_active: bool,
    pub email_notifications: bool,
    pub sms_notifications: bool,
    pub appointed_by: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A board meeting.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BoardMeeting {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub building_id: Option<Uuid>,
    pub meeting_number: Option<String>,
    pub title: String,
    pub meeting_type: MeetingType,
    pub status: MeetingStatus,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub location_type: Option<String>,
    pub physical_location: Option<String>,
    pub virtual_meeting_url: Option<String>,
    pub virtual_meeting_id: Option<String>,
    pub dial_in_number: Option<String>,
    pub quorum_required: Option<i32>,
    pub quorum_present: Option<i32>,
    pub quorum_met: bool,
    pub is_recorded: bool,
    pub recording_url: Option<String>,
    pub agenda_document_id: Option<Uuid>,
    pub minutes_document_id: Option<Uuid>,
    pub notice_sent_at: Option<DateTime<Utc>>,
    pub reminder_sent_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub secretary_id: Option<Uuid>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A meeting agenda item.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeetingAgendaItem {
    pub id: Uuid,
    pub meeting_id: Uuid,
    pub item_number: String,
    pub title: String,
    pub description: Option<String>,
    pub item_type: Option<String>,
    pub display_order: i32,
    pub estimated_duration_minutes: Option<i32>,
    pub actual_duration_minutes: Option<i32>,
    pub status: AgendaItemStatus,
    pub presenter_id: Option<Uuid>,
    pub presenter_name: Option<String>,
    pub document_ids: Option<Vec<Uuid>>,
    pub discussion_notes: Option<String>,
    pub outcome: Option<String>,
    pub follow_up_required: bool,
    pub follow_up_assignee: Option<Uuid>,
    pub follow_up_due_date: Option<NaiveDate>,
    pub added_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A motion raised during a meeting.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeetingMotion {
    pub id: Uuid,
    pub meeting_id: Uuid,
    pub agenda_item_id: Option<Uuid>,
    pub motion_number: Option<String>,
    pub title: String,
    pub motion_text: String,
    pub status: MotionStatus,
    pub proposed_by: Uuid,
    pub seconded_by: Option<Uuid>,
    pub votes_in_favor: i32,
    pub votes_opposed: i32,
    pub votes_abstain: i32,
    pub votes_recused: i32,
    pub voting_started_at: Option<DateTime<Utc>>,
    pub voting_ended_at: Option<DateTime<Utc>>,
    pub original_motion_id: Option<Uuid>,
    pub amendment_text: Option<String>,
    pub resolution_text: Option<String>,
    pub effective_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A vote on a motion.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MotionVote {
    pub id: Uuid,
    pub motion_id: Uuid,
    pub board_member_id: Uuid,
    pub vote: VoteChoice,
    pub recusal_reason: Option<String>,
    pub voted_at: DateTime<Utc>,
}

/// Meeting attendance record.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeetingAttendance {
    pub id: Uuid,
    pub meeting_id: Uuid,
    pub board_member_id: Option<Uuid>,
    pub user_id: Uuid,
    pub status: AttendanceStatus,
    pub attendance_type: Option<String>,
    pub arrived_at: Option<DateTime<Utc>>,
    pub departed_at: Option<DateTime<Utc>>,
    pub guest_name: Option<String>,
    pub guest_affiliation: Option<String>,
    pub notes: Option<String>,
    pub marked_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Meeting minutes.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeetingMinutes {
    pub id: Uuid,
    pub meeting_id: Uuid,
    pub version: i32,
    pub status: Option<String>,
    pub call_to_order: Option<String>,
    pub roll_call: Option<String>,
    pub approval_of_minutes: Option<String>,
    pub reports: Option<String>,
    pub old_business: Option<String>,
    pub new_business: Option<String>,
    pub announcements: Option<String>,
    pub adjournment: Option<String>,
    pub full_content: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub approval_motion_id: Option<Uuid>,
    pub document_id: Option<Uuid>,
    pub prepared_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Meeting action item.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeetingActionItem {
    pub id: Uuid,
    pub meeting_id: Uuid,
    pub agenda_item_id: Option<Uuid>,
    pub motion_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub assigned_to_name: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub notes: Option<String>,
    pub completion_notes: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Meeting document.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeetingDocument {
    pub id: Uuid,
    pub meeting_id: Uuid,
    pub document_type: String,
    pub title: String,
    pub description: Option<String>,
    pub file_id: Option<Uuid>,
    pub file_url: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub is_public: bool,
    pub board_only: bool,
    pub uploaded_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Meeting statistics.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeetingStatistics {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_meetings: i32,
    pub regular_meetings: i32,
    pub special_meetings: i32,
    pub emergency_meetings: i32,
    pub cancelled_meetings: i32,
    pub avg_attendance_rate: Option<Decimal>,
    pub avg_quorum_margin: Option<Decimal>,
    pub total_motions: i32,
    pub motions_passed: i32,
    pub motions_failed: i32,
    pub motions_tabled: i32,
    pub avg_meeting_duration_minutes: Option<i32>,
    pub total_meeting_hours: Option<Decimal>,
    pub total_action_items: i32,
    pub completed_action_items: i32,
    pub overdue_action_items: i32,
    pub calculated_at: DateTime<Utc>,
}

// ============================================================================
// CREATE DTOs
// ============================================================================

/// Create a new board member.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateBoardMember {
    pub user_id: Uuid,
    pub role: BoardRole,
    pub title: Option<String>,
    pub term_start: NaiveDate,
    pub term_end: Option<NaiveDate>,
    pub email_notifications: Option<bool>,
    pub sms_notifications: Option<bool>,
    pub notes: Option<String>,
}

/// Create a new board meeting.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateBoardMeeting {
    pub building_id: Option<Uuid>,
    pub meeting_number: Option<String>,
    pub title: String,
    pub meeting_type: MeetingType,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub location_type: Option<String>,
    pub physical_location: Option<String>,
    pub virtual_meeting_url: Option<String>,
    pub virtual_meeting_id: Option<String>,
    pub dial_in_number: Option<String>,
    pub quorum_required: Option<i32>,
    pub is_recorded: Option<bool>,
    pub secretary_id: Option<Uuid>,
    pub description: Option<String>,
}

/// Create a new agenda item.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateAgendaItem {
    pub meeting_id: Uuid,
    pub item_number: String,
    pub title: String,
    pub description: Option<String>,
    pub item_type: Option<String>,
    pub display_order: Option<i32>,
    pub estimated_duration_minutes: Option<i32>,
    pub presenter_id: Option<Uuid>,
    pub presenter_name: Option<String>,
    pub document_ids: Option<Vec<Uuid>>,
}

/// Create a new motion.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateMotion {
    pub meeting_id: Uuid,
    pub agenda_item_id: Option<Uuid>,
    pub motion_number: Option<String>,
    pub title: String,
    pub motion_text: String,
    pub effective_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

/// Cast a vote on a motion.
#[derive(Debug, Clone, Deserialize)]
pub struct CastVote {
    pub motion_id: Uuid,
    pub vote: VoteChoice,
    pub recusal_reason: Option<String>,
}

/// Record attendance.
#[derive(Debug, Clone, Deserialize)]
pub struct RecordAttendance {
    pub meeting_id: Uuid,
    pub board_member_id: Option<Uuid>,
    pub user_id: Uuid,
    pub status: AttendanceStatus,
    pub attendance_type: Option<String>,
    pub guest_name: Option<String>,
    pub guest_affiliation: Option<String>,
    pub notes: Option<String>,
}

/// Create meeting minutes.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateMinutes {
    pub meeting_id: Uuid,
    pub call_to_order: Option<String>,
    pub roll_call: Option<String>,
    pub approval_of_minutes: Option<String>,
    pub reports: Option<String>,
    pub old_business: Option<String>,
    pub new_business: Option<String>,
    pub announcements: Option<String>,
    pub adjournment: Option<String>,
    pub full_content: Option<String>,
}

/// Create an action item.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateActionItem {
    pub meeting_id: Uuid,
    pub agenda_item_id: Option<Uuid>,
    pub motion_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub assigned_to_name: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub priority: Option<String>,
    pub notes: Option<String>,
}

/// Upload a meeting document.
#[derive(Debug, Clone, Deserialize)]
pub struct UploadMeetingDocument {
    pub meeting_id: Uuid,
    pub document_type: String,
    pub title: String,
    pub description: Option<String>,
    pub file_id: Option<Uuid>,
    pub file_url: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub is_public: Option<bool>,
    pub board_only: Option<bool>,
}

// ============================================================================
// UPDATE DTOs
// ============================================================================

/// Update a board member.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateBoardMember {
    pub role: Option<BoardRole>,
    pub title: Option<String>,
    pub term_end: Option<NaiveDate>,
    pub is_active: Option<bool>,
    pub email_notifications: Option<bool>,
    pub sms_notifications: Option<bool>,
    pub notes: Option<String>,
}

/// Update a board meeting.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateBoardMeeting {
    pub meeting_number: Option<String>,
    pub title: Option<String>,
    pub meeting_type: Option<MeetingType>,
    pub status: Option<MeetingStatus>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub location_type: Option<String>,
    pub physical_location: Option<String>,
    pub virtual_meeting_url: Option<String>,
    pub virtual_meeting_id: Option<String>,
    pub dial_in_number: Option<String>,
    pub quorum_required: Option<i32>,
    pub quorum_present: Option<i32>,
    pub quorum_met: Option<bool>,
    pub is_recorded: Option<bool>,
    pub recording_url: Option<String>,
    pub secretary_id: Option<Uuid>,
    pub description: Option<String>,
    pub notes: Option<String>,
}

/// Update an agenda item.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAgendaItem {
    pub item_number: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub item_type: Option<String>,
    pub display_order: Option<i32>,
    pub estimated_duration_minutes: Option<i32>,
    pub actual_duration_minutes: Option<i32>,
    pub status: Option<AgendaItemStatus>,
    pub presenter_id: Option<Uuid>,
    pub presenter_name: Option<String>,
    pub document_ids: Option<Vec<Uuid>>,
    pub discussion_notes: Option<String>,
    pub outcome: Option<String>,
    pub follow_up_required: Option<bool>,
    pub follow_up_assignee: Option<Uuid>,
    pub follow_up_due_date: Option<NaiveDate>,
}

/// Update a motion.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMotion {
    pub title: Option<String>,
    pub motion_text: Option<String>,
    pub status: Option<MotionStatus>,
    pub resolution_text: Option<String>,
    pub effective_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

/// Update meeting minutes.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMinutes {
    pub status: Option<String>,
    pub call_to_order: Option<String>,
    pub roll_call: Option<String>,
    pub approval_of_minutes: Option<String>,
    pub reports: Option<String>,
    pub old_business: Option<String>,
    pub new_business: Option<String>,
    pub announcements: Option<String>,
    pub adjournment: Option<String>,
    pub full_content: Option<String>,
}

/// Update an action item.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateActionItem {
    pub title: Option<String>,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub assigned_to_name: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub notes: Option<String>,
    pub completion_notes: Option<String>,
}

// ============================================================================
// QUERY DTOs
// ============================================================================

/// Query parameters for listing board members.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BoardMemberQuery {
    pub role: Option<BoardRole>,
    pub is_active: Option<bool>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Query parameters for listing meetings.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MeetingQuery {
    pub building_id: Option<Uuid>,
    pub meeting_type: Option<MeetingType>,
    pub status: Option<MeetingStatus>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Query parameters for listing motions.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MotionQuery {
    pub meeting_id: Option<Uuid>,
    pub status: Option<MotionStatus>,
    pub proposed_by: Option<Uuid>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// Query parameters for action items.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ActionItemQuery {
    pub meeting_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub status: Option<String>,
    pub overdue_only: Option<bool>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ============================================================================
// RESPONSE DTOs
// ============================================================================

/// Board member summary for list views.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct BoardMemberSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: BoardRole,
    pub title: Option<String>,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub term_start: NaiveDate,
    pub term_end: Option<NaiveDate>,
    pub is_active: bool,
}

/// Meeting summary for list views.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct MeetingSummary {
    pub id: Uuid,
    pub meeting_number: Option<String>,
    pub title: String,
    pub meeting_type: MeetingType,
    pub status: MeetingStatus,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub location_type: Option<String>,
    pub quorum_met: bool,
    pub agenda_item_count: Option<i64>,
    pub motion_count: Option<i64>,
}

/// Motion summary for list views.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct MotionSummary {
    pub id: Uuid,
    pub motion_number: Option<String>,
    pub title: String,
    pub status: MotionStatus,
    pub proposed_by_name: Option<String>,
    pub votes_in_favor: i32,
    pub votes_opposed: i32,
    pub votes_abstain: i32,
    pub created_at: DateTime<Utc>,
}

/// Action item summary.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ActionItemSummary {
    pub id: Uuid,
    pub title: String,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assigned_to_name: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub is_overdue: Option<bool>,
}

/// Meeting dashboard data.
#[derive(Debug, Clone, Serialize)]
pub struct MeetingDashboard {
    pub upcoming_meetings: Vec<MeetingSummary>,
    pub recent_meetings: Vec<MeetingSummary>,
    pub open_action_items: Vec<ActionItemSummary>,
    pub pending_minutes: i64,
    pub board_member_count: i64,
    pub statistics: Option<MeetingStatistics>,
}

/// Meeting detail with all related data.
#[derive(Debug, Clone, Serialize)]
pub struct MeetingDetail {
    pub meeting: BoardMeeting,
    pub agenda_items: Vec<MeetingAgendaItem>,
    pub motions: Vec<MeetingMotion>,
    pub attendance: Vec<MeetingAttendance>,
    pub action_items: Vec<MeetingActionItem>,
    pub documents: Vec<MeetingDocument>,
    pub minutes: Option<MeetingMinutes>,
}

/// Motion detail with votes.
#[derive(Debug, Clone, Serialize)]
pub struct MotionDetail {
    pub motion: MeetingMotion,
    pub votes: Vec<MotionVote>,
    pub vote_summary: VoteSummary,
}

/// Vote summary for a motion.
#[derive(Debug, Clone, Serialize)]
pub struct VoteSummary {
    pub total_votes: i32,
    pub in_favor: i32,
    pub opposed: i32,
    pub abstain: i32,
    pub recused: i32,
    pub quorum_met: bool,
    pub passed: bool,
}

/// Board member attendance history.
#[derive(Debug, Clone, Serialize)]
pub struct AttendanceHistory {
    pub board_member: BoardMember,
    pub total_meetings: i64,
    pub attended: i64,
    pub absent: i64,
    pub excused: i64,
    pub remote: i64,
    pub attendance_rate: Decimal,
}

/// Meeting type count for statistics.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct MeetingTypeCount {
    pub meeting_type: MeetingType,
    pub count: i64,
}

/// Motion status count for statistics.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct MotionStatusCount {
    pub status: MotionStatus,
    pub count: i64,
}
