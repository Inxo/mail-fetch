mod imap_client;
mod database;
mod email_processor;

use chrono::{Duration, Utc};
use dotenv::dotenv;
use native_tls::TlsConnector;
// use sqlx::sqlite::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Загрузка переменных окружения из .env файла
    dotenv().ok();

    // Получение переменных окружения
    let imap_server = env::var("IMAP_SERVER")?;
    let imap_port: u16 = env::var("IMAP_PORT")?.parse()?;
    let imap_email = env::var("IMAP_EMAIL")?;
    let imap_password = env::var("IMAP_PASSWORD")?;

    // Подключение к базе данных
    let pool = database::connect().await?;

    // Создание таблицы, если она не существует
    database::initialize(&pool).await?;

    // Настройки подключения к IMAP серверу
    let tls = TlsConnector::builder().build()?;
    let mut imap_session = imap_client::connect(&imap_server, imap_port, &imap_email, &imap_password, &tls)?;

    // Получение времени 4 часа назад
    let since = Utc::now() - Duration::hours(24);
    let since_str = since.format("%d-%b-%Y").to_string();

    // Обработка входящих писем
    email_processor::process_emails(&mut imap_session, &pool, "INBOX").await?;

    // Обработка отправленных писем
    email_processor::process_emails(&mut imap_session, &pool, "Sent").await?;

    // Завершение сессии
    imap_session.logout()?;

    Ok(())
}
