use std::sync::Arc;

use cat_core::contracts::{ChatCreateUC, ChatUpdateUC, PictureGetUC};

use crate::config_util::Config;

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
        cat_bot::run(
            self.config.bot_token,
            self.config.delay_in_sec,
            self.picture_uc,
            self.create_chat_uc,
            self.update_chat_uc,
        )
        .await;

        Ok(())
    }
}
