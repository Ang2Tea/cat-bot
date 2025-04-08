use teloxide::{dispatching::{HandlerExt, UpdateFilterExt, UpdateHandler}, dptree, types::Update};

use crate::{
    adapters::bot::{commands::Command, endpoints},
    contracts::{ChatCreateUC, ChatUpdateUC, PictureGetUC},
};

use super::BotError;

pub fn schema<P, CC, UC>() -> UpdateHandler<BotError>
where
    P: PictureGetUC + Send + Sync + 'static,
    CC: ChatCreateUC + Send + Sync + 'static,
    UC: ChatUpdateUC + Send + Sync + 'static,
{
    use dptree::case;

    let command_handler = Update::filter_message()
        .filter_command::<Command>()
        .branch(case![Command::Help].endpoint(endpoints::help))
        .branch(case![Command::Start].endpoint(endpoints::start::<CC>))
        .branch(case![Command::GetMeCats].endpoint(endpoints::get_cat::<P>))
        .branch(case![Command::GetMeDogs].endpoint(endpoints::get_dog::<P>))
        .branch(case![Command::ChangePush].endpoint(endpoints::change_push::<UC>));

    // let command_handler = teloxide::filter_command::<Command, _>()
    //     .branch(case![Command::Help].endpoint(endpoints::help))
    //     .branch(case![Command::Start].endpoint(endpoints::start::<CC>))
    //     .branch(case![Command::GetMeCats].endpoint(endpoints::get_cat::<P>))
    //     .branch(case![Command::GetMeDogs].endpoint(endpoints::get_dog::<P>))
    //     .branch(case![Command::ChangePush].endpoint(endpoints::change_push::<UC>));

    command_handler
}
