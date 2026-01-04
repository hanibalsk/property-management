//! Feature flag CLI commands (Epic 110, Story 110.4).
//!
//! CLI tool for managing feature flags from the command line.
//! Supports listing, enabling/disabling, importing, and exporting features.
//!
//! # Usage
//!
//! ```bash
//! # List all feature flags
//! ppt-features list
//! ppt-features list --format json
//!
//! # Enable/disable features
//! ppt-features enable ai_suggestions
//! ppt-features enable ai_suggestions --org <org-id>
//! ppt-features disable beta_features --user <user-id>
//!
//! # Show feature details
//! ppt-features show ai_suggestions
//!
//! # Export/import configuration
//! ppt-features export --output features.yaml
//! ppt-features import features.yaml --dry-run
//! ```

use clap::{Parser, Subcommand};
use db::models::platform_admin::FeatureFlagScope;
use db::repositories::{FeatureFlagRepository, FeatureFlagWithCount, ResolvedFeatureFlag};
use std::io::Write;
use uuid::Uuid;

/// Feature Flag Management CLI
#[derive(Parser)]
#[command(name = "ppt-features")]
#[command(author, version, about = "Feature flag management for Property Management System", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all feature flags
    List {
        /// Output format: table (default) or json
        #[arg(long, default_value = "table")]
        format: String,
    },

    /// Enable a feature flag
    Enable {
        /// Feature flag key
        key: String,

        /// Organization ID (for org-level override)
        #[arg(long)]
        org: Option<Uuid>,

        /// User ID (for user-level override)
        #[arg(long)]
        user: Option<Uuid>,

        /// Role ID (for role-level override)
        #[arg(long)]
        role: Option<Uuid>,
    },

    /// Disable a feature flag
    Disable {
        /// Feature flag key
        key: String,

        /// Organization ID (for org-level override)
        #[arg(long)]
        org: Option<Uuid>,

        /// User ID (for user-level override)
        #[arg(long)]
        user: Option<Uuid>,

        /// Role ID (for role-level override)
        #[arg(long)]
        role: Option<Uuid>,
    },

    /// Export feature configuration to YAML
    Export {
        /// Output file path
        #[arg(long, default_value = "features.yaml")]
        output: String,
    },

    /// Import feature configuration from YAML
    Import {
        /// Input file path
        file: String,

        /// Dry run (don't apply changes)
        #[arg(long)]
        dry_run: bool,
    },

    /// Show feature flag details
    Show {
        /// Feature flag key
        key: String,
    },

    /// Resolve features for a specific context
    Resolve {
        /// User ID
        #[arg(long)]
        user: Option<Uuid>,

        /// Organization ID
        #[arg(long)]
        org: Option<Uuid>,

        /// Role ID
        #[arg(long)]
        role: Option<Uuid>,

        /// Output format: table (default) or json
        #[arg(long, default_value = "table")]
        format: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env if present
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    // Get database URL
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        eprintln!("Warning: DATABASE_URL not set, using default");
        "postgres://postgres:postgres@localhost:5432/ppt".to_string()
    });

    // Connect to database with RLS-safe pool
    let pool = db::create_rls_safe_pool(&database_url).await?;
    let repo = FeatureFlagRepository::new(pool);

    match cli.command {
        Commands::List { format } => {
            list_features(&repo, &format).await?;
        }
        Commands::Enable {
            key,
            org,
            user,
            role,
        } => {
            set_feature(&repo, &key, true, org, user, role).await?;
        }
        Commands::Disable {
            key,
            org,
            user,
            role,
        } => {
            set_feature(&repo, &key, false, org, user, role).await?;
        }
        Commands::Export { output } => {
            export_features(&repo, &output).await?;
        }
        Commands::Import { file, dry_run } => {
            import_features(&repo, &file, dry_run).await?;
        }
        Commands::Show { key } => {
            show_feature(&repo, &key).await?;
        }
        Commands::Resolve {
            user,
            org,
            role,
            format,
        } => {
            resolve_features(&repo, user, org, role, &format).await?;
        }
    }

    Ok(())
}

