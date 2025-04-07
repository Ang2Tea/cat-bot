use cat_bot::{
    adapters::{
        get_pictures::{GetPicturesBox, the_cats_api::TheCatsApi, the_dogs_api::TheDogsApi},
        repositories::sqlite_chat_repository::SqlLiteChatRepository,
    },
    contracts::{
        ChatCreateUC, ChatGetUC, ChatUpdateUC, GetPictures, PictureGetUC,
        models::{ChangeChatDto, PictureType},
    },
    usecases::{chat_uc::ChatUC, picture_uc::PictureUC},
};
use reqwest::Url;
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc, time::Duration};
use teloxide::{
    Bot,
    prelude::{Requester, ResponseResult},
    repls::CommandReplExt,
    types::{ChatId, InputFile, Message},
    utils::command::BotCommands,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let _ = dotenvy::from_path(".env");

    env_logger::init();
    log::debug!("Starting command bot...");

    let api_key = std::env::var("THE_API_KEY").expect("No API key found");
    let db_urn = std::env::var("DATABASE_URL").expect("No database url found");

    let delay_in_sec = {
        let delay = std::env::var("DELAY_IN_SEC").expect("No delay found");
        delay.parse::<u64>().unwrap()
    };

    cat_bot::adapters::repositories::init_db(&db_urn)
        .await
        .unwrap();

    let db = SqlitePool::connect(&db_urn).await.unwrap();

    let chat_repository = Arc::new(SqlLiteChatRepository::new(db));

    let the_cats_api: Arc<dyn GetPictures + Send + Sync + 'static> =
        Arc::new(TheCatsApi::new(api_key.clone()));
    let the_dogs_api: Arc<dyn GetPictures + Send + Sync + 'static> =
        Arc::new(TheDogsApi::new(api_key.clone()));

    let mut apis = HashMap::new();
    apis.insert(PictureType::Cat, the_cats_api.clone());
    apis.insert(PictureType::Dog, the_dogs_api.clone());

    let the_apis = Arc::new(GetPicturesBox::new(apis));

    let chat_uc = Arc::new(ChatUC::new(chat_repository.clone()));
    let picture_uc = Arc::new(PictureUC::new(the_apis.clone(), chat_repository.clone()));

    let bot = Bot::from_env();
    bot.set_my_commands(Command::bot_commands()).await.unwrap();

    let write_future = write_image(bot.clone(), delay_in_sec, picture_uc.clone());
    tokio::spawn(write_future);

    Command::repl(bot, move |bot, msg, cmd| {
        answer(bot, msg, cmd, picture_uc.clone(), chat_uc.clone())
    })
    .await;
}

async fn send_photo(bot: &Bot, chat_id: i64, url: &str) -> ResponseResult<()> {
    let url = Url::parse(url);
    if let Err(err) = url {
        log::error!("Failed to parse url: {}", err);
        return Ok(());
    }

    let input_file = InputFile::url(url.unwrap());

    let result = bot.send_photo(ChatId(chat_id), input_file).await;
    if let Err(err) = result {
        log::error!("Failed to send photo: {}", err);
    }

    Ok(())
}

async fn write_image<P>(bot: Bot, delay_in_sec: u64, picture_helper: Arc<P>)
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

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Get cat")]
    GetMeCats,
    #[command(description = "Get dog")]
    GetMeDogs,
    #[command(description = "Change push mod")]
    ChangePush,
    #[command(description = "Get help info")]
    Help,
}

async fn start<R>(bot: Bot, msg: Message, chat_repository: Arc<R>) -> ResponseResult<()>
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

    let result = chat_repository.create(dto).await;

    if let Err(_) = result {
        log::error!("Failed to create user");
    }

    bot.send_message(msg.chat.id, "Добро пожаловать")
        .await
        .map(|_| ())
}

async fn get_picture<P>(
    bot: Bot,
    msg: Message,
    picture_type: PictureType,
    picture_helper: Arc<P>,
) -> ResponseResult<()>
where
    P: PictureGetUC,
{
    let raw_cats = picture_helper.get_picture(Some(picture_type)).await;

    if let Ok(picture_url) = raw_cats {
        let _ = send_photo(&bot, msg.chat.id.0, &picture_url).await;
    }

    Ok(())
}

async fn update_user<T>(bot: Bot, msg: Message, chat_helper: Arc<T>) -> ResponseResult<()>
where
    T: ChatGetUC + ChatUpdateUC,
{
    let user = chat_helper.get_by_id(msg.chat.id.0).await;
    if let Err(_) = user {
        log::error!("Failed to get user");
        return Ok(());
    }

    let chat_name = msg.chat.username().map(|name| name.to_string());
    let chat_title = msg.chat.title().map(|name| name.to_string());

    let user = user.unwrap();

    let current_push = !user.enable_push;

    let dto = ChangeChatDto {
        chat_id: msg.chat.id.0,
        name: chat_name,
        title: chat_title,
        enable_push: current_push,
    };

    let result = chat_helper.update(dto).await;
    if let Err(_) = result {
        log::error!("Failed to create user");
    }

    match current_push {
        true => bot
            .send_message(msg.chat.id, "Уведомления включены")
            .await
            .map(|_| ()),
        false => bot
            .send_message(msg.chat.id, "Уведомления выключены")
            .await
            .map(|_| ()),
    }
}

async fn answer<P, C>(
    bot: Bot,
    msg: Message,
    cmd: Command,
    picture_helper: Arc<P>,
    chat_helper: Arc<C>,
) -> ResponseResult<()>
where
    P: PictureGetUC,
    C: ChatCreateUC + ChatGetUC + ChatUpdateUC,
{
    match cmd {
        Command::Start => start(bot, msg, chat_helper).await?,
        Command::GetMeCats => get_picture(bot, msg, PictureType::Cat, picture_helper).await?,
        Command::GetMeDogs => get_picture(bot, msg, PictureType::Dog, picture_helper).await?,
        Command::ChangePush => update_user(bot, msg, chat_helper).await?,
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
    };

    Ok(())
}
