use crate::application::services::value_factory::ValueFactory;
use crate::CoreValue;
use crate::infrastructure::implementations::serde_core_value::SerdeCoreValue;

pub struct DefaultValueFactory;

impl ValueFactory for DefaultValueFactory {
    type Value = SerdeCoreValue;

    fn create_object() -> Self::Value {
        SerdeCoreValue::new_object()
    }

    fn create_array() -> Self::Value {
        SerdeCoreValue::new_array()
    }

    fn create_string(s: &str) -> Self::Value {
        SerdeCoreValue::from_str(s)
    }

    fn create_number(n: f64) -> Result<Self::Value, String> {
        SerdeCoreValue::from_number(n).map_err(|e| e.to_string())
    }

    fn create_bool(b: bool) -> Self::Value {
        SerdeCoreValue::from_bool(b)
    }
}