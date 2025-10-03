mod configs;

use std::{collections::HashMap, sync::Arc};

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use cat_bot::{
    contracts::PictureType,
    usecases::{chat_uc::ChatUC, picture_uc::PictureUC},
};
use cat_bot_adapters::get_pictures::{CompositeApi, GetPictureEnum, TheCatsApi, TheDogsApi};
use cat_bot_databases::in_memory;

use crate::configs::Config;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_path(".env");

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    tracing::debug!("Starting command bot...");

    let config = Config::init();

    let db = in_memory::init_db().await.unwrap();

    let chat_repository = Arc::new(in_memory::InMemoryChatRepository::new(db));

    let the_cats_api = Arc::new(GetPictureEnum::Cat(TheCatsApi::new(config.api_key.clone())));
    let the_dogs_api = Arc::new(GetPictureEnum::Dog(TheDogsApi::new(config.api_key.clone())));

    let mut apis = HashMap::new();
    apis.insert(PictureType::Cat, the_cats_api.clone());
    apis.insert(PictureType::Dog, the_dogs_api.clone());

    let the_apis = Arc::new(CompositeApi::new(apis));

    let chat_uc = Arc::new(ChatUC::new(chat_repository.clone()));
    let picture_uc = Arc::new(PictureUC::new(the_apis.clone(), chat_repository.clone()));

    cat_bot_adapters::run(
        config.bot_token,
        config.delay_in_sec,
        picture_uc,
        chat_uc.clone(),
        chat_uc.clone(),
    )
    .await
}
