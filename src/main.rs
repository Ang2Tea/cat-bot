use cat_bot::{
    interface::GetCats,
    models::ChatDto,
    repositories::ChatRepository,
    sql_lite_db::{self, SqlLiteChatsRepository},
    the_cats_api::TheCatsApi,
};
use reqwest::Url;
use sqlx::SqlitePool;
use std::{sync::Arc, time::Duration};
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

    let api_key = std::env::var("THE_CAT_API_KEY").expect("No API key found");
    let db_urn = std::env::var("DATABASE_URL").expect("No database url found");

    sql_lite_db::init_db(&db_urn).await;

    let db = SqlitePool::connect(&db_urn).await.unwrap();

    let chat_repository = Arc::new(SqlLiteChatsRepository::new(db));
    let the_cat_api = Arc::new(TheCatsApi::new(api_key));

    let bot = Bot::from_env();
    bot.set_my_commands(Command::bot_commands()).await.unwrap();

    let write_future = write_image(bot.clone(), the_cat_api.clone(), chat_repository.clone());
    tokio::spawn(write_future);

    Command::repl(bot, move |bot, msg, cmd| {
        answer(bot, msg, cmd, the_cat_api.clone(), chat_repository.clone())
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

async fn write_image<C, R>(bot: Bot, get_cats: Arc<C>, chat_repository: Arc<R>)
where
    C: GetCats,
    R: ChatRepository,
{
    log::debug!("Starting image writer");

    loop {
        sleep(Duration::from_secs(1 * 60)).await;
        log::debug!("Writing image");

        let users = chat_repository.get_list_for_push().await;
        if let None = users {
            log::warn!("Failed to get users");
            continue;
        }

        for user in users.unwrap() {
            let raw_cats = get_cats.get_random_cats().await;

            if let Some(cats) = raw_cats {
                for cat in cats {
                    let r = send_photo(&bot, user.chat_id, &cat.url).await;
                    if let Err(err) = r {
                        log::warn!("Failed to send photo: {}", err);
                    }
                }
            }
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
    #[command(description = "Get cat")]
    ChangePush,
    #[command(description = "Get help info")]
    Help,
}

async fn start<T>(bot: Bot, msg: Message, chat_repository: Arc<T>) -> ResponseResult<()>
where
    T: ChatRepository,
{
    let chat_name = msg.chat.username().map(|name| name.to_string());
    let chat_title = msg.chat.title().map(|name| name.to_string());

    let dto = ChatDto {
        chat_id: msg.chat.id.0,
        name: chat_name,
        title: chat_title,
        enable_push: false,
    };

    let result = chat_repository.create_user(dto).await;

    if let None = result {
        log::error!("Failed to create user");
    }

    bot.send_message(msg.chat.id, "Добро пожаловать")
        .await
        .map(|_| ())
}

async fn get_cats<T>(bot: Bot, msg: Message, get_cats: Arc<T>) -> ResponseResult<()>
where
    T: GetCats,
{
    let raw_cats = get_cats.get_random_cats().await;

    if let Some(cats) = raw_cats {
        for cat in cats {
            let _ = send_photo(&bot, msg.chat.id.0, &cat.url).await;
        }
    }

    Ok(())
}

async fn update_user<T>(bot: Bot, msg: Message, chat_repository: Arc<T>) -> ResponseResult<()>
where
    T: ChatRepository,
{
    let user = chat_repository.get_by_id(msg.chat.id.0).await;
    if let None = user {
        log::error!("Failed to get user");
        return Ok(());
    }

    let chat_name = msg.chat.username().map(|name| name.to_string());
    let chat_title = msg.chat.title().map(|name| name.to_string());

    let user = user.unwrap();
    
    let current_push = !user.enable_push;

    let dto = ChatDto {
        chat_id: msg.chat.id.0,
        name: chat_name,
        title: chat_title,
        enable_push: current_push,
    };

    let result = chat_repository.update_user(user.chat_id, dto).await;
    if let None = result {
        log::error!("Failed to create user");
    }

    match current_push {
        true =>  bot.send_message(msg.chat.id, "Уведомления включены").await.map(|_| ()),
        false => bot.send_message(msg.chat.id, "Уведомления выключены").await.map(|_| ()),
    }
}

async fn answer<T, R>(
    bot: Bot,
    msg: Message,
    cmd: Command,
    get_cats_helper: Arc<T>,
    chat_repository: Arc<R>,
) -> ResponseResult<()>
where
    T: GetCats,
    R: ChatRepository,
{
    match cmd {
        Command::Start => start(bot, msg, chat_repository).await?,
        Command::GetMeCats => get_cats(bot, msg, get_cats_helper).await?,
        Command::ChangePush => update_user(bot, msg, chat_repository).await?,
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
    };

    Ok(())
}
