use crate::application::services::dynamic_value_factory::DynamicValueFactory;
use crate::AsyncDynamicValue;
use crate::infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;

pub struct DefaultValueFactory;

impl DynamicValueFactory for DefaultValueFactory {
    type Value = SerdeDynamicValue;
    fn create() -> Self::Value {
        SerdeDynamicValue::new_object()
    }
}