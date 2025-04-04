use crate::models::CatDto;

pub trait GetCats {
    fn get_random_cats(&self) -> impl Future<Output = Option<Vec<CatDto>>>;
}
