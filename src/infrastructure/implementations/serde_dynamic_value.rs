use std::pin::Pin;
use std::future::Future;
use serde_json::Value;
use crate::{DynamicValue, ModelError, ModelResult};

#[derive(Debug, Clone)]
pub struct SerdeDynamicValue {
    inner: Value,
}

impl SerdeDynamicValue {
    pub fn from_value(value: Value) -> Self {
        Self { inner: value }
    }

    pub fn into_value(self) -> Value {
        self.inner
    }
}

impl DynamicValue for SerdeDynamicValue {

    fn new_object() -> Self {
        Self {
            inner: Value::Object(serde_json::Map::new())
        }
    }

    fn new_array() -> Self {
        Self {
            inner: Value::Array(Vec::new())
        }
    }

    fn from_str(s: &str) -> Self {
        Self {
            inner: Value::String(s.to_string())
        }
    }

    fn from_number(n: f64) -> ModelResult<Self> {
        if n.is_finite() {
            if let Some(num) = serde_json::Number::from_f64(n) {
                Ok(Self { inner: Value::Number(num) })
            } else {
                Err(ModelError::InvalidData(format!("Invalid number: {}", n)))
            }
        } else {
            Err(ModelError::InvalidData(format!("Non-finite number: {}", n)))
        }
    }

    fn from_bool(b: bool) -> Self {
        Self {
            inner: Value::Bool(b)
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

    fn get<'a>(
        &'a self,
        key: &'a str
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<Self>>> + Send + 'a>> {
        Box::pin(async move {
            Ok(self.inner.get(key).map(|v| Self::from_value(v.clone())))
        })
    }

    fn set<'a>(
        &'a mut self,
        key: &'a str,
        value: Self
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>> {
        Box::pin(async move {
            match &mut self.inner {
                Value::Object(map) => {
                    map.insert(key.to_string(), value.inner);
                    Ok(())
                }
                _ => Err(ModelError::InvalidData(
                    "Cannot set key on non-object value".to_string()
                ))
            }
        })
    }

    fn push<'a>(
        &'a mut self,
        value: Self
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>> {
        Box::pin(async move {
            match &mut self.inner {
                Value::Array(arr) => {
                    arr.push(value.inner);
                    Ok(())
                }
                _ => Err(ModelError::InvalidData(
                    "Cannot push to non-array value".to_string()
                ))
            }
        })
    }

    fn as_array<'a>(
        &'a self
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<Vec<Self>>>> + Send + 'a>> {
        Box::pin(async move {
            match &self.inner {
                Value::Array(arr) => {
                    let result = arr.iter()
                        .map(|v| Self::from_value(v.clone()))
                        .collect();
                    Ok(Some(result))
                }
                _ => Ok(None)
            }
        })
    }

    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}