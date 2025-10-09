mod bot_service;
mod config_util;

use std::{collections::HashMap, sync::Arc};

use cat_core::{
    contracts::PictureType,
    usecases::{chat_uc::ChatUC, picture_uc::PictureUC},
};
use cat_sqlx_repo::{self, postgres::PostgresRepository};
use cat_adapters::get_pictures::{CompositeApi, GetPictureEnum, TheCatsApi, TheDogsApi};

use crate::{bot_service::BotService, config_util::Config};

type BotShuttleService = BotService<
    PictureUC<CompositeApi, PostgresRepository>,
    ChatUC<PostgresRepository>,
    ChatUC<PostgresRepository>,
>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db_url: String,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> Result<BotShuttleService, shuttle_runtime::Error> {
    tracing::debug!("Starting command bot...");

    let config = Config::from(secrets.clone());

    cat_sqlx_repo::postgres::migrate(&db_url)
        .await
        .map_err(|e| shuttle_runtime::Error::Database(e))?;

    let chat_repository = Arc::new(PostgresRepository::new(&db_url).await);

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
