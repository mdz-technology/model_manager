use std::collections::HashMap;
use std::fmt::Debug;

pub type DynamicResult<T> = Result<T, DynamicError>;

#[derive(Debug)]
pub enum DynamicError {
    TypeMismatch(String),
    InvalidNumber(String),
    PathNotFound(String),
    InvalidPath(String),
    SerializationError(String),
}

pub trait DynamicValue: Clone + Debug + Send + Sync {
    fn new_object() -> Self;
    fn new_array() -> Self;
    fn from_str(s: &str) -> Self;
    fn from_number(n: f64) -> DynamicResult<Self> where Self: Sized;
    fn from_bool(b: bool) -> Self;

    fn is_object(&self) -> bool;
    fn is_array(&self) -> bool;
    fn as_str(&self) -> Option<String>;
    fn as_number(&self) -> Option<f64>;
    fn as_bool(&self) -> Option<bool>;
    fn as_map(&self) -> Option<HashMap<String, Self>>;
    fn as_array(&self) -> Option<Vec<Self>>;
    fn get(&self, key: &str) -> Option<Self>;
    fn get_all(&self) -> Vec<Self>;

    fn set(&mut self, key: &str, value: Self) -> DynamicResult<()>;
    fn remove(&mut self, key: &str) -> DynamicResult<()>;
    fn push(&mut self, value: Self) -> DynamicResult<()>;

    fn to_string(&self) -> String;
    fn is_empty(&self) -> bool;
    fn get_type(&self) -> String;

    fn iter_object(&self) -> Option<Box<dyn Iterator<Item = (String, Self)> + '_>>;
    fn iter_array(&self) -> Option<Box<dyn Iterator<Item = Self> + '_>>;

    fn get_by_path(&self, path: &str) -> DynamicResult<Option<Self>>;
    fn set_by_path(&mut self, path: &str, value: Self) -> DynamicResult<()>;
    fn deep_clone(&self) -> Self;
    fn merge(&mut self, other: &Self) -> DynamicResult<()>;

    fn keys(&self) -> Vec<String>;
    fn matches_schema(&self, schema: &Self) -> bool;

    fn is_valid_path(path: &str) -> bool where Self: Sized {
        if path.is_empty() {
            return false;
        }

        if path.starts_with('.') || path.ends_with('.') {
            return false;
        }

        if path.contains("..") {
            return false;
        }

        path.split('.').all(|part| !part.is_empty())
    }

    fn split_path(path: &str) -> Vec<String> where Self: Sized {
        path.split('.').map(|s| s.to_string()).collect()
    }

}