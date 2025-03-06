use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use shuttle_runtime::SecretStore;
use chrono::NaiveDate;

pub async fn send_vacation_email(
    secrets: &SecretStore, 
    start_date: &NaiveDate, 
    end_date: &NaiveDate
) -> Result<(), Box<dyn std::error::Error>> {
    let email_user = secrets.get("GMAIL_USER")
        .expect("Gmail user not configured");
    let email_pass = secrets.get("GMAIL_PASS")
        .expect("Gmail password not configured");

    let email = Message::builder()
        .from(email_user.parse()?)
        .to("boss@company.com".parse()?)
        .subject("Vacation Request")
        .body(format!(
            "Hi Boss,\n\nI request vacation from {} to {}.\n\nBest,\nMe",
            start_date, end_date
        ))?;

    let creds = Credentials::new(email_user.to_string(), email_pass.to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}