use cat_bot::{
    adapters::{
        bot,
        get_pictures::{CompositeApi, TheCatsApi, TheDogsApi},
        repositories::sqlite_chat_repository::SqlLiteChatRepository,
    },
    contracts::{GetPictures, PictureType},
    usecases::{chat_uc::ChatUC, picture_uc::PictureUC},
};
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc};

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_path(".env");

    env_logger::init();
    log::debug!("Starting command bot...");

    let api_key = std::env::var("THE_API_KEY").expect("No API key found");
    let db_urn = std::env::var("DATABASE_URL").expect("No database url found");

    let delay_in_sec = {
        let delay = std::env::var("DELAY_IN_SEC").expect("No delay found");
        delay.parse::<u64>().unwrap()
    };

    cat_bot::adapters::repositories::init_db(&db_urn)
        .await
        .unwrap();

    let db = SqlitePool::connect(&db_urn).await.unwrap();

    let chat_repository = Arc::new(SqlLiteChatRepository::new(db));

    let the_cats_api: Arc<dyn GetPictures + Send + Sync + 'static> =
        Arc::new(TheCatsApi::new(api_key.clone()));
    let the_dogs_api: Arc<dyn GetPictures + Send + Sync + 'static> =
        Arc::new(TheDogsApi::new(api_key.clone()));

    let mut apis = HashMap::new();
    apis.insert(PictureType::Cat, the_cats_api.clone());
    apis.insert(PictureType::Dog, the_dogs_api.clone());

    let the_apis = Arc::new(CompositeApi::new(apis));

    let chat_uc = Arc::new(ChatUC::new(chat_repository.clone()));
    let picture_uc = Arc::new(PictureUC::new(the_apis.clone(), chat_repository.clone()));

    let write_future = bot::write_image(delay_in_sec, picture_uc.clone());
    tokio::spawn(write_future);

    bot::run(picture_uc, chat_uc.clone(), chat_uc.clone()).await.dispatch().await;
}
