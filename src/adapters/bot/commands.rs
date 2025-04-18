use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
pub enum Command {
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