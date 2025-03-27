use std::collections::HashMap;
use std::fmt::Debug;

pub type DynamicResult<T> = Result<T, DynamicError>;

#[derive(Debug)]
pub enum DynamicError {
    TypeMismatch(String),
    InvalidNumber(String),
    // Se pueden agregar más variantes según necesidad.
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

}