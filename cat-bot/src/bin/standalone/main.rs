mod configs;

use std::{collections::HashMap, sync::Arc};

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use cat_adapters::get_pictures::{CompositeApi, GetPictureEnum, TheCatsApi, TheDogsApi};
use cat_core::{
    contracts::PictureType,
    usecases::{chat_uc::ChatUC, picture_uc::PictureUC},
};

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

    cat_diesel_repo::postgres::run_migrations(&config.db_url).unwrap();

    let chat_repository =
        cat_diesel_repo::postgres::PostgresRepository::try_new(config.db_url).unwrap();

    let the_cats_api = GetPictureEnum::Cat(TheCatsApi::try_new(&config.api_key).unwrap());
    let the_dogs_api = GetPictureEnum::Dog(TheDogsApi::try_new(&config.api_key).unwrap());

    let apis = {
        let mut apis = HashMap::new();
        apis.insert(PictureType::Cat, the_cats_api.clone());
        apis.insert(PictureType::Dog, the_dogs_api.clone());

        Arc::new(apis)
    };

    let the_apis = CompositeApi::new(apis);

    let chat_uc = ChatUC::new(chat_repository.clone());
    let picture_uc = PictureUC::new(the_apis.clone(), chat_repository.clone());

    cat_bot::run(
        config.bot_token,
        config.delay_in_sec,
        picture_uc,
        chat_uc.clone(),
        chat_uc.clone(),
    )
    .await
}
