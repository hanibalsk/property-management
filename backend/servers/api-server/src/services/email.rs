//! Email service (Epic 1, Story 1.7).
//!
//! Supports SMTP email sending in production and logging fallback in development.
//! Configure via environment variables:
//! - SMTP_HOST: SMTP server hostname
//! - SMTP_PORT: SMTP server port (default: 587)
//! - SMTP_USERNAME: SMTP authentication username
//! - SMTP_PASSWORD: SMTP authentication password
//! - SMTP_FROM: Sender email address
//! - SMTP_TLS: Enable TLS (default: true)

use db::models::Locale;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::sync::Arc;
use thiserror::Error;

/// Email service errors.
#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Failed to send email: {0}")]
    SendFailed(String),

    #[error("Invalid email address: {0}")]
    InvalidAddress(String),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("SMTP configuration error: {0}")]
    ConfigurationError(String),

    #[error("Failed to build email message: {0}")]
    MessageBuildError(String),
}

/// SMTP configuration loaded from environment variables.
#[derive(Clone, Debug)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub from_name: String,
    pub use_tls: bool,
}

impl SmtpConfig {
    /// Load SMTP configuration from environment variables.
    pub fn from_env() -> Option<Self> {
        let host = std::env::var("SMTP_HOST").ok()?;
        let port = std::env::var("SMTP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(587);
        let username = std::env::var("SMTP_USERNAME").ok()?;
        let password = std::env::var("SMTP_PASSWORD").ok()?;
        let from_address = std::env::var("SMTP_FROM").ok()?;
        let from_name = std::env::var("SMTP_FROM_NAME")
            .unwrap_or_else(|_| "Property Management System".to_string());
        let use_tls = std::env::var("SMTP_TLS")
            .map(|v| v != "false" && v != "0")
            .unwrap_or(true);

        Some(Self {
            host,
            port,
            username,
            password,
            from_address,
            from_name,
            use_tls,
        })
    }
}

/// Email transport type supporting both SMTP and logging fallback.
enum EmailTransport {
    /// SMTP transport for production email sending.
    Smtp(AsyncSmtpTransport<Tokio1Executor>),
    /// Logging fallback for development mode.
    Log,
}

/// Email service for sending transactional emails.
#[derive(Clone)]
pub struct EmailService {
    /// Base URL for verification/reset links
    base_url: String,
    /// Whether to actually send emails (false in development)
    enabled: bool,
    /// SMTP configuration (None if using log fallback)
    smtp_config: Option<SmtpConfig>,
    /// SMTP transport (shared via Arc for Clone)
    transport: Arc<EmailTransport>,
}

impl EmailService {
    /// Create a new EmailService with SMTP support.
    ///
    /// If SMTP configuration is available via environment variables, it will be used.
    /// Otherwise, falls back to logging mode.
    pub fn new(base_url: String, enabled: bool) -> Self {
        let smtp_config = SmtpConfig::from_env();

        let transport = if enabled {
            if let Some(ref config) = smtp_config {
                match Self::create_smtp_transport(config) {
                    Ok(smtp) => {
                        tracing::info!(
                            host = %config.host,
                            port = %config.port,
                            "SMTP email transport configured"
                        );
                        Arc::new(EmailTransport::Smtp(smtp))
                    }
                    Err(e) => {
                        tracing::warn!(
                            error = %e,
                            "Failed to create SMTP transport, falling back to logging mode"
                        );
                        Arc::new(EmailTransport::Log)
                    }
                }
            } else {
                tracing::info!("SMTP not configured, using logging mode for emails");
                Arc::new(EmailTransport::Log)
            }
        } else {
            tracing::info!("Email sending disabled, using logging mode");
            Arc::new(EmailTransport::Log)
        };

        Self {
            base_url,
            enabled,
            smtp_config,
            transport,
        }
    }

