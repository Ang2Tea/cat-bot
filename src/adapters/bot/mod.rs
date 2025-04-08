use std::{sync::Arc, time::Duration};

use endpoints::send_photo;
use teloxide::{Bot, dispatching::DefaultKey, dptree, prelude::Dispatcher};
use tokio::time::sleep;

use crate::contracts::{ChatCreateUC, ChatUpdateUC, PictureGetUC};

mod commands;
mod endpoints;
mod schemas;

type BotError = Box<dyn std::error::Error + Send + Sync>;

pub async fn run<P, CC, UC>(picture_uc: Arc<P>, create_chat_uc: Arc<CC>, update_chat_uc: Arc<UC>) -> Dispatcher<Bot, BotError, DefaultKey>
where
    P: PictureGetUC + Send + Sync + 'static,
    CC: ChatCreateUC + Send + Sync + 'static,
    UC: ChatUpdateUC + Send + Sync + 'static,
{
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schemas::schema::<P, CC, UC>())
        .dependencies(dptree::deps![picture_uc, create_chat_uc, update_chat_uc])
        .enable_ctrlc_handler()
        .build()
}

pub async fn write_image<P>(delay_in_sec: u64, picture_helper: Arc<P>)
where
    P: PictureGetUC,
{
    let bot = Bot::from_env();
    log::debug!("Starting image writer");

    loop {
        sleep(Duration::from_secs(delay_in_sec)).await;
        log::debug!("Writing image");

        let chats = picture_helper.get_picture_for_notification().await;
        if let Err(_) = chats {
            log::warn!("Failed to get chats");
            continue;
        }

        for (url, chat) in chats.unwrap() {
            let _ = send_photo(&bot, chat.chat_id, &url).await;
        }
    }
}
