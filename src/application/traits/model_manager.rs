use crate::{DynamicValue, ModelResult};

pub trait ModelManager<T: DynamicValue> {
    fn insert(&mut self, model_name: String, id: Option<String>, data: T) -> ModelResult<T>;
    fn update(&mut self, model_name: String, id: String, data: T) -> ModelResult<T>;
    fn get(&mut self, model_name: String, id: String) -> ModelResult<T>;
    fn remove(&mut self, model_name: String, id: String) -> ModelResult<T>;
    fn get_all(&mut self, model_name: String) -> ModelResult<Vec<T>>;
    fn get_by_path(&mut self, model_name: String, id: String, path: String) -> ModelResult<Option<T>>;
    fn find_by_path_exists(&mut self, model_name: String, path: String) -> ModelResult<Vec<T>>;
    fn find_by_path_value(&mut self, model_name: String, path: String, expected_value: T) -> ModelResult<Vec<T>>;
}