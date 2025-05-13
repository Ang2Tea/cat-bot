# Cat Bot
[![en](https://img.shields.io/badge/lang-en-red.svg)](https://github.com/Ang2Tea/cat-bot/README.md)
[![ru](https://img.shields.io/badge/lang-ru-green.svg)](https://github.com/Ang2Tea/cat-bot/README.ru.md)

A Telegram bot written in Rust that sends cat and dog pictures on demand or periodically. Built with Teloxide, TheCatAPI, and TheDogAPI. Supports SQLite and PostgreSQL, and can be deployed via Docker.

## Features

- `/cat` – send a random cat picture  
- `/dog` – send a random dog picture  
- Periodic delivery of images at configurable intervals  
- Stores chat subscription info in SQLite or PostgreSQL  
- Easy configuration via environment variables  
- Dockerized for quick deployment  

## Prerequisites

- Rust ≥ 1.86
- Docker & Docker Compose (optional, for container deployment)
- Telegram bot token (obtain via @BotFather)
- API key for TheCatAPI & TheDogAPI (optional; without key, public endpoints used)

## Installation

1. **Clone the repository**  
```sh
git clone https://github.com/your-username/cat-bot.git
cd cat-bot
```

2. Create & configure .env
Copy `.env.example` (or use the template below) and set your values:

## Build & Run
Locally:
```sh
cargo run --release
```

With Docker Compose:
```sh
docker-compose up --build
```
## Usage
Open a chat with your bot (or add it to a group).

Send `/start`, after `/cat` or `/dog` to receive an image.

The bot will also send images automatically every `DELAY_IN_SEC` seconds to subscribed chats.