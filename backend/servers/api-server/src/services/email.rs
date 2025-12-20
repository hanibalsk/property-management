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

    fn get_security_alert_body(&self, name: &str, device: &str, ip: &str, locale: &Locale) -> String {
        match locale {
            Locale::Slovak => format!(
                "Dobrý deň {},\n\nZaznamenali sme nové prihlásenie do vášho účtu:\n\n• Zariadenie: {}\n• IP adresa: {}\n\nAk ste to boli vy, túto správu môžete ignorovať.\n\nAk ste sa neprihlasovali, odporúčame okamžite zmeniť heslo a skontrolovať aktívne relácie v nastaveniach účtu.\n\nS pozdravom,\nTím PPT",
                name, device, ip
            ),
            Locale::Czech => format!(
                "Dobrý den {},\n\nZaznamenali jsme nové přihlášení k vašemu účtu:\n\n• Zařízení: {}\n• IP adresa: {}\n\nPokud jste to byli vy, tuto zprávu můžete ignorovat.\n\nPokud jste se nepřihlašovali, doporučujeme okamžitě změnit heslo a zkontrolovat aktivní relace v nastavení účtu.\n\nS pozdravem,\nTým PPT",
                name, device, ip
            ),
            Locale::German => format!(
                "Guten Tag {},\n\nWir haben eine neue Anmeldung bei Ihrem Konto festgestellt:\n\n• Gerät: {}\n• IP-Adresse: {}\n\nWenn Sie das waren, können Sie diese Nachricht ignorieren.\n\nFalls Sie sich nicht angemeldet haben, empfehlen wir, sofort das Passwort zu ändern und aktive Sitzungen in den Kontoeinstellungen zu überprüfen.\n\nMit freundlichen Grüßen,\nDas PPT-Team",
                name, device, ip
            ),
            Locale::English => format!(
                "Hello {},\n\nWe detected a new login to your account:\n\n• Device: {}\n• IP Address: {}\n\nIf this was you, you can ignore this message.\n\nIf you didn't log in, we recommend immediately changing your password and reviewing active sessions in your account settings.\n\nBest regards,\nThe PPT Team",
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

    fn get_account_suspended_body(&self, name: &str, reason: Option<&str>, locale: &Locale) -> String {
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
}
