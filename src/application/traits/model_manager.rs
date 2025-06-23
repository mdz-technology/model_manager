use crate::{DynamicValue, ModelResult};

pub trait ModelManager<T: DynamicValue>: Send + Sync {
    fn insert(&self, model_name: &str, id: Option<&str>, data: T) -> ModelResult<T>;
    fn update(&self, model_name: &str, id: &str, data: T) -> ModelResult<T>;
    fn get(&self, model_name: &str, id: &str) -> ModelResult<T>;
    fn remove(&self, model_name: &str, id: &str) -> ModelResult<T>;

    fn iter_all(&self, model_name: &str) -> ModelResult<Box<dyn ExactSizeIterator<Item = ModelResult<T>> + Send>>;
    fn get_all(&self, model_name: &str) -> ModelResult<Vec<T>> {
        self.iter_all(model_name)?
            .collect::<Result<Vec<_>, _>>()
    }

    fn get_by_path(&self, model_name: &str, id: &str, path: &str) -> ModelResult<Option<T>>;

    fn find_by_path_exists(&self, model_name: &str, path: &str) -> ModelResult<Vec<T>>;
    fn find_by_path_value(&self, model_name: &str, path: &str, expected_value: T) -> ModelResult<Vec<T>>;

    fn count(&self, model_name: &str) -> ModelResult<usize>;
    fn exists(&self, model_name: &str, id: &str) -> ModelResult<bool>;
}