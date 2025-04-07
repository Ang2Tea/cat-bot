use std::{sync::Arc, time::Duration};

use endpoints::send_photo;
use teloxide::{Bot, dispatching::DefaultKey, dptree, prelude::Dispatcher};
use tokio::time::sleep;

use crate::contracts::{ChatCreateUC, ChatGetUC, ChatUpdateUC, PictureGetUC};

mod commands;
mod endpoints;
mod schemas;

type BotError = Box<dyn std::error::Error + Send + Sync>;

pub async fn run<P, C>(picture_uc: Arc<P>, chat_uc: Arc<C>) -> Dispatcher<Bot, BotError, DefaultKey>
where
    P: PictureGetUC + Send + Sync + 'static,
    C: ChatCreateUC + ChatGetUC + ChatUpdateUC + Send + Sync + 'static,
{
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schemas::schema::<P, C>())
        .dependencies(dptree::deps![picture_uc, chat_uc])
        .enable_ctrlc_handler()
        .build()
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
        if let Err(_) = chats {
            log::warn!("Failed to get chats");
            continue;
        }

        for (url, chat) in chats.unwrap() {
            let _ = send_photo(&bot, chat.chat_id, &url).await;
        }
    }
}
