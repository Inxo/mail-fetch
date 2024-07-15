use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;
// use sqlx::sqlite::SqlitePoolOptions;

pub async fn connect() -> anyhow::Result<SqlitePool> {
    // Создаем опции подключения к SQLite
    let options = SqliteConnectOptions::new()
        .filename("emails.db")  // Указываем имя файла базы данных SQLite
        .create_if_missing(true);  // Создаем файл, если он не существует

    // Устанавливаем соединение с базой данных
    let pool = SqlitePool::connect_with(options).await?;
    Ok(pool)
}

pub async fn initialize(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS emails (
            id INTEGER PRIMARY KEY,
            sender TEXT,
            recipient TEXT,
            subject TEXT,
            body TEXT,
            message_id TEXT,
            folder TEXT,
            UNIQUE(message_id, folder)
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn email_exists(pool: &SqlitePool, message_id: &str, folder: &str) -> anyhow::Result<bool> {
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM emails WHERE message_id = ? AND folder = ?)")
        .bind(message_id)
        .bind(folder)
        .fetch_one(pool)
        .await?;
    Ok(exists)
}

pub async fn save_email(pool: &SqlitePool, sender: &str, recipient: &str, subject: &str, body: &str, message_id: &str, folder: &str) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO emails (sender, recipient, subject, body, message_id, folder) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(sender)
        .bind(recipient)
        .bind(subject)
        .bind(body)
        .bind(message_id)
        .bind(folder)
        .execute(pool)
        .await?;
    Ok(())
}
