use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use anyhow::{Result, Context};
use crate::utils::config::SmtpConfig;

pub struct EmailService {
    config: SmtpConfig,
}

impl EmailService {
    pub fn new(config: SmtpConfig) -> Self {
        Self { config }
    }

    pub fn send_vacation_request(
        &self, 
        boss_name: &str, 
        employee_name: &str, 
        start_date: &str, 
        end_date: &str
    ) -> Result<()> {
        let email = Message::builder()
            .from(self.config.username.parse()?)
            .to(format!("{} <{}>", boss_name, self.config.username).parse()?)
            .subject("Vacation Request")
            .body(format!(
                "Hi {},\n\nI request vacation from {} to {}.\n\nBest,\n{}",
                boss_name, start_date, end_date, employee_name
            ))?;

        let creds = Credentials::new(
            self.config.username.clone(), 
            self.config.password.expose_secret().clone()
        );

        let mailer = SmtpTransport::relay(&self.config.host)?
            .credentials(creds)
            .port(self.config.port)
            .build();

        mailer.send(&email)
            .context("Failed to send email")?;

        Ok(())
    }
}