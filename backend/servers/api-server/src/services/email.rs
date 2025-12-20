//! Email service (Epic 1, Story 1.7).
//!
//! This is a stub implementation. In production, integrate with
//! an email provider (SendGrid, AWS SES, etc.).

use db::models::Locale;
use thiserror::Error;

/// Email service errors.
#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Failed to send email: {0}")]
    SendFailed(String),

    #[error("Invalid email address")]
    InvalidAddress,

    #[error("Template not found: {0}")]
    TemplateNotFound(String),
}

/// Email service for sending transactional emails.
#[derive(Clone)]
pub struct EmailService {
    /// Base URL for verification/reset links
    base_url: String,
    /// Whether to actually send emails (false in development)
    enabled: bool,
}

impl EmailService {
    /// Create a new EmailService.
    pub fn new(base_url: String, enabled: bool) -> Self {
        Self { base_url, enabled }
    }

    /// Create a development EmailService (logs instead of sending).
    pub fn development() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
            enabled: false,
        }
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

    /// Internal send method.
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
        if self.enabled {
            // TODO: Implement actual email sending
            // In production, use an email provider SDK
            tracing::info!(to = %to, subject = %subject, "Would send email (not implemented)");
        } else {
            // Development mode: log the email
            tracing::info!(
                to = %to,
                subject = %subject,
                body = %body,
                "Email (dev mode - not sent)"
            );
        }
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
}
