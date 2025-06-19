use std::pin::Pin;
use std::future::Future;
use super::super::models::errors::ModelResult;

pub trait DynamicValue: Clone + Send + Sync + Unpin + 'static {

    fn new_object() -> Self;
    fn new_array() -> Self;
    fn from_str(s: &str) -> Self;
    fn from_number(n: f64) -> ModelResult<Self> where Self: Sized;
    fn from_bool(b: bool) -> Self;
    fn is_object(&self) -> bool;
    fn is_array(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn get_type(&self) -> String;
    fn as_str(&self) -> Option<String>;
    fn as_number(&self) -> Option<f64>;
    fn as_bool(&self) -> Option<bool>;
    fn get<'a>(
        &'a self,
        key: &'a str
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<Self>>> + Send + 'a>>;
    fn set<'a>(
        &'a mut self,
        key: &'a str,
        value: Self
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>>;
    fn push<'a>(
        &'a mut self,
        value: Self
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>>;
    fn as_array<'a>(
        &'a self
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<Vec<Self>>>> + Send + 'a>>;
    fn to_string(&self) -> String;
    fn get_by_path<'a>(
        &'a self,
        path: &'a str
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<Self>>> + Send + 'a>>;

    fn has_path<'a>(
        &'a self,
        path: &'a str
    ) -> Pin<Box<dyn Future<Output = ModelResult<bool>> + Send + 'a>>;

    fn set_by_path<'a>(
        &'a mut self,
        path: &'a str,
        value: Self
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>>;

    fn deep_clone<'a>(
        &'a self
    ) -> Pin<Box<dyn Future<Output = ModelResult<Self>> + Send + 'a>>;
    
}