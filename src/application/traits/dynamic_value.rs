use std::pin::Pin;
use std::future::Future;
use super::super::models::errors::ModelResult;

pub trait DynamicValue: Clone + Send + Sync + 'static {
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
    fn get(&self, key: &str) -> ModelResult<Option<Self>>;
    fn set(&mut self, key: &str, value: Self) -> ModelResult<()>;
    fn push(&mut self, value: Self) -> ModelResult<()>;
    fn as_array(&self) -> ModelResult<Option<Vec<Self>>>;
    fn to_string(&self) -> String;
    fn get_by_path(&self, path: &str) -> ModelResult<Option<Self>>;
    fn has_path(&self, path: &str) -> ModelResult<bool>;
    fn set_by_path(&mut self, path: &str, value: Self) -> ModelResult<()>;
    fn deep_clone(&self) -> ModelResult<Self>;
    fn has_property(&self, key: &str) -> ModelResult<bool>;
    fn get_property_type(&self, key: &str) -> ModelResult<Option<String>>;
    fn get_property_names(&self) -> ModelResult<Vec<String>>;
    fn count_properties(&self) -> ModelResult<usize>;
    fn merge(&mut self, other: &Self) -> ModelResult<()>;
    fn calculate_hash(&self) -> ModelResult<u64>;
    fn equals(&self, other: &Self) -> ModelResult<bool>;
}