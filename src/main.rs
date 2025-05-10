use cat_bot::{
    adapters::{
        bot,
        get_pictures::{CompositeApi, GetPictureEnum, TheCatsApi, TheDogsApi},
        repositories::{self, ChatRepository},
    },
    configs,
    contracts::PictureType,
    usecases::{chat_uc::ChatUC, picture_uc::PictureUC},
};
use std::{collections::HashMap, sync::Arc};

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_path(".env");

    env_logger::init();
    log::debug!("Starting command bot...");

    let config = configs::init_config();

    let db = repositories::init_db(&config.db_url)
        .await
        .unwrap();

    let chat_repository = Arc::new(ChatRepository::new(db));

    let the_cats_api = Arc::new(GetPictureEnum::Cat(TheCatsApi::new(config.api_key.clone())));
    let the_dogs_api = Arc::new(GetPictureEnum::Dog(TheDogsApi::new(config.api_key.clone())));

    let mut apis = HashMap::new();
    apis.insert(PictureType::Cat, the_cats_api.clone());
    apis.insert(PictureType::Dog, the_dogs_api.clone());

    let the_apis = Arc::new(CompositeApi::new(apis));

    let chat_uc = Arc::new(ChatUC::new(chat_repository.clone()));
    let picture_uc = Arc::new(PictureUC::new(the_apis.clone(), chat_repository.clone()));

    bot::run(config, picture_uc, chat_uc.clone(), chat_uc.clone()).await
}
