# Cat Bot
[![en](https://img.shields.io/badge/lang-en-red.svg)](https://github.com/Ang2Tea/cat-bot/README.md)
[![ru](https://img.shields.io/badge/lang-ru-green.svg)](https://github.com/Ang2Tea/cat-bot/README.ru.md)

## Возможности

- `/cat` – отправить случайное фото кота  
- `/dog` – отправить случайное фото собаки  
- Периодическая отправка изображений с настраиваемым интервалом  
- Хранение подписок чатов в SQLite или PostgreSQL  
- Легкая настройка через переменные окружения  
- Docker-образ для быстрого деплоя  

## Требования

- Rust ≥ 1.86;  
- Docker & Docker Compose (для контейнерного развёртывания);  
- Токен Telegram-бота (от @BotFather);  
- API-ключ для TheCatAPI и TheDogAPI.

## Установка

1. **Клонировать репозиторий**  
```sh
git clone https://github.com/your-username/cat-bot.git
cd cat-bot
```

2. **Конфигурация .env**:
Для настройки конфигурации скопируйте и настройте `.env.example`

## Сборка и запуск
Локально:
```sh
cargo run --release
```

Через Docker Compose:
```sh
docker-compose up --build
```

## Использование
Откройте чат с ботом (или добавьте в группу).

Отправьте `/start`, потом `/cat` или `/dog` для получения изображения.

Бот также будет автоматически присылать картинки каждые `DELAY_IN_SEC` секунд в подписанные чаты.