// src/utils/config.rs
use std::env;
use dotenvy::dotenv;
use secrecy::{Secret, ExposeSecret};

#[derive(Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
}

impl SmtpConfig {
    pub fn load() -> Result<Self, env::VarError> {
        // Load .env file
        dotenv().ok();

        Ok(Self {
            host: env::var("SMTP_HOST")?.to_string(),
            port: env::var("SMTP_PORT")?.parse().unwrap_or(587),
            username: env::var("GMAIL_USER")?.to_string(),
            password: Secret::new(env::var("GMAIL_PASSWORD")?),
        })
    }
}

// Secure credential handling
pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}