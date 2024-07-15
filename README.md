
# Rust Email Checker

Rust Email Checker - это приложение для проверки электронной почты через IMAP и сохранения информации о письмах в базу данных SQLite. Приложение использует расписание в формате cron для периодической проверки почты.

## Функциональные возможности

- Подключение к IMAP серверу для проверки входящих и отправленных писем.
- Сохранение информации о письмах (отправитель, получатель, тема, тело) в базу данных SQLite.
- Расписание проверок почты с использованием формата cron.
- Обработка писем в формате `text/plain` и `text/html`, а также писем с типом `multipart/related`.

## Установка

### Требования

- Rust
- Cargo
- SQLite

### Шаги установки

1. Клонируйте репозиторий:

```sh
git clone https://github.com/your-username/rust-email-checker.git
cd rust-email-checker
```

2. Установите зависимости:

```sh
cargo build
```

3. Настройте переменные окружения. Создайте файл `.env` в корне проекта и добавьте следующие параметры:

```env
IMAP_SERVER=imap.example.com
IMAP_PORT=993
IMAP_USERNAME=your_username
IMAP_PASSWORD=your_password
IMAP_FOLDER=INBOX
IMAP_SENT_FOLDER=Sent
CRON_SCHEDULE=0 */4 * * * * # Проверка каждые 4 часа
```

4. Создайте папку `migrations` и добавьте в нее миграцию для создания таблицы `emails`.

### `migrations/V1__create_emails_table.sql`

```sql
CREATE TABLE emails (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sender TEXT NOT NULL,
    recipient TEXT NOT NULL,
    subject TEXT,
    body TEXT,
    message_id TEXT NOT NULL,
    folder TEXT NOT NULL,
    UNIQUE(message_id, folder)
);
```

## Использование

Запустите приложение:

```sh
cargo run
```

Приложение автоматически будет проверять почту в соответствии с расписанием, указанным в переменной `CRON_SCHEDULE`, и сохранять информацию о письмах в базу данных SQLite.

## Структура проекта

- `main.rs`: Точка входа в приложение. Запускает планировщик задач.
- `imap_client.rs`: Модуль для подключения к IMAP серверу.
- `email_processor.rs`: Модуль для обработки писем.
- `database.rs`: Модуль для взаимодействия с базой данных SQLite.
- `scheduler.rs`: Модуль для планирования задач с использованием формата cron.

## Примеры cron выражений

- `0 */4 * * * *`: Запуск задачи каждые 4 часа.
- `0 0 * * * *`: Запуск задачи ежедневно в полночь.
- `0 0 * * 0 *`: Запуск задачи еженедельно в полночь в воскресенье.

## Поддержка

Если у вас возникли вопросы или проблемы с использованием приложения, пожалуйста, создайте issue в репозитории на GitHub.

## Лицензия

Этот проект лицензирован под лицензией MIT.