    /// Create a development EmailService (logs instead of sending).
    pub fn development() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
            enabled: false,
            smtp_config: None,
            transport: Arc::new(EmailTransport::Log),
        }
    }

    /// Create SMTP transport from configuration.
    fn create_smtp_transport(
        config: &SmtpConfig,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, EmailError> {
        let creds = Credentials::new(config.username.clone(), config.password.clone());

        let builder = if config.use_tls {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
                .map_err(|e| EmailError::ConfigurationError(e.to_string()))?
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
        };

        Ok(builder.port(config.port).credentials(creds).build())
    }

    /// Send email verification email.
    pub async fn send_verification_email(
        &self,
        email: &str,
        name: &str,
        token: &str,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let verification_url = format!("{}/verify-email?token={}", self.base_url, token);
        let subject = self.get_verification_subject(locale);
        let body = self.get_verification_body(name, &verification_url, locale);

        self.send_email(email, &subject, &body).await
    }

    /// Send password reset email.
    pub async fn send_password_reset_email(
        &self,
        email: &str,
        name: &str,
        token: &str,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let reset_url = format!("{}/reset-password?token={}", self.base_url, token);
        let subject = self.get_reset_subject(locale);
        let body = self.get_reset_body(name, &reset_url, locale);

        self.send_email(email, &subject, &body).await
    }

    /// Send invitation email.
    pub async fn send_invitation_email(
        &self,
        email: &str,
        inviter_name: &str,
        token: &str,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let invitation_url = format!("{}/accept-invitation?token={}", self.base_url, token);
        let subject = self.get_invitation_subject(locale);
        let body = self.get_invitation_body(inviter_name, &invitation_url, locale);

        self.send_email(email, &subject, &body).await
    }

    /// Send a generic notification email.
    pub async fn send_notification_email(
        &self,
        email: &str,
        name: &str,
        subject: &str,
        message: &str,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let body = self.get_notification_body(name, message, locale);
        self.send_email(email, subject, &body).await
    }

    /// Internal send method supporting both SMTP and logging modes.
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
        match self.transport.as_ref() {
            EmailTransport::Smtp(smtp) => self.send_via_smtp(smtp, to, subject, body).await,
            EmailTransport::Log => {
                // Development mode: log the email content
                tracing::info!(
                    to = %to,
                    subject = %subject,
                    body_length = %body.len(),
                    "Email logged (development mode)"
                );
                tracing::debug!(
                    to = %to,
                    subject = %subject,
                    body = %body,
                    "Full email content (development mode)"
                );
                Ok(())
            }
        }
    }

    /// Send email via SMTP transport.
    async fn send_via_smtp(
        &self,
        transport: &AsyncSmtpTransport<Tokio1Executor>,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), EmailError> {
        let config = self
            .smtp_config
            .as_ref()
            .ok_or_else(|| EmailError::ConfigurationError("SMTP config missing".to_string()))?;

        // Parse sender address
        let from_mailbox: Mailbox = format!("{} <{}>", config.from_name, config.from_address)
            .parse()
            .map_err(|e| EmailError::InvalidAddress(format!("Invalid from address: {}", e)))?;

        // Parse recipient address
        let to_mailbox: Mailbox = to.parse().map_err(|e| {
            EmailError::InvalidAddress(format!("Invalid to address '{}': {}", to, e))
        })?;

        // Build the email message
        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .map_err(|e| EmailError::MessageBuildError(e.to_string()))?;

        // Send the email
        transport.send(email).await.map_err(|e| {
            tracing::error!(
                error = %e,
                to = %to,
                subject = %subject,
                "Failed to send email via SMTP"
            );
            EmailError::SendFailed(e.to_string())
        })?;

        tracing::info!(
            to = %to,
            subject = %subject,
            "Email sent successfully via SMTP"
        );

        Ok(())
    }

    /// Send email with HTML content.
    pub async fn send_html_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<(), EmailError> {
        match self.transport.as_ref() {
            EmailTransport::Smtp(smtp) => {
                self.send_html_via_smtp(smtp, to, subject, html_body, text_body)
                    .await
            }
            EmailTransport::Log => {
                tracing::info!(
                    to = %to,
                    subject = %subject,
                    html_length = %html_body.len(),
                    text_length = %text_body.len(),
                    "HTML email logged (development mode)"
                );
                Ok(())
            }
        }
    }

    /// Send HTML email via SMTP transport.
    async fn send_html_via_smtp(
        &self,
        transport: &AsyncSmtpTransport<Tokio1Executor>,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<(), EmailError> {
        use lettre::message::{MultiPart, SinglePart};

        let config = self
            .smtp_config
            .as_ref()
            .ok_or_else(|| EmailError::ConfigurationError("SMTP config missing".to_string()))?;

        let from_mailbox: Mailbox = format!("{} <{}>", config.from_name, config.from_address)
            .parse()
            .map_err(|e| EmailError::InvalidAddress(format!("Invalid from address: {}", e)))?;

        let to_mailbox: Mailbox = to.parse().map_err(|e| {
            EmailError::InvalidAddress(format!("Invalid to address '{}': {}", to, e))
        })?;

        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(text_body.to_string()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(html_body.to_string()),
                    ),
            )
            .map_err(|e| EmailError::MessageBuildError(e.to_string()))?;

        transport.send(email).await.map_err(|e| {
            tracing::error!(
                error = %e,
                to = %to,
                subject = %subject,
                "Failed to send HTML email via SMTP"
            );
            EmailError::SendFailed(e.to_string())
        })?;

        tracing::info!(
            to = %to,
            subject = %subject,
            "HTML email sent successfully via SMTP"
        );

        Ok(())
    }

    // ==================== Localized Templates ====================

    fn get_verification_subject(&self, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => "Overte svoju e-mailovú adresu".to_string(),
            Locale::Czech => "Ověřte svou e-mailovou adresu".to_string(),
            Locale::German => "Bestätigen Sie Ihre E-Mail-Adresse".to_string(),
            Locale::English => "Verify your email address".to_string(),
        }
    }

    fn get_verification_body(&self, name: &str, url: &str, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\nĎakujeme za registráciu. Kliknutím na odkaz nižšie overte svoju e-mailovú adresu:\n\n{}\n\nOdkaz je platný 24 hodín.\n\nS pozdravom,\nTím PPT",
                name, url
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\nDěkujeme za registraci. Kliknutím na odkaz níže ověřte svou e-mailovou adresu:\n\n{}\n\nOdkaz je platný 24 hodin.\n\nS pozdravem,\nTým PPT",
                name, url
            ),
            Locale::German => format!(
                "Guten Tag {},\n\nVielen Dank für Ihre Registrierung. Klicken Sie auf den Link unten, um Ihre E-Mail-Adresse zu bestätigen:\n\n{}\n\nDer Link ist 24 Stunden gültig.\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name, url
            ),
            Locale::English => format!(
                "Hello {},\n\nThank you for registering. Click the link below to verify your email address:\n\n{}\n\nThis link is valid for 24 hours.\n\nBest regards,\nThe PPT Team",
                name, url
            ),
        }
    }

    fn get_reset_subject(&self, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => "Obnovenie hesla".to_string(),
            Locale::Czech => "Obnovení hesla".to_string(),
            Locale::German => "Passwort zurücksetzen".to_string(),
            Locale::English => "Reset your password".to_string(),
        }
    }

    fn get_reset_body(&self, name: &str, url: &str, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\nDostali sme žiadosť o obnovenie vášho hesla. Kliknutím na odkaz nižšie ho obnovte:\n\n{}\n\nOdkaz je platný 1 hodinu. Ak ste o obnovenie hesla nežiadali, túto správu ignorujte.\n\nS pozdravom,\nTím PPT",
                name, url
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\nObdrželi jsme žádost o obnovení vašeho hesla. Kliknutím na odkaz níže ho obnovte:\n\n{}\n\nOdkaz je platný 1 hodinu. Pokud jste o obnovení hesla nežádali, tuto zprávu ignorujte.\n\nS pozdravem,\nTým PPT",
                name, url
            ),
            Locale::German => format!(
                "Guten Tag {},\n\nWir haben eine Anfrage zum Zurücksetzen Ihres Passworts erhalten. Klicken Sie auf den Link unten:\n\n{}\n\nDer Link ist 1 Stunde gültig. Wenn Sie kein Zurücksetzen angefordert haben, ignorieren Sie diese Nachricht.\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name, url
            ),
            Locale::English => format!(
                "Hello {},\n\nWe received a request to reset your password. Click the link below:\n\n{}\n\nThis link is valid for 1 hour. If you didn't request a password reset, please ignore this email.\n\nBest regards,\nThe PPT Team",
                name, url
            ),
        }
    }

    fn get_invitation_subject(&self, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => "Pozvánka do Property Management System".to_string(),
            Locale::Czech => "Pozvánka do Property Management System".to_string(),
            Locale::German => "Einladung zum Property Management System".to_string(),
            Locale::English => "Invitation to Property Management System".to_string(),
        }
    }

    fn get_invitation_body(&self, inviter: &str, url: &str, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň,\n\n{} vás pozýva pripojiť sa k Property Management System. Kliknutím na odkaz nižšie vytvorte účet:\n\n{}\n\nS pozdravom,\nTím PPT",
                inviter, url
            ),
            Locale::Czech => format!(
                "Dobrý den,\n\n{} vás zve připojit se k Property Management System. Kliknutím na odkaz níže vytvořte účet:\n\n{}\n\nS pozdravem,\nTým PPT",
                inviter, url
            ),
            Locale::German => format!(
                "Guten Tag,\n\n{} lädt Sie ein, dem Property Management System beizutreten. Klicken Sie auf den Link unten:\n\n{}\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                inviter, url
            ),
            Locale::English => format!(
                "Hello,\n\n{} has invited you to join Property Management System. Click the link below to create your account:\n\n{}\n\nBest regards,\nThe PPT Team",
                inviter, url
            ),
        }
    }

    fn get_notification_body(&self, name: &str, message: &str, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\n{}\n\nS pozdravom,\nTím PPT",
                name, message
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\n{}\n\nS pozdravem,\nTým PPT",
                name, message
            ),
            Locale::German => format!(
                "Guten Tag {},\n\n{}\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name, message
            ),
            Locale::English => format!(
                "Hello {},\n\n{}\n\nBest regards,\nThe PPT Team",
                name, message
            ),
        }
    }

    // ==================== Additional Email Types ====================

    /// Send welcome email after successful email verification.
    pub async fn send_welcome_email(
        &self,
        email: &str,
        name: &str,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let subject = self.get_welcome_subject(locale);
        let body = self.get_welcome_body(name, locale);

        self.send_email(email, &subject, &body).await
    }

    fn get_welcome_subject(&self, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => "Vitajte v Property Management System".to_string(),
            Locale::Czech => "Vítejte v Property Management System".to_string(),
            Locale::German => "Willkommen im Property Management System".to_string(),
            Locale::English => "Welcome to Property Management System".to_string(),
        }
    }

    fn get_welcome_body(&self, name: &str, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\nVitajte v Property Management System! Váš účet bol úspešne overený a je pripravený na použitie.\n\nPrihláste sa na {} a začnite spravovať svoje nehnuteľnosti.\n\nAk máte akékoľvek otázky, neváhajte nás kontaktovať.\n\nS pozdravom,\nTím PPT",
                name, self.base_url
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\nVítejte v Property Management System! Váš účet byl úspěšně ověřen a je připraven k použití.\n\nPřihlaste se na {} a začněte spravovat své nemovitosti.\n\nPokud máte jakékoli otázky, neváhejte nás kontaktovat.\n\nS pozdravem,\nTým PPT",
                name, self.base_url
            ),
            Locale::German => format!(
                "Guten Tag {},\n\nWillkommen im Property Management System! Ihr Konto wurde erfolgreich verifiziert und ist einsatzbereit.\n\nMelden Sie sich unter {} an und beginnen Sie mit der Verwaltung Ihrer Immobilien.\n\nBei Fragen stehen wir Ihnen gerne zur Verfügung.\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name, self.base_url
            ),
            Locale::English => format!(
                "Hello {},\n\nWelcome to Property Management System! Your account has been successfully verified and is ready to use.\n\nLog in at {} to start managing your properties.\n\nIf you have any questions, don't hesitate to contact us.\n\nBest regards,\nThe PPT Team",
                name, self.base_url
            ),
        }
    }

    /// Send security alert email (e.g., new login from unknown device).
    pub async fn send_security_alert_email(
        &self,
        email: &str,
        name: &str,
        device_info: &str,
        ip_address: &str,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let subject = self.get_security_alert_subject(locale);
        let body = self.get_security_alert_body(name, device_info, ip_address, locale);

        self.send_email(email, &subject, &body).await
    }

    fn get_security_alert_subject(&self, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => "Bezpečnostné upozornenie - nové prihlásenie".to_string(),
            Locale::Czech => "Bezpečnostní upozornění - nové přihlášení".to_string(),
            Locale::German => "Sicherheitswarnung - Neue Anmeldung".to_string(),
            Locale::English => "Security Alert - New Login".to_string(),
        }
    }

    fn get_security_alert_body(
        &self,
        name: &str,
        device: &str,
        ip: &str,
        locale: &Locale,
    ) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\nZaznamenali sme nové prihlásenie do vášho účtu:\n\n- Zariadenie: {}\n- IP adresa: {}\n\nAk ste to boli vy, túto správu môžete ignorovať.\n\nAk ste sa neprihlasovali, odporúčame okamžite zmeniť heslo a skontrolovať aktívne relácie v nastaveniach účtu.\n\nS pozdravom,\nTím PPT",
                name, device, ip
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\nZaznamenali jsme nové přihlášení k vašemu účtu:\n\n- Zařízení: {}\n- IP adresa: {}\n\nPokud jste to byli vy, tuto zprávu můžete ignorovat.\n\nPokud jste se nepřihlašovali, doporučujeme okamžitě změnit heslo a zkontrolovat aktivní relace v nastavení účtu.\n\nS pozdravem,\nTým PPT",
                name, device, ip
            ),
            Locale::German => format!(
                "Guten Tag {},\n\nWir haben eine neue Anmeldung bei Ihrem Konto festgestellt:\n\n- Gerät: {}\n- IP-Adresse: {}\n\nWenn Sie das waren, können Sie diese Nachricht ignorieren.\n\nFalls Sie sich nicht angemeldet haben, empfehlen wir, sofort das Passwort zu ändern und aktive Sitzungen in den Kontoeinstellungen zu überprüfen.\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name, device, ip
            ),
            Locale::English => format!(
                "Hello {},\n\nWe detected a new login to your account:\n\n- Device: {}\n- IP Address: {}\n\nIf this was you, you can ignore this message.\n\nIf you didn't log in, we recommend immediately changing your password and reviewing active sessions in your account settings.\n\nBest regards,\nThe PPT Team",
                name, device, ip
            ),
        }
    }

    /// Send password changed notification email.
    pub async fn send_password_changed_email(
        &self,
        email: &str,
        name: &str,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let subject = self.get_password_changed_subject(locale);
        let body = self.get_password_changed_body(name, locale);

        self.send_email(email, &subject, &body).await
    }

    fn get_password_changed_subject(&self, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => "Vaše heslo bolo zmenené".to_string(),
            Locale::Czech => "Vaše heslo bylo změněno".to_string(),
            Locale::German => "Ihr Passwort wurde geändert".to_string(),
            Locale::English => "Your password has been changed".to_string(),
        }
    }

    fn get_password_changed_body(&self, name: &str, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\nVaše heslo bolo úspešne zmenené.\n\nAk ste túto zmenu nevykonali, okamžite kontaktujte podporu a zmeňte si heslo.\n\nS pozdravom,\nTím PPT",
                name
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\nVaše heslo bylo úspěšně změněno.\n\nPokud jste tuto změnu neprovedli, okamžitě kontaktujte podporu a změňte si heslo.\n\nS pozdravem,\nTým PPT",
                name
            ),
            Locale::German => format!(
                "Guten Tag {},\n\nIhr Passwort wurde erfolgreich geändert.\n\nWenn Sie diese Änderung nicht vorgenommen haben, kontaktieren Sie sofort den Support und ändern Sie Ihr Passwort.\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name
            ),
            Locale::English => format!(
                "Hello {},\n\nYour password has been successfully changed.\n\nIf you didn't make this change, please contact support immediately and change your password.\n\nBest regards,\nThe PPT Team",
                name
            ),
        }
    }

    /// Send account suspended notification email.
    pub async fn send_account_suspended_email(
        &self,
        email: &str,
        name: &str,
        reason: Option<&str>,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let subject = self.get_account_suspended_subject(locale);
        let body = self.get_account_suspended_body(name, reason, locale);

        self.send_email(email, &subject, &body).await
    }

    fn get_account_suspended_subject(&self, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => "Váš účet bol pozastavený".to_string(),
            Locale::Czech => "Váš účet byl pozastaven".to_string(),
            Locale::German => "Ihr Konto wurde gesperrt".to_string(),
            Locale::English => "Your account has been suspended".to_string(),
        }
    }

    fn get_account_suspended_body(
        &self,
        name: &str,
        reason: Option<&str>,
        locale: &Locale,
    ) -> String {
        let reason_text = reason.unwrap_or("-");
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\nVáš účet bol pozastavený.\n\nDôvod: {}\n\nPre viac informácií kontaktujte podporu.\n\nS pozdravom,\nTím PPT",
                name, reason_text
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\nVáš účet byl pozastaven.\n\nDůvod: {}\n\nPro více informací kontaktujte podporu.\n\nS pozdravem,\nTým PPT",
                name, reason_text
            ),
            Locale::German => format!(
                "Guten Tag {},\n\nIhr Konto wurde gesperrt.\n\nGrund: {}\n\nFür weitere Informationen wenden Sie sich bitte an den Support.\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name, reason_text
            ),
            Locale::English => format!(
                "Hello {},\n\nYour account has been suspended.\n\nReason: {}\n\nFor more information, please contact support.\n\nBest regards,\nThe PPT Team",
                name, reason_text
            ),
        }
    }

    /// Send fault notification email to building manager.
    #[allow(clippy::too_many_arguments)]
    pub async fn send_fault_notification_email(
        &self,
        email: &str,
        name: &str,
        fault_title: &str,
        fault_description: &str,
        building_name: &str,
        reporter_name: &str,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let subject = self.get_fault_notification_subject(building_name, locale);
        let body = self.get_fault_notification_body(
            name,
            fault_title,
            fault_description,
            building_name,
            reporter_name,
            locale,
        );

        self.send_email(email, &subject, &body).await
    }

    fn get_fault_notification_subject(&self, building_name: &str, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => format!("Nová porucha nahlásená - {}", building_name),
            Locale::Czech => format!("Nová porucha nahlášena - {}", building_name),
            Locale::German => format!("Neuer Mangel gemeldet - {}", building_name),
            Locale::English => format!("New Fault Reported - {}", building_name),
        }
    }

    fn get_fault_notification_body(
        &self,
        name: &str,
        fault_title: &str,
        fault_description: &str,
        building_name: &str,
        reporter_name: &str,
        locale: &Locale,
    ) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\nBola nahlásená nová porucha v budove {}.\n\nNázov: {}\nPopis: {}\nNahlásil: {}\n\nPreskúmajte prosím túto poruchu v systéme.\n\nS pozdravom,\nTím PPT",
                name, building_name, fault_title, fault_description, reporter_name
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\nByla nahlášena nová porucha v budově {}.\n\nNázev: {}\nPopis: {}\nNahlásil: {}\n\nProsím přezkoumejte tuto poruchu v systému.\n\nS pozdravem,\nTým PPT",
                name, building_name, fault_title, fault_description, reporter_name
            ),
            Locale::German => format!(
                "Guten Tag {},\n\nEin neuer Mangel wurde im Gebäude {} gemeldet.\n\nTitel: {}\nBeschreibung: {}\nGemeldet von: {}\n\nBitte überprüfen Sie diesen Mangel im System.\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name, building_name, fault_title, fault_description, reporter_name
            ),
            Locale::English => format!(
                "Hello {},\n\nA new fault has been reported in building {}.\n\nTitle: {}\nDescription: {}\nReported by: {}\n\nPlease review this fault in the system.\n\nBest regards,\nThe PPT Team",
                name, building_name, fault_title, fault_description, reporter_name
            ),
        }
    }

    /// Send voting reminder email.
    pub async fn send_voting_reminder_email(
        &self,
        email: &str,
        name: &str,
        vote_title: &str,
        deadline: &str,
        locale: &Locale,
    ) -> Result<(), EmailError> {
        let subject = self.get_voting_reminder_subject(locale);
        let body = self.get_voting_reminder_body(name, vote_title, deadline, locale);

        self.send_email(email, &subject, &body).await
    }

    fn get_voting_reminder_subject(&self, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => "Pripomienka hlasovania".to_string(),
            Locale::Czech => "Připomínka hlasování".to_string(),
            Locale::German => "Abstimmungserinnerung".to_string(),
            Locale::English => "Voting Reminder".to_string(),
        }
    }

    fn get_voting_reminder_body(
        &self,
        name: &str,
        vote_title: &str,
        deadline: &str,
        locale: &Locale,
    ) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\nPripomíname vám hlasovanie: {}\n\nTermín na hlasovanie: {}\n\nPrihláste sa do systému a odovzdajte svoj hlas.\n\nS pozdravom,\nTím PPT",
                name, vote_title, deadline
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\nPřipomínáme vám hlasování: {}\n\nTermín pro hlasování: {}\n\nPřihlaste se do systému a odevzdejte svůj hlas.\n\nS pozdravem,\nTým PPT",
                name, vote_title, deadline
            ),
            Locale::German => format!(
                "Guten Tag {},\n\nWir erinnern Sie an die Abstimmung: {}\n\nAbstimmungsfrist: {}\n\nMelden Sie sich im System an und geben Sie Ihre Stimme ab.\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name, vote_title, deadline
            ),
            Locale::English => format!(
                "Hello {},\n\nReminder about the vote: {}\n\nVoting deadline: {}\n\nLog in to the system to cast your vote.\n\nBest regards,\nThe PPT Team",
                name, vote_title, deadline
            ),
        }
    }

    /// Check if SMTP is properly configured.
    pub fn is_smtp_configured(&self) -> bool {
        matches!(self.transport.as_ref(), EmailTransport::Smtp(_))
    }

    /// Check if email sending is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smtp_config_from_env_missing() {
        // When SMTP env vars are not set, from_env returns None
        std::env::remove_var("SMTP_HOST");
        let config = SmtpConfig::from_env();
        assert!(config.is_none());
    }

    #[test]
    fn test_development_mode() {
        let service = EmailService::development();
        assert!(!service.is_enabled());
        assert!(!service.is_smtp_configured());
    }

    #[test]
    fn test_verification_subject_localization() {
        let service = EmailService::development();

        assert_eq!(
            service.get_verification_subject(&Locale::English),
            "Verify your email address"
        );
        assert_eq!(
            service.get_verification_subject(&Locale::Slovak),
            "Overte svoju e-mailovú adresu"
        );
        assert_eq!(
            service.get_verification_subject(&Locale::Czech),
            "Ověřte svou e-mailovou adresu"
        );
        assert_eq!(
            service.get_verification_subject(&Locale::German),
            "Bestätigen Sie Ihre E-Mail-Adresse"
        );
    }

    #[test]
    fn test_reset_subject_localization() {
        let service = EmailService::development();

        assert_eq!(
            service.get_reset_subject(&Locale::English),
            "Reset your password"
        );
        assert_eq!(
            service.get_reset_subject(&Locale::Slovak),
            "Obnovenie hesla"
        );
    }

    #[tokio::test]
    async fn test_send_email_development_mode() {
        let service = EmailService::development();

        // In development mode, send_email should succeed without actually sending
        let result = service
            .send_email("test@example.com", "Test Subject", "Test Body")
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_verification_email_development_mode() {
        let service = EmailService::development();

        let result = service
            .send_verification_email(
                "test@example.com",
                "John Doe",
                "test-token-123",
                &Locale::English,
            )
            .await;

        assert!(result.is_ok());
    }
}
