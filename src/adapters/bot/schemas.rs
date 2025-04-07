use teloxide::{
    dispatching::{UpdateHandler, dialogue},
    dptree,
};

use crate::{
    adapters::bot::{commands::Command, endpoints},
    contracts::{ChatCreateUC, ChatGetUC, ChatUpdateUC, PictureGetUC},
};

use super::BotError;

pub fn schema<P, C>() -> UpdateHandler<BotError>
where
    P: PictureGetUC + Send + Sync + 'static,
    C: ChatCreateUC + ChatGetUC + ChatUpdateUC + Send + Sync + 'static,
{
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(endpoints::start::<C>))
        .branch(case![Command::GetMeCats].endpoint(endpoints::get_cat::<P>))
        .branch(case![Command::GetMeDogs].endpoint(endpoints::get_dog::<P>))
        .branch(case![Command::ChangePush].endpoint(endpoints::change_push::<C>))
        .branch(case![Command::Help].endpoint(endpoints::help));

    dialogue::enter::<_, _, _, _>().branch(command_handler)
}
