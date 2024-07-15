mod imap_client;
mod database;
mod email_processor;
mod scheduler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    scheduler::run_scheduler().await?;

    Ok(())
}