async fn list_features(repo: &FeatureFlagRepository, format: &str) -> anyhow::Result<()> {
    let flags = repo.list_all().await?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&flags)?);
    } else {
        print_flags_table(&flags);
    }

    Ok(())
}

fn print_flags_table(flags: &[FeatureFlagWithCount]) {
    println!(
        "{:<30} {:<40} {:<10} {:<10}",
        "KEY", "NAME", "ENABLED", "OVERRIDES"
    );
    println!("{}", "-".repeat(90));

    for flag in flags {
        let enabled = if flag.is_enabled { "Yes" } else { "No" };
        println!(
            "{:<30} {:<40} {:<10} {:<10}",
            flag.key, flag.name, enabled, flag.override_count
        );
    }

    println!();
    println!("Total: {} feature flags", flags.len());
}

async fn set_feature(
    repo: &FeatureFlagRepository,
    key: &str,
    enabled: bool,
    org_id: Option<Uuid>,
    user_id: Option<Uuid>,
    role_id: Option<Uuid>,
) -> anyhow::Result<()> {
    // Get flag by key
    let flag = repo
        .get_by_key(key)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Feature flag '{}' not found", key))?;

    // Determine scope
    let (scope_type, scope_id) = if let Some(uid) = user_id {
        (FeatureFlagScope::User, uid)
    } else if let Some(oid) = org_id {
        (FeatureFlagScope::Organization, oid)
    } else if let Some(rid) = role_id {
        (FeatureFlagScope::Role, rid)
    } else {
        // No scope specified, update global default
        repo.update(flag.id, None, None, Some(enabled)).await?;
        let action = if enabled { "enabled" } else { "disabled" };
        println!("Feature '{}' {} globally", key, action);
        return Ok(());
    };

    // Create or update override
    repo.create_override(flag.id, scope_type.clone(), scope_id, enabled)
        .await?;

    let action = if enabled { "enabled" } else { "disabled" };
    println!(
        "Feature '{}' {} for {} {}",
        key,
        action,
        scope_type.as_str(),
        scope_id
    );

    Ok(())
}

async fn export_features(repo: &FeatureFlagRepository, output: &str) -> anyhow::Result<()> {
    let flags = repo.list_all().await?;

    let mut file = std::fs::File::create(output)?;

    writeln!(file, "# Feature Flags Export")?;
    writeln!(file, "# Generated: {}", chrono::Utc::now().to_rfc3339())?;
    writeln!(file)?;
    writeln!(file, "feature_flags:")?;

    for flag in &flags {
        writeln!(file, "  - key: {}", flag.key)?;
        writeln!(file, "    name: \"{}\"", flag.name)?;
        if let Some(ref desc) = flag.description {
            writeln!(file, "    description: \"{}\"", desc)?;
        }
        writeln!(file, "    is_enabled: {}", flag.is_enabled)?;
        writeln!(file)?;
    }

    println!("Exported {} feature flags to {}", flags.len(), output);

    Ok(())
}

