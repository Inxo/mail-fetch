use crate::email_processor;
use crate::imap_client;
use crate::database;
// use sqlx::sqlite::SqlitePool;
use native_tls::TlsConnector;
use std::env;
use cron::Schedule;
use std::str::FromStr;
use tokio::time::sleep;
use chrono::Utc;

pub async fn run_scheduler() -> anyhow::Result<()> {
    // Получаем расписание из переменной окружения или используем значение по умолчанию
    let cron_expression = env::var("CRON_SCHEDULE").unwrap_or_else(|_| "* * * * * *".to_string());
    let schedule = Schedule::from_str(&cron_expression)?;

    let mut next = schedule.upcoming(Utc).next().unwrap();
    loop {
        let now = Utc::now();
        if next > now {
            let duration = next.signed_duration_since(now).to_std().unwrap();
            sleep(duration).await;
        }

        // Выполняем задачу
        check_emails().await?;

        next = schedule.upcoming(Utc).next().unwrap();
    }
}

async fn check_emails() -> anyhow::Result<()> {
    let imap_server = env::var("IMAP_SERVER")?;
    let imap_port: u16 = env::var("IMAP_PORT")?.parse()?;
    let imap_username = env::var("IMAP_EMAIL")?;
    let imap_password = env::var("IMAP_PASSWORD")?;
    let folder = env::var("IMAP_FOLDER").unwrap_or_else(|_| "INBOX".to_string());
    let folder_sent: String = env::var("IMAP_SENT_FOLDER").unwrap_or_else(|_| "Sent".to_string());

    // Устанавливаем соединение с базой данных
    let pool = database::connect().await?;

    // Создание таблицы, если она не существует
    database::initialize(&pool).await?;

    // Настройки подключения к IMAP серверу
    let tls = TlsConnector::builder().build()?;
    let mut imap_session = imap_client::connect(&imap_server, imap_port, &imap_username, &imap_password, &tls)?;

    // Обработка входящих писем
    email_processor::process_emails(&mut imap_session, &pool, &folder).await?;

    // Обработка отправленных писем
    email_processor::process_emails(&mut imap_session, &pool, &folder_sent).await?;

    // Завершение сессии
    imap_session.logout()?;
    
    Ok(())
}
