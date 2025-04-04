use crate::models::ChatDto;


pub trait ChatRepository {
    fn create_user(&self, dto: ChatDto) -> impl Future<Output = Option<()>> + Send;
    fn get_list(&self) -> impl Future<Output = Option<Vec<ChatDto>>> + Send;
    fn get_by_id(&self, id: i64) -> impl Future<Output = Option<ChatDto>> + Send;
    fn get_list_for_push(&self) -> impl Future<Output = Option<Vec<ChatDto>>> + Send;
    fn update_user(&self, id: i64, dto: ChatDto) -> impl Future<Output = Option<()>> + Send;
}