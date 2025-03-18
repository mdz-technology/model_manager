use std::any::Any;
use std::collections::HashMap;



pub trait DynamicValue {
    // These methods should NOT be used with `dyn DynamicValue`
    fn from_map(map: HashMap<&str, Box<dyn DynamicValue>>) -> Box<dyn DynamicValue>
    where Self: Sized;

    fn from_array(array: Vec<Box<dyn DynamicValue>>) -> Box<dyn DynamicValue>
    where Self: Sized;

    fn from_str(s: &str) -> Box<dyn DynamicValue>
    where Self: Sized;

    fn from_number(n: f64) -> Box<dyn DynamicValue>
    where Self: Sized;

    fn from_bool(b: bool) -> Box<dyn DynamicValue>
    where Self: Sized;

    // Safe for `dyn DynamicValue`
    fn is_object(&self) -> bool;
    fn is_array(&self) -> bool;
    fn as_str(&self) -> Option<String>;
    fn as_number(&self) -> Option<f64>;
    fn as_bool(&self) -> Option<bool>;
    fn as_map(&self) -> Option<HashMap<String, Box<dyn DynamicValue>>>;
    fn as_array(&self) -> Option<Vec<Box<dyn DynamicValue>>>;
    fn get(&self, key: &str) -> Option<Box<dyn DynamicValue>>;
    fn set(&mut self, key: &str, value: Box<dyn DynamicValue>);
    fn remove(&mut self, key: &str);
    fn push(&mut self, value: Box<dyn DynamicValue>);
    fn to_string(&self) -> String;
    fn is_empty(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn DynamicValue>;
}

impl Clone for Box<dyn DynamicValue> {
    fn clone(&self) -> Box<dyn DynamicValue> {
        self.clone_box()
    }
}