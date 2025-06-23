use crate::ModelResult;

pub trait PathNavigation: Clone {
    fn get_by_path(&self, path: &str) -> ModelResult<Option<Self>>;
    fn has_path(&self, path: &str) -> ModelResult<bool>;
    fn set_by_path(&mut self, path: &str, value: Self) -> ModelResult<()>;
}