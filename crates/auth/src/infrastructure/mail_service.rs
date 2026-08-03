use anyhow::Context;
use async_trait::async_trait;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use tracing;

/// Abstraction over email delivery so use cases stay testable.
#[async_trait]
pub trait MailService: Send + Sync {
    /// Send an email-verification link for `token`.
    async fn send_verification(&self, to: &str, token: &str) -> anyhow::Result<()>;

    /// Send a password-reset link for `token`.
    async fn send_password_reset(&self, to: &str, token: &str) -> anyhow::Result<()>;
}

/// SMTP-backed implementation. Reads configuration from environment:
///
/// - `SMTP_HOST`     (required)
/// - `SMTP_PORT`     (default 587)
/// - `SMTP_USERNAME` (required for auth)
/// - `SMTP_PASSWORD` (required for auth)
/// - `SMTP_FROM`     (default "HuluPay <no-reply@hulupay.com>")
/// - `APP_BASE_URL`  (default "http://localhost:3000" — used to build links)
pub struct SmtpMailService {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    base_url: String,
}

impl SmtpMailService {
    pub fn from_env() -> anyhow::Result<Self> {
        let host = std::env::var("SMTP_HOST")
            .context("SMTP_HOST must be set to send email")?;
        let port: u16 = std::env::var("SMTP_PORT")
            .ok() 
            .and_then(|p| p.parse().ok())
            .unwrap_or(587);
        let username = std::env::var("SMTP_USERNAME")
            .context("SMTP_USERNAME must be set to send email")?;
        let password = std::env::var("SMTP_PASSWORD")
            .context("SMTP_PASSWORD must be set to send email")?;

        let from_str = std::env::var("SMTP_FROM")
            .unwrap_or_else(|_| "HuluPay <no-reply@hulupay.com>".to_string());
        let from: Mailbox = from_str
            .parse()
            .context("SMTP_FROM is not a valid mailbox")?;

        let base_url = std::env::var("APP_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
            // .port(port)
            .credentials(Credentials::new(username, password))
            .build();

        Ok(Self {
            transport,
            from,
            base_url,
        })
    }

    async fn send(&self, to: &str, subject: &str, body: String) -> anyhow::Result<()> {
        let to_mailbox: Mailbox = to
            .parse()
            .with_context(|| format!("recipient '{to}' is not a valid mailbox"))?;

        let email = Message::builder()
            .from(self.from.clone())
            .to(to_mailbox)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)?;

        match self.transport.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!(error = %e, to = to, "failed to send email");
                Err(anyhow::anyhow!("failed to send email: {e}"))
            }
        }
    }
}

#[async_trait]
impl MailService for SmtpMailService {
    async fn send_verification(&self, to: &str, token: &str) -> anyhow::Result<()> {
        let link = format!("{}/verify-email?token={token}", self.base_url);
        let body = format!(
            "Welcome to HuluPay!\n\nPlease verify your email address by clicking the link below:\n\n{link}\n\nIf you did not create an account, you can safely ignore this email."
        );
        self.send(to, "Verify your HuluPay email", body).await
    }

    async fn send_password_reset(&self, to: &str, token: &str) -> anyhow::Result<()> {
        let link = format!("{}/reset-password?token={token}", self.base_url);
        let body = format!(
            "We received a request to reset your HuluPay password.\n\nReset it by clicking the link below:\n\n{link}\n\nIf you did not request a password reset, you can safely ignore this email."
        );
        self.send(to, "Reset your HuluPay password", body).await
    }
}
