//! Background scheduler service for periodic tasks.
//!
//! Handles scheduled announcements publishing and other background jobs.

use db::repositories::AnnouncementRepository;
use std::time::Duration;
use tokio::time::interval;

/// Scheduler service configuration.
#[derive(Clone)]
pub struct SchedulerConfig {
    /// Interval between scheduler runs (in seconds).
    pub interval_secs: u64,
    /// Whether the scheduler is enabled.
    pub enabled: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            interval_secs: 60, // Check every minute
            enabled: true,
        }
    }
}

/// Background scheduler for periodic tasks.
pub struct Scheduler {
    announcement_repo: AnnouncementRepository,
    config: SchedulerConfig,
}

impl Scheduler {
    /// Create a new scheduler.
    pub fn new(announcement_repo: AnnouncementRepository, config: SchedulerConfig) -> Self {
        Self {
            announcement_repo,
            config,
        }
    }

    /// Start the scheduler background loop.
    ///
    /// This spawns a tokio task that runs indefinitely,
    /// checking for scheduled tasks at the configured interval.
    pub fn start(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if !self.config.enabled {
                tracing::info!("Scheduler disabled, not starting background tasks");
                return;
            }

            tracing::info!(
                "Starting background scheduler with {}s interval",
                self.config.interval_secs
            );

            let mut ticker = interval(Duration::from_secs(self.config.interval_secs));

            loop {
                ticker.tick().await;
                self.run_scheduled_tasks().await;
            }
        })
    }

    /// Run all scheduled tasks.
    async fn run_scheduled_tasks(&self) {
        // Publish scheduled announcements
        if let Err(e) = self.publish_scheduled_announcements().await {
            tracing::error!("Failed to publish scheduled announcements: {}", e);
        }

        // TODO: Add other scheduled tasks here as needed
        // - Close expired votes (Epic 5)
        // - Send reminder notifications
        // - Clean up old sessions
    }

    /// Publish all scheduled announcements that are due.
    async fn publish_scheduled_announcements(&self) -> Result<(), sqlx::Error> {
        let published = self.announcement_repo.publish_scheduled().await?;

        if !published.is_empty() {
            tracing::info!(
                "Published {} scheduled announcement(s): {:?}",
                published.len(),
                published.iter().map(|a| a.id).collect::<Vec<_>>()
            );

            // TODO(Epic-2B): Trigger notifications for each published announcement
            // Integration point with notification service from Epic 2B:
            //
            // for announcement in &published {
            //     notification_service.send_announcement_notification(
            //         &announcement,
            //         NotificationType::AnnouncementPublished,
            //     ).await?;
            // }
            //
            // The notification should:
            // 1. Determine target users based on announcement.target_type and target_ids
            // 2. Send push notifications (mobile), email, and/or in-app notifications
            // 3. Respect user notification preferences
            for announcement in &published {
                tracing::info!(
                    announcement_id = %announcement.id,
                    title = %announcement.title,
                    target_type = %announcement.target_type,
                    "Scheduled announcement published - notification integration pending Epic 2B"
                );
            }
        }

        Ok(())
    }
}
