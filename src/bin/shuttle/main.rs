mod bot_service;
mod config_util;

use std::{collections::HashMap, sync::Arc};

use cat_bot::{
    adapters::{
        get_pictures::{CompositeApi, GetPictureEnum, TheCatsApi, TheDogsApi},
        repositories::postgres as db,
    },
    contracts::PictureType,
    usecases::{chat_uc::ChatUC, picture_uc::PictureUC},
};
use sqlx::PgPool;

use crate::bot_service::BotService;

type BotShuttleService = BotService<
    PictureUC<CompositeApi, db::ChatRepository>,
    ChatUC<db::ChatRepository>,
    ChatUC<db::ChatRepository>,
>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> Result<BotShuttleService, shuttle_runtime::Error> {
    tracing::debug!("Starting command bot...");

    let config = config_util::to_config(secrets.clone());

    sqlx::migrate!()
        .run(&pool)
        .await
        .unwrap();

    let chat_repository = Arc::new(db::ChatRepository::new(pool));

    let the_cats_api = Arc::new(GetPictureEnum::Cat(TheCatsApi::new(config.api_key.clone())));
    let the_dogs_api = Arc::new(GetPictureEnum::Dog(TheDogsApi::new(config.api_key.clone())));

    let mut apis = HashMap::new();
    apis.insert(PictureType::Cat, the_cats_api.clone());
    apis.insert(PictureType::Dog, the_dogs_api.clone());

    let the_apis = Arc::new(CompositeApi::new(apis));

    let chat_uc = Arc::new(ChatUC::new(chat_repository.clone()));
    let picture_uc = Arc::new(PictureUC::new(the_apis.clone(), chat_repository.clone()));

    Ok(BotService {
        config: config,
        picture_uc: picture_uc,
        create_chat_uc: chat_uc.clone(),
        update_chat_uc: chat_uc.clone(),
    })
}
