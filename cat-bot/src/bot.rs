mod commands;
mod endpoints;
mod schemas;

use std::time::Duration;

use endpoints::send_photo;
use teloxide::{Bot, dptree, prelude::Dispatcher};
use tokio::time::sleep;

use cat_core::contracts::{ChatCreateUC, ChatUpdateUC, PictureGetUC};

type BotError = Box<dyn std::error::Error + Send + Sync>;

pub async fn run<P, CC, UC>(
    bot_token: String,
    delay_in_sec: u64,
    picture_uc: P,
    create_chat_uc: CC,
    update_chat_uc: UC,
) where
    P: PictureGetUC,
    CC: ChatCreateUC,
    UC: ChatUpdateUC,
{
    let bot = Bot::new(bot_token);

    let write_future = write_image(bot.clone(), delay_in_sec, picture_uc.clone());
    tokio::spawn(write_future);

    Dispatcher::builder(bot, schemas::schema::<P, CC, UC>())
        .dependencies(dptree::deps![picture_uc, create_chat_uc, update_chat_uc])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await
}

pub async fn write_image<P>(bot: Bot, delay_in_sec: u64, picture_helper: P)
where
    P: PictureGetUC,
{
    tracing::debug!("Starting image writer");

    loop {
        tracing::debug!("Writing image");

        let chats = picture_helper.get_picture_for_notification().await;

        let chats = match chats {
            Ok(c) => c,
            Err(_) => {
                tracing::warn!("Failed to get chats");
                continue;
            }
        };

        for (url, chat) in chats {
            let _ = send_photo(&bot, chat.chat_id, &url).await;
        }

        sleep(Duration::from_secs(delay_in_sec)).await;
    }
}
