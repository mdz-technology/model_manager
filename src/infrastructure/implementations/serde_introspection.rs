use crate::application::traits::value_introspection::ValueIntrospection;
use crate::infrastructure::implementations::serde_core_value::SerdeCoreValue;
use crate::{ModelError, ModelResult};
use serde_json::Value;

impl ValueIntrospection for SerdeCoreValue {
    fn has_property(&self, key: &str) -> ModelResult<bool> {
        match &self.inner {
            Value::Object(obj) => Ok(obj.contains_key(key)),
            _ => Err(ModelError::InvalidData("Cannot check property on non-object value".to_string())),
        }
    }

    fn get_property_type(&self, key: &str) -> ModelResult<Option<String>> {
        match &self.inner {
            Value::Object(obj) => {
                match obj.get(key) {
                    Some(value) => Ok(Some(get_value_type_string(value))),
                    None => Ok(None),
                }
            }
            _ => Err(ModelError::InvalidData("Cannot get property type on non-object value".to_string())),
        }
    }

    fn get_property_names(&self) -> ModelResult<Vec<String>> {
        match &self.inner {
            Value::Object(obj) => {
                let mut names: Vec<String> = obj.keys().cloned().collect();
                names.sort();
                Ok(names)
            }
            _ => Err(ModelError::InvalidData("Cannot get property names on non-object value".to_string())),
        }
    }

    fn count_properties(&self) -> ModelResult<usize> {
        match &self.inner {
            Value::Object(obj) => Ok(obj.len()),
            _ => Err(ModelError::InvalidData("Cannot count properties on non-object value".to_string())),
        }
    }
}

fn get_value_type_string(value: &Value) -> String {
    match value {
        Value::Null => "Null".to_string(),
        Value::Bool(_) => "Bool".to_string(),
        Value::Number(_) => "Number".to_string(),
        Value::String(_) => "String".to_string(),
        Value::Array(_) => "Array".to_string(),
        Value::Object(_) => "Object".to_string(),
    }
}