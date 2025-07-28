use crate::application::traits::core_value::CoreValue;
use crate::infrastructure::implementations::{serde_array_iterator::SerdeArrayIterator, serde_object_iterator::SerdeObjectIterator};
use crate::{ModelError, ModelResult};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SerdeCoreValue {
    pub inner: Value,
}

impl SerdeCoreValue {
    pub (crate) fn from_serde_value(value: Value) -> Self {
        Self { inner: value }
    }

    pub (crate) fn into_serde_value(self) -> Value {
        self.inner
    }

    pub (crate) fn iter_object(&self) -> ModelResult<SerdeObjectIterator> {
        match &self.inner {
            Value::Object(obj) => Ok(SerdeObjectIterator::new(obj.clone())),
            _ => Err(ModelError::InvalidData("Value is not an object".to_string())),
        }
    }

    pub (crate) fn iter_array(&self) -> ModelResult<SerdeArrayIterator> {
        match &self.inner {
            Value::Array(arr) => Ok(SerdeArrayIterator::new(arr.clone())),
            _ => Err(ModelError::InvalidData("Value is not an array".to_string())),
        }
    }

}

impl CoreValue for SerdeCoreValue {
    fn new_object() -> Self {
        Self {
            inner: Value::Object(serde_json::Map::new()),
        }
    }

    fn new_array() -> Self {
        Self {
            inner: Value::Array(Vec::new()),
        }
    }

    fn from_str(s: &str) -> Self {
        Self {
            inner: Value::String(s.to_string()),
        }
    }

    fn from_number(n: f64) -> ModelResult<Self> {
        if n.is_finite() {
            if let Some(num) = serde_json::Number::from_f64(n) {
                Ok(Self {
                    inner: Value::Number(num),
                })
            } else {
                Err(ModelError::InvalidData(format!("Invalid number: {}", n)))
            }
        } else {
            Err(ModelError::InvalidData(format!("Non-finite number: {}", n)))
        }
    }

    fn from_bool(b: bool) -> Self {
        Self {
            inner: Value::Bool(b),
        }
    }

    fn is_object(&self) -> bool {
        self.inner.is_object()
    }

    fn is_array(&self) -> bool {
        self.inner.is_array()
    }

    fn is_empty(&self) -> bool {
        match &self.inner {
            Value::Null => true,
            Value::String(s) => s.is_empty(),
            Value::Array(arr) => arr.is_empty(),
            Value::Object(obj) => obj.is_empty(),
            _ => false,
        }
    }

    fn get_type(&self) -> String {
        match &self.inner {
            Value::Null => "Null".to_string(),
            Value::Bool(_) => "Bool".to_string(),
            Value::Number(_) => "Number".to_string(),
            Value::String(_) => "String".to_string(),
            Value::Array(_) => "Array".to_string(),
            Value::Object(_) => "Object".to_string(),
        }
    }

    fn as_str(&self) -> Option<String> {
        self.inner.as_str().map(|s| s.to_string())
    }

    fn as_number(&self) -> Option<f64> {
        self.inner.as_f64()
    }

    fn as_bool(&self) -> Option<bool> {
        self.inner.as_bool()
    }

    fn as_array(&self) -> ModelResult<Option<Vec<Self>>> {
        match &self.inner {
            Value::Array(arr) => {
                let result = arr.iter().map(|v| Self::from_serde_value(v.clone())).collect();
                Ok(Some(result))
            }
            _ => Ok(None),
        }
    }

    fn get(&self, key: &str) -> ModelResult<Option<Self>> {
        Ok(self.inner.get(key).map(|v| Self::from_serde_value(v.clone())))
    }

    fn set(&mut self, key: &str, value: Self) -> ModelResult<()> {
        match &mut self.inner {
            Value::Object(map) => {
                map.insert(key.to_string(), value.inner);
                Ok(())
            }
            _ => Err(ModelError::InvalidData("Cannot set key on non-object value".to_string())),
        }
    }

    fn push(&mut self, value: Self) -> ModelResult<()> {
        match &mut self.inner {
            Value::Array(arr) => {
                arr.push(value.inner);
                Ok(())
            }
            _ => Err(ModelError::InvalidData("Cannot push to non-array value".to_string())),
        }
    }

    fn remove_key(&mut self, key: &str) -> ModelResult<Option<Self>> {
        match &mut self.inner {
            Value::Object(map) => {
                Ok(map.remove(key).map(|v| Self::from_serde_value(v)))
            }
            _ => Err(ModelError::InvalidData("Cannot remove key from non-object value".to_string())),
        }
    }

    fn remove_at(&mut self, index: usize) -> ModelResult<Option<Self>> {
        match &mut self.inner {
            Value::Array(arr) => {
                if index < arr.len() {
                    Ok(Some(Self::from_serde_value(arr.remove(index))))
                } else {
                    Ok(None)
                }
            }
            _ => Err(ModelError::InvalidData("Cannot remove index from non-array value".to_string())),
        }
    }

    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}