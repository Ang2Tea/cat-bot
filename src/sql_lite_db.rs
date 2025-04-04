use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

use crate::{models::ChatDto, repositories::ChatRepository};

pub async fn init_db(db_urn: &str) {
    if !Sqlite::database_exists(db_urn).await.unwrap_or(false) {
        match Sqlite::create_database(db_urn).await {
            Ok(_) => log::info!("Create db success {}", db_urn),
            Err(error) => log::error!("error: {}", error),
        }
    } else {
        log::debug!("Database already exists");
    }

    let db = SqlitePool::connect(db_urn).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => log::debug!("Migration success"),
        Err(error) => {
            log::error!("error: {}", error);
        }
    }
}

pub struct SqlLiteChatsRepository {
    pub db: SqlitePool,
}

impl SqlLiteChatsRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl ChatRepository for SqlLiteChatsRepository {
    async fn create_user(&self, dto: ChatDto) -> Option<()> {
        let result = sqlx::query("INSERT INTO chats (chat_id, name, title) VALUES (?, ?, ?);")
            .bind(dto.chat_id)
            .bind(dto.name)
            .bind(dto.title)
            .execute(&self.db)
            .await;

        match result {
            Ok(_) => Some(()),
            Err(err) => {
                log::error!("Failed to create user: {}", err);
                None
            }
        }
    }

    async fn get_list(&self) -> Option<Vec<ChatDto>> {
        let result = sqlx::query_as::<_, ChatDto>("SELECT * FROM chats;")
            .fetch_all(&self.db)
            .await;

        match result {
            Ok(r) => Some(r),
            Err(err) => {
                log::error!("Failed to get chats list: {}", err);
                None
            }
        }
    }

    async fn get_list_for_push(&self) -> Option<Vec<ChatDto>> {
        let result =
            sqlx::query_as::<_, ChatDto>("SELECT * FROM chats WHERE enable_push;")
                .fetch_all(&self.db)
                .await;

        match result {
            Ok(r) => Some(r),
            Err(err) => {
                log::error!("Failed to get chats for push : {}", err);
                None
            }
        }
    }

    async fn update_user(&self, id: i64, dto: ChatDto) -> Option<()> {
        let result =
            sqlx::query("UPDATE chats SET name = ?, enable_push = ?, title = ? WHERE chat_id = ?;")
                .bind(dto.name)
                .bind(dto.enable_push)
                .bind(dto.title)
                .bind(id)
                .execute(&self.db)
                .await;

        match result {
            Ok(_) => Some(()),
            Err(err) => {
                log::error!("Failed to update chat by id {}: {}", id, err);
                None
            }
        }
    }

    async fn get_by_id(&self, id: i64) -> Option<ChatDto> {
        let result =
            sqlx::query_as::<_, ChatDto>("SELECT * FROM chats WHERE chat_id = ?;")
                .bind(id)
                .fetch_one(&self.db)
                .await;

        match result {
            Ok(r) => Some(r),
            Err(err) => {
                log::error!("Failed to get chat by id {}: {}", id, err);
                None
            }
        }
    }
}
