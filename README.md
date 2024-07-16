
# Rust Email Checker

Rust Email Checker is an application for checking emails via IMAP and storing email information in an SQLite database. The application uses a cron format schedule for periodic email checks.

## Features

- Connects to an IMAP server to check incoming and sent emails.
- Stores email information (sender, recipient, subject, body) in an SQLite database.
- Schedules email checks using cron format.
- Processes emails in `text/plain` and `text/html` formats, as well as emails with `multipart/related` type.

## Installation

### Requirements

- Rust
- Cargo
- SQLite

### Installation Steps

1. Clone the repository:

```sh
git clone https://github.com/your-username/rust-email-checker.git
cd rust-email-checker
```

2. Install dependencies:

```sh
cargo build
```

3. Configure environment variables. Create a .env file in the project root and add the following parameters:

```env
IMAP_SERVER=imap.example.com
IMAP_PORT=993
IMAP_USERNAME=your_username
IMAP_PASSWORD=your_password
IMAP_FOLDER=INBOX
IMAP_SENT_FOLDER=Sent
CRON_SCHEDULE=0 */4 * * * * # Check every 4 hours
```

## Usage

Run the application:

```sh
cargo run
```

The application will automatically check emails according to the schedule specified in the `CRON_SCHEDULE` variable and store email information in the SQLite database.

## Project Structure

- `main.rs`: Entry point of the application. Starts the task scheduler.
- `imap_client.rs`: Module for connecting to the IMAP server.
- `email_processor.rs`: Module for processing emails.
- `database.rs`: Module for interacting with the SQLite database.
- `scheduler.rs`: Module for scheduling tasks using cron format.

## Cron Expression Examples

- `0 */4 * * * *`: Run the task every 4 hours.
- `0 0 * * * *`: Run the task daily at midnight.
- `0 0 * * 0 *`: Run the task weekly at midnight on Sunday.

## Support

If you have any questions or issues using the application, please create an issue in the GitHub repository.

## License

This project is licensed under the MIT License.
