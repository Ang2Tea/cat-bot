use std::sync::Arc;

use reqwest::Url;
use teloxide::{
    Bot,
    prelude::Requester,
    types::{ChatId, InputFile, Message},
    utils::command::BotCommands,
};

use crate::{
    contracts::{ChangeChatDto, ChatCreateUC, ChatUpdateUC, PictureGetUC, PictureType},
    shared::ErrorKind,
};

use super::commands::Command;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn send_photo(bot: &Bot, chat_id: i64, url: &str) -> HandlerResult {
    let url = Url::parse(url);
    if let Err(err) = url {
        log::error!("Failed to parse url: {}", err);
        return Ok(());
    }

    let input_file = InputFile::url(url.unwrap());

    bot.send_photo(ChatId(chat_id), input_file).await?;
    Ok(())
}

async fn get_picture<P>(
    bot: Bot,
    msg: Message,
    picture_type: PictureType,
    picture_helper: Arc<P>,
) -> HandlerResult
where
    P: PictureGetUC,
{
    let raw_cats = picture_helper.get_picture(Some(picture_type)).await;

    match raw_cats {
        Ok(cats) => send_photo(&bot, msg.chat.id.0, &cats).await.map(|_| ()),
        Err(err) => {
            let log_message = match err {
                ErrorKind::NotFound => String::from("Not found"),
                ErrorKind::Other(message) => message,
            };

            log::error!("{}", log_message);

            bot.send_message(msg.chat.id, "Что то пошло не так").await?;
            Ok(())
        }
    }
}

pub async fn start<R>(bot: Bot, chat_helper: Arc<R>, msg: Message) -> HandlerResult
where
    R: ChatCreateUC,
{
    let chat_name = msg.chat.username().map(|name| name.to_string());
    let chat_title = msg.chat.title().map(|name| name.to_string());

    let dto = ChangeChatDto {
        chat_id: msg.chat.id.0,
        name: chat_name,
        title: chat_title,
        enable_push: false,
    };

    let result = chat_helper.create(dto).await;

    if result.is_err() {
        log::error!("Failed to create user");
    }

    bot.send_message(msg.chat.id, "Добро пожаловать").await?;
    Ok(())
}

pub async fn get_dog<P>(bot: Bot, msg: Message, picture_helper: Arc<P>) -> HandlerResult
where
    P: PictureGetUC,
{
    get_picture(bot, msg, PictureType::Dog, picture_helper).await
}

pub async fn get_cat<P>(bot: Bot, msg: Message, picture_helper: Arc<P>) -> HandlerResult
where
    P: PictureGetUC,
{
    get_picture(bot, msg, PictureType::Cat, picture_helper).await
}


pub async fn change_push<T>(bot: Bot, msg: Message, chat_helper: Arc<T>) -> HandlerResult
where
    T: ChatUpdateUC,
{
    let result = chat_helper.change_push(msg.chat.id.0).await;
    if result.is_err() {
        log::error!("Failed to create user");
        return Ok(());
    }

    let message = if result.unwrap() {
        "Уведомления включены"
    } else {
        "Уведомления выключены"
    };

    bot.send_message(msg.chat.id, message).await?;
    Ok(())
}

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}