async fn import_features(
    repo: &FeatureFlagRepository,
    file: &str,
    dry_run: bool,
) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(file)?;

    // Simple YAML parsing (in production, use serde_yaml)
    let mut created = 0;
    let mut updated = 0;

    println!("Importing feature flags from {}", file);
    if dry_run {
        println!("(Dry run - no changes will be applied)");
    }
    println!();

    // Parse YAML manually for now (basic implementation)
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.starts_with("- key:") {
            let key = line.strip_prefix("- key:").unwrap().trim();

            // Read next lines for name, description, is_enabled
            let mut name = key.to_string();
            let mut description: Option<String> = None;
            let mut is_enabled = false;

            i += 1;
            while i < lines.len() {
                let next_line = lines[i].trim();
                if next_line.starts_with("- key:")
                    || next_line.is_empty()
                        && lines
                            .get(i + 1)
                            .map(|l| l.trim().starts_with("- key:"))
                            .unwrap_or(false)
                {
                    break;
                }

                if next_line.starts_with("name:") {
                    name = next_line
                        .strip_prefix("name:")
                        .unwrap()
                        .trim()
                        .trim_matches('"')
                        .to_string();
                } else if next_line.starts_with("description:") {
                    description = Some(
                        next_line
                            .strip_prefix("description:")
                            .unwrap()
                            .trim()
                            .trim_matches('"')
                            .to_string(),
                    );
                } else if next_line.starts_with("is_enabled:") {
                    is_enabled = next_line.strip_prefix("is_enabled:").unwrap().trim() == "true";
                }

                i += 1;
            }

            println!("  {} - {} (enabled: {})", key, name, is_enabled);

            if !dry_run {
                // Check if flag exists
                match repo.get_by_key(key).await? {
                    Some(existing) => {
                        repo.update(
                            existing.id,
                            Some(&name),
                            description.as_deref(),
                            Some(is_enabled),
                        )
                        .await?;
                        updated += 1;
                    }
                    None => {
                        repo.create(key, &name, description.as_deref(), is_enabled)
                            .await?;
                        created += 1;
                    }
                }
            }
        }

        i += 1;
    }

    println!();
    if dry_run {
        println!(
            "Would create {} and update {} feature flags",
            created, updated
        );
    } else {
        println!("Created {} and updated {} feature flags", created, updated);
    }

    Ok(())
}

async fn show_feature(repo: &FeatureFlagRepository, key: &str) -> anyhow::Result<()> {
    // Get flag by key
    let flag = repo
        .get_by_key(key)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Feature flag '{}' not found", key))?;

    // Get full details with overrides
    let details = repo
        .get_by_id(flag.id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Feature flag '{}' not found", key))?;

    println!("Feature Flag: {}", details.flag.key);
    println!("  Name: {}", details.flag.name);
    if let Some(ref desc) = details.flag.description {
        println!("  Description: {}", desc);
    }
    println!("  Global Enabled: {}", details.flag.is_enabled);
    println!(
        "  Created: {}",
        details.flag.created_at.format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "  Updated: {}",
        details.flag.updated_at.format("%Y-%m-%d %H:%M:%S")
    );
    println!();

    if details.overrides.is_empty() {
        println!("  No overrides configured.");
    } else {
        println!("  Overrides ({}):", details.overrides.len());
        for override_item in &details.overrides {
            let status = if override_item.is_enabled {
                "enabled"
            } else {
                "disabled"
            };
            println!(
                "    - {} {}: {}",
                override_item.scope_type, override_item.scope_id, status
            );
        }
    }

    Ok(())
}

async fn resolve_features(
    repo: &FeatureFlagRepository,
    user_id: Option<Uuid>,
    org_id: Option<Uuid>,
    role_id: Option<Uuid>,
    format: &str,
) -> anyhow::Result<()> {
    let resolved = repo
        .resolve_all_for_context(user_id, org_id, role_id)
        .await?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&resolved)?);
    } else {
        print_resolved_table(&resolved, user_id, org_id, role_id);
    }

    Ok(())
}

fn print_resolved_table(
    resolved: &[ResolvedFeatureFlag],
    user_id: Option<Uuid>,
    org_id: Option<Uuid>,
    role_id: Option<Uuid>,
) {
    println!("Resolved Features for Context:");
    if let Some(uid) = user_id {
        println!("  User: {}", uid);
    }
    if let Some(oid) = org_id {
        println!("  Organization: {}", oid);
    }
    if let Some(rid) = role_id {
        println!("  Role: {}", rid);
    }
    println!();

    println!("{:<40} {:<10}", "KEY", "ENABLED");
    println!("{}", "-".repeat(50));

    let mut enabled_count = 0;
    for flag in resolved {
        let status = if flag.is_enabled {
            enabled_count += 1;
            "Yes"
        } else {
            "No"
        };
        println!("{:<40} {:<10}", flag.key, status);
    }

    println!();
    println!(
        "Total: {} features, {} enabled",
        resolved.len(),
        enabled_count
    );
}
