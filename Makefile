# Имя бинарного файла
BINARY_NAME=email_service

# Установка зависимостей
install:
	cargo build
	cargo install --path .

# Запуск проекта
run: build
	@echo "Запуск приложения..."
	@./target/debug/$(BINARY_NAME)

# Сборка проекта
build:
	@echo "Сборка проекта..."
	cargo build

# Тестирование проекта
test:
	@echo "Запуск тестов..."
	cargo test

# Очистка сгенерированных файлов
clean:
	@echo "Очистка проекта..."
	cargo clean

# Запуск с использованием .env файла
run_with_env: build
	@echo "Запуск приложения с .env файлом..."
	@env $(shell cat .env | xargs) ./target/debug/$(BINARY_NAME)

# Помощь
help:
	@echo "Использование: make [target]"
	@echo ""
	@echo "Цели:"
	@echo "  install        Установка зависимостей"
	@echo "  build          Сборка проекта"
	@echo "  run            Запуск проекта"
	@echo "  run_with_env   Запуск проекта с использованием .env файла"
	@echo "  test           Запуск тестов"
	@echo "  clean          Очистка проекта"

# Установить цель по умолчанию
.DEFAULT_GOAL := help
