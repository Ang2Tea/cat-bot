use std::sync::Arc;

use cat_bot::{
    adapters::bot,
    configs::Config,
    contracts::{ChatCreateUC, ChatUpdateUC, PictureGetUC},
};

pub struct BotService<P, CC, UC>
where
    P: PictureGetUC,
    CC: ChatCreateUC,
    UC: ChatUpdateUC,
{
    pub config: Config,
    pub picture_uc: Arc<P>,
    pub create_chat_uc: Arc<CC>,
    pub update_chat_uc: Arc<UC>,
}

#[shuttle_runtime::async_trait]
impl<P, CC, UC> shuttle_runtime::Service for BotService<P, CC, UC>
where
    P: PictureGetUC,
    CC: ChatCreateUC,
    UC: ChatUpdateUC,
{
    async fn bind(self, _: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        bot::run(
            self.config,
            self.picture_uc,
            self.create_chat_uc,
            self.update_chat_uc,
        )
        .await;

        Ok(())
    }
}
