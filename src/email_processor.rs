use imap::Session;
use mailparse::{parse_mail, MailHeader, ParsedMail};
use native_tls::TlsStream;
use sqlx::sqlite::SqlitePool;
use std::collections::HashSet;
use anyhow::Context;
use html2text::from_read;

fn get_header_value(headers: &[MailHeader], name: &str) -> Option<String> {
    headers.iter()
        .find(|header| header.get_key().map_or(false, |k| k.eq_ignore_ascii_case(name)))
        .and_then(|header| header.get_value().ok())
}

fn get_body_text(mail: &ParsedMail) -> String {
    if mail.subparts.is_empty() {
        if mail.ctype.mimetype == "text/plain" {
            mail.get_body().unwrap_or_default()
        } else if mail.ctype.mimetype == "text/html" {
            let html_body = mail.get_body().unwrap_or_default();
            from_read(html_body.as_bytes(), 80).to_string()
        } else {
            String::new()
        }
    } else {
        mail.subparts.iter().filter_map(|subpart| {
            let body = get_body_text(subpart);
            if !body.is_empty() {
                Some(body)
            } else {
                None
            }
        }).collect::<Vec<_>>().join("\n")
    }
}

pub async fn process_emails(imap_session: &mut Session<TlsStream<std::net::TcpStream>>, pool: &SqlitePool, folder: &str) -> anyhow::Result<()> {
    imap_session.select(folder)?;

    let since = chrono::Utc::now() - chrono::Duration::hours(24);
    let since_str = since.format("%d-%b-%Y").to_string();

    let search_query = format!("SINCE {}", since_str);
    let messages = imap_session.search(&search_query).context("Failed to search emails")?;
    let message_ids: HashSet<_> = messages.iter().collect();

    for message_id in message_ids {
        if let Ok(fetch) = imap_session.fetch(message_id.to_string(), "RFC822").context("Failed to fetch email") {
            if let Some(message) = fetch.iter().next() {
                let body = message.body().expect("message did not have a body!");
                let email = parse_mail(body).context("Failed to parse email")?;

                let sender = get_header_value(&email.headers, "From").unwrap_or_default();
                let recipient = get_header_value(&email.headers, "To").unwrap_or_default();
                let subject = get_header_value(&email.headers, "Subject").unwrap_or_default();
                let body = get_body_text(&email);
                println!("{}", body);
                let message_id = get_header_value(&email.headers, "Message-ID").unwrap_or_default();

                let exists = crate::database::email_exists(pool, &message_id, folder).await?;

                if !exists {
                    crate::database::save_email(pool, &sender, &recipient, &subject, &body, &message_id, folder).await?;
                }
            }
        }
    }

    Ok(())
}
