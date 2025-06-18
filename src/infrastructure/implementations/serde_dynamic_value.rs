use crate::infrastructure::implementations::array_iterator::SerdeArrayIterator;
use crate::infrastructure::implementations::object_iterator::SerdeObjectIterator;
use crate::{DynamicValue, ModelError, ModelResult};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

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

    pub fn iter_object(&self) -> ModelResult<SerdeObjectIterator> {
        match &self.inner {
            Value::Object(obj) => Ok(SerdeObjectIterator::new(obj.clone())),
            _ => Err(ModelError::InvalidData(
                "Value is not an object".to_string(),
            )),
        }
    }

    pub fn iter_array(&self) -> ModelResult<SerdeArrayIterator> {
        match &self.inner {
            Value::Array(arr) => Ok(SerdeArrayIterator::new(arr.clone())),
            _ => Err(ModelError::InvalidData("Value is not an array".to_string())),
        }
    }

    fn parse_path(path: &str) -> Vec<&str> {
        if path.is_empty() {
            Vec::new()
        } else {
            path.split('.').collect()
        }
    }

    fn navigate_to_value<'a>(
        &'a self,
        parts: &'a [&str],
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<Self>>> + Send + 'a>> {
        Box::pin(async move {
            if parts.is_empty() {
                return Ok(Some(self.clone()));
            }

            let current_key = parts[0];
            let remaining_parts = &parts[1..];

            if !self.is_object() {
                return Ok(None);
            }

            match self.inner.get(current_key) {
                Some(value) => {
                    let next_value = Self::from_value(value.clone());
                    next_value.navigate_to_value(remaining_parts).await
                }
                None => Ok(None),
            }
        })
    }

    fn set_by_path_internal<'a>(
        &'a mut self,
        parts: &'a [&str],
        value: Self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>> {
        Box::pin(async move {
            if parts.is_empty() {
                return Err(ModelError::InvalidData("Cannot set empty path".to_string()));
            }

            if parts.len() == 1 {
                return self.set(parts[0], value).await;
            }

            let current_key = parts[0];
            let remaining_parts = &parts[1..];

            if !self.is_object() {
                return Err(ModelError::InvalidData(format!(
                    "Cannot set path '{}' on non-object value",
                    current_key
                )));
            }

            let next_object = match self.inner.get_mut(current_key) {
                Some(existing_value) => {
                    if existing_value.is_object() {
                        existing_value
                    } else {
                        *existing_value = Value::Object(serde_json::Map::new());
                        existing_value
                    }
                }
                None => {
                    if let Value::Object(ref mut map) = &mut self.inner {
                        map.insert(
                            current_key.to_string(),
                            Value::Object(serde_json::Map::new()),
                        );
                        map.get_mut(current_key).unwrap()
                    } else {
                        return Err(ModelError::InvalidData(
                            "Internal error: expected object".to_string(),
                        ));
                    }
                }
            };

            let mut next_dynamic = Self::from_value(next_object.clone());
            next_dynamic
                .set_by_path_internal(remaining_parts, value)
                .await?;

            *next_object = next_dynamic.inner;

            Ok(())
        })
    }

    fn values_are_equal(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    pub async fn equals(&self, other: &Self) -> bool {
        self.values_are_equal(other)
    }

    pub fn get_path_parts(path: &str) -> Vec<String> {
        Self::parse_path(path)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn is_valid_path(path: &str) -> bool {
        if path.is_empty() {
            return false;
        }

        let parts = Self::parse_path(path);
        !parts.iter().any(|part| part.is_empty())
    }

    fn validate_path_for_setting(path: &str) -> ModelResult<()> {
        if path.is_empty() {
            return Err(ModelError::InvalidData(
                "Empty path not allowed".to_string(),
            ));
        }

        let parts = Self::parse_path(path);

        for (index, part) in parts.iter().enumerate() {
            if part.is_empty() {
                return Err(ModelError::InvalidData(format!(
                    "Invalid path '{}': empty segment at position {}",
                    path, index
                )));
            }
        }

        for part in &parts {
            if part.contains('[') || part.contains(']') {
                return Err(ModelError::InvalidData(format!(
                    "Invalid path '{}': array notation not supported in paths",
                    path
                )));
            }
        }

        Ok(())
    }

    fn normalize_path(path: &str) -> String {
        path.trim()
            .split('.')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(".")
    }
}

impl DynamicValue for SerdeDynamicValue {
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

    fn get<'a>(
        &'a self,
        key: &'a str,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<Self>>> + Send + 'a>> {
        Box::pin(async move { Ok(self.inner.get(key).map(|v| Self::from_value(v.clone()))) })
    }

    fn set<'a>(
        &'a mut self,
        key: &'a str,
        value: Self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>> {
        Box::pin(async move {
            match &mut self.inner {
                Value::Object(map) => {
                    map.insert(key.to_string(), value.inner);
                    Ok(())
                }
                _ => Err(ModelError::InvalidData(
                    "Cannot set key on non-object value".to_string(),
                )),
            }
        })
    }

    fn push<'a>(
        &'a mut self,
        value: Self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>> {
        Box::pin(async move {
            match &mut self.inner {
                Value::Array(arr) => {
                    arr.push(value.inner);
                    Ok(())
                }
                _ => Err(ModelError::InvalidData(
                    "Cannot push to non-array value".to_string(),
                )),
            }
        })
    }

    fn as_array<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<Vec<Self>>>> + Send + 'a>> {
        Box::pin(async move {
            match &self.inner {
                Value::Array(arr) => {
                    let result = arr.iter().map(|v| Self::from_value(v.clone())).collect();
                    Ok(Some(result))
                }
                _ => Ok(None),
            }
        })
    }

    fn to_string(&self) -> String {
        self.inner.to_string()
    }

    fn get_by_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<Self>>> + Send + 'a>> {
        Box::pin(async move {
            let parts = Self::parse_path(path);
            self.navigate_to_value(&parts).await
        })
    }

    fn has_path<'a>(
        &'a self,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = ModelResult<bool>> + Send + 'a>> {
        Box::pin(async move {
            match self.get_by_path(path).await? {
                Some(_) => Ok(true),
                None => Ok(false),
            }
        })
    }

    fn set_by_path<'a>(
        &'a mut self,
        path: &'a str,
        value: Self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>> {
        Box::pin(async move {
            Self::validate_path_for_setting(path)?;

            let normalized_path = Self::normalize_path(path);
            if normalized_path.is_empty() {
                return Err(ModelError::InvalidData(
                    "Path becomes empty after normalization".to_string(),
                ));
            }

            let parts = Self::parse_path(&normalized_path);

            if !self.is_object() {
                return Err(ModelError::InvalidData(
                    "Cannot set path on non-object root value".to_string(),
                ));
            }

            self.set_by_path_internal(&parts, value).await
        })
    }
}
