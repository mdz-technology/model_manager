use crate::DynamicValue;

pub trait ValueFactory {
    type Value: DynamicValue;

    fn create_object() -> Self::Value;
    fn create_array() -> Self::Value;
    fn create_string(s: &str) -> Self::Value;
    fn create_number(n: f64) -> Result<Self::Value, String>;
    fn create_bool(b: bool) -> Self::Value;
}