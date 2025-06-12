use std::pin::Pin;
use std::future::Future;
use crate::{DynamicValue, ModelResult};

pub trait ModelManager<T: DynamicValue> {

    fn insert(
        &mut self,
        model_name: String,
        id: Option<String>,
        data: T,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>>;

    fn update(
        &mut self,
        model_name: String,
        id: String,
        data: T,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>>;

    fn get(
        &mut self,
        model_name: String,
        id: String,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>>;

    fn remove(
        &mut self,
        model_name: String,
        id: String,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>>;

    fn get_all(
        &mut self,
        model_name: String,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Vec<T>>> + Send + '_>>;
}