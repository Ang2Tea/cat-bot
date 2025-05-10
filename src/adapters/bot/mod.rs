use std::{sync::Arc, time::Duration};

use endpoints::send_photo;
use teloxide::{Bot, dptree, prelude::Dispatcher};
use tokio::time::sleep;

use crate::{
    configs::Config,
    contracts::{ChatCreateUC, ChatUpdateUC, PictureGetUC},
};

mod commands;
mod endpoints;
mod schemas;

type BotError = Box<dyn std::error::Error + Send + Sync>;

pub async fn run<P, CC, UC>(
    config: Config,
    picture_uc: Arc<P>,
    create_chat_uc: Arc<CC>,
    update_chat_uc: Arc<UC>,
) where
    P: PictureGetUC,
    CC: ChatCreateUC,
    UC: ChatUpdateUC,
{
    let bot = Bot::from_env();

    let write_future = write_image(bot.clone(), config.delay_in_sec, picture_uc.clone());
    tokio::spawn(write_future);

    Dispatcher::builder(bot, schemas::schema::<P, CC, UC>())
        .dependencies(dptree::deps![picture_uc, create_chat_uc, update_chat_uc])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await
}

pub async fn write_image<P>(bot: Bot, delay_in_sec: u64, picture_helper: Arc<P>)
where
    P: PictureGetUC,
{
    log::debug!("Starting image writer");

    loop {
        sleep(Duration::from_secs(delay_in_sec)).await;
        log::debug!("Writing image");

        let chats = picture_helper.get_picture_for_notification().await;
        if chats.is_err() {
            log::warn!("Failed to get chats");
            continue;
        }

        for (url, chat) in chats.unwrap() {
            let _ = send_photo(&bot, chat.chat_id, &url).await;
        }
    }
}
