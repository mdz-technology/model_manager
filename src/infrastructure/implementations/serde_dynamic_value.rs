use crate::infrastructure::implementations::array_iterator::SerdeArrayIterator;
use crate::infrastructure::implementations::object_iterator::SerdeObjectIterator;
use crate::{DynamicValue, ModelError, ModelResult};
use serde_json::Value;
use std::future::Future;
use std::hash::{DefaultHasher, Hash, Hasher};
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

    fn deep_clone_optimized<'a>(
        &'a self
    ) -> Pin<Box<dyn Future<Output = ModelResult<Self>> + Send + 'a>> {
        Box::pin(async move {
            if self.estimate_size() < 1024 * 1024 {
                return Ok(self.clone());
            }
            self.streaming_clone().await
        })
    }

    fn estimate_size(&self) -> usize {
        match &self.inner {
            Value::Null => 8,
            Value::Bool(_) => 8,
            Value::Number(_) => 16,
            Value::String(s) => s.len() + 24,
            Value::Array(arr) => {
                24 + arr.iter().map(|v| Self::from_value(v.clone()).estimate_size()).sum::<usize>()
            }
            Value::Object(obj) => {
                24 + obj.iter()
                    .map(|(k, v)| k.len() + Self::from_value(v.clone()).estimate_size())
                    .sum::<usize>()
            }
        }
    }

    async fn streaming_clone(&self) -> ModelResult<Self> {
        match &self.inner {
            Value::Object(obj) => {
                let mut new_map = serde_json::Map::new();

                let mut batch = Vec::new();
                const BATCH_SIZE: usize = 1000;

                for (key, value) in obj.iter() {
                    batch.push((key.clone(), value.clone()));

                    if batch.len() >= BATCH_SIZE {
                        for (k, v) in batch.drain(..) {
                            let cloned_value = Self::from_value(v).deep_clone_optimized().await?;
                            new_map.insert(k, cloned_value.inner);
                        }

                        tokio::task::yield_now().await;
                    }
                }

                for (k, v) in batch {
                    let cloned_value = Self::from_value(v).deep_clone_optimized().await?;
                    new_map.insert(k, cloned_value.inner);
                }

                Ok(Self::from_value(Value::Object(new_map)))
            }
            Value::Array(arr) => {
                let mut new_array = Vec::new();

                let mut batch = Vec::new();
                const BATCH_SIZE: usize = 1000;

                for value in arr.iter() {
                    batch.push(value.clone());

                    if batch.len() >= BATCH_SIZE {
                        for v in batch.drain(..) {
                            let cloned_value = Self::from_value(v).deep_clone_optimized().await?;
                            new_array.push(cloned_value.inner);
                        }

                        tokio::task::yield_now().await;
                    }
                }

                for v in batch {
                    let cloned_value = Self::from_value(v).deep_clone_optimized().await?;
                    new_array.push(cloned_value.inner);
                }

                Ok(Self::from_value(Value::Array(new_array)))
            }
            _ => {
                Ok(self.clone())
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

    fn merge_internal<'a>(
        &'a mut self,
        other: &'a Self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>> {
        Box::pin(async move {
            match (&mut self.inner, &other.inner) {
                (Value::Object(self_map), Value::Object(other_map)) => {
                    for (key, other_value) in other_map.iter() {
                        let other_dynamic = Self::from_value(other_value.clone());

                        match self_map.get_mut(key) {
                            Some(self_value) => {
                                if self_value.is_object() && other_value.is_object() {
                                    let mut self_dynamic = Self::from_value(self_value.clone());
                                    Box::pin(self_dynamic.merge_internal(&other_dynamic)).await?;
                                    *self_value = self_dynamic.inner;
                                } else if self_value.is_array() && other_value.is_array() {
                                        if let (Value::Array(self_arr), Value::Array(other_arr)) = (self_value, other_value) {
                                            self_arr.extend(other_arr.iter().cloned());
                                        }
                                } else {
                                    *self_value = other_value.clone();
                                }
                            }
                            None => {
                                self_map.insert(key.clone(), other_value.clone());
                            }
                        }
                    }
                    Ok(())
                }

                (Value::Array(self_arr), Value::Array(other_arr)) => {
                    self_arr.extend(other_arr.iter().cloned());
                    Ok(())
                }

                _ => {
                    self.inner = other.inner.clone();
                    Ok(())
                }
            }
        })
    }

    fn calculate_hash_internal<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<u64>> + Send + 'a>> {
        Box::pin(async move {
            let mut hasher = DefaultHasher::new();

            match &self.inner {
                Value::Null => {
                    "null".hash(&mut hasher);
                }
                Value::Bool(b) => {
                    "bool".hash(&mut hasher);
                    b.hash(&mut hasher);
                }
                Value::Number(n) => {
                    "number".hash(&mut hasher);
                    // Convertir a string para hash consistente
                    n.to_string().hash(&mut hasher);
                }
                Value::String(s) => {
                    "string".hash(&mut hasher);
                    s.hash(&mut hasher);
                }
                Value::Array(arr) => {
                    "array".hash(&mut hasher);
                    arr.len().hash(&mut hasher);

                    if arr.len() > 1000 {
                        let indices = [0, arr.len()/4, arr.len()/2, 3*arr.len()/4, arr.len()-1];
                        for &idx in &indices {
                            if idx < arr.len() {
                                let elem_dynamic = Self::from_value(arr[idx].clone());
                                let elem_hash = Box::pin(elem_dynamic.calculate_hash_internal()).await?;
                                elem_hash.hash(&mut hasher);
                            }
                        }
                    } else {
                        for value in arr {
                            let elem_dynamic = Self::from_value(value.clone());
                            let elem_hash = Box::pin(elem_dynamic.calculate_hash_internal()).await?;
                            elem_hash.hash(&mut hasher);
                        }
                    }
                }
                Value::Object(obj) => {
                    "object".hash(&mut hasher);
                    obj.len().hash(&mut hasher);

                    let mut sorted_keys: Vec<_> = obj.keys().collect();
                    sorted_keys.sort();

                    if sorted_keys.len() > 100 {
                        let sample_size = 20;
                        let step = sorted_keys.len() / sample_size;
                        for i in (0..sorted_keys.len()).step_by(step.max(1)).take(sample_size) {
                            let key = sorted_keys[i];
                            key.hash(&mut hasher);
                            if let Some(value) = obj.get(key) {
                                let value_dynamic = Self::from_value(value.clone());
                                let value_hash = Box::pin(value_dynamic.calculate_hash_internal()).await?;
                                value_hash.hash(&mut hasher);
                            }
                        }
                    } else {
                        for key in sorted_keys {
                            key.hash(&mut hasher);
                            if let Some(value) = obj.get(key) {
                                let value_dynamic = Self::from_value(value.clone());
                                let value_hash = Box::pin(value_dynamic.calculate_hash_internal()).await?;
                                value_hash.hash(&mut hasher);
                            }
                        }
                    }
                }
            }

            Ok(hasher.finish())
        })
    }

    fn equals_internal<'a>(
        &'a self,
        other: &'a Self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<bool>> + Send + 'a>> {
        Box::pin(async move {
            if std::mem::discriminant(&self.inner) != std::mem::discriminant(&other.inner) {
                return Ok(false);
            }

            if self.estimate_size() > 10000 || other.estimate_size() > 10000 {
                let self_hash = Box::pin(self.calculate_hash_internal()).await?;
                let other_hash = Box::pin(other.calculate_hash_internal()).await?;

                if self_hash != other_hash {
                    return Ok(false);
                }
            }

            match (&self.inner, &other.inner) {
                (Value::Null, Value::Null) => Ok(true),
                (Value::Bool(a), Value::Bool(b)) => Ok(a == b),
                (Value::Number(a), Value::Number(b)) => Ok(a == b),
                (Value::String(a), Value::String(b)) => Ok(a == b),

                (Value::Array(a), Value::Array(b)) => {
                    if a.len() != b.len() {
                        return Ok(false);
                    }

                    for (a_val, b_val) in a.iter().zip(b.iter()) {
                        let a_dynamic = Self::from_value(a_val.clone());
                        let b_dynamic = Self::from_value(b_val.clone());

                        if !Box::pin(a_dynamic.equals_internal(&b_dynamic)).await? {
                            return Ok(false);
                        }
                    }

                    Ok(true)
                }

                (Value::Object(a), Value::Object(b)) => {
                    if a.len() != b.len() {
                        return Ok(false);
                    }

                    for (key, a_val) in a.iter() {
                        match b.get(key) {
                            Some(b_val) => {
                                let a_dynamic = Self::from_value(a_val.clone());
                                let b_dynamic = Self::from_value(b_val.clone());

                                if !Box::pin(a_dynamic.equals_internal(&b_dynamic)).await? {
                                    return Ok(false);
                                }
                            }
                            None => return Ok(false),
                        }
                    }

                    Ok(true)
                }

                _ => Ok(false),
            }
        })
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

    fn deep_clone<'a>(
        &'a self
    ) -> Pin<Box<dyn Future<Output = ModelResult<Self>> + Send + 'a>> {
        Box::pin(async move {
            self.deep_clone_optimized().await
        })
    }

    fn has_property<'a>(
        &'a self,
        key: &'a str,
    ) -> Pin<Box<dyn Future<Output = ModelResult<bool>> + Send + 'a>> {
        Box::pin(async move {
            match &self.inner {
                Value::Object(obj) => Ok(obj.contains_key(key)),
                _ => Err(ModelError::InvalidData(
                    "Cannot check property on non-object value".to_string(),
                )),
            }
        })
    }


    fn get_property_type<'a>(
        &'a self,
        key: &'a str,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<String>>> + Send + 'a>> {
        Box::pin(async move {
            match &self.inner {
                Value::Object(obj) => {
                    match obj.get(key) {
                        Some(value) => Ok(Some(Self::get_value_type_string(value))),
                        None => Ok(None),
                    }
                }
                _ => Err(ModelError::InvalidData(
                    "Cannot get property type on non-object value".to_string(),
                )),
            }
        })
    }

    fn get_property_names<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Vec<String>>> + Send + 'a>> {
        Box::pin(async move {
            match &self.inner {
                Value::Object(obj) => {
                    let mut names: Vec<String> = obj.keys().cloned().collect();
                    names.sort();
                    Ok(names)
                }
                _ => Err(ModelError::InvalidData(
                    "Cannot get property names on non-object value".to_string(),
                )),
            }
        })
    }

    fn count_properties<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = ModelResult<usize>> + Send + 'a>> {
        Box::pin(async move {
            match &self.inner {
                Value::Object(obj) => Ok(obj.len()),
                _ => Err(ModelError::InvalidData(
                    "Cannot count properties on non-object value".to_string(),
                )),
            }
        })
    }

    fn merge<'a>(
        &'a mut self,
        other: &'a Self
    ) -> Pin<Box<dyn Future<Output = ModelResult<()>> + Send + 'a>> {
        Box::pin(async move {
            self.merge_internal(other).await
        })
    }

    fn calculate_hash<'a>(
        &'a self
    ) -> Pin<Box<dyn Future<Output = ModelResult<u64>> + Send + 'a>> {
        Box::pin(async move {
            self.calculate_hash_internal().await
        })
    }

    fn equals<'a>(
        &'a self,
        other: &'a Self
    ) -> Pin<Box<dyn Future<Output = ModelResult<bool>> + Send + 'a>> {
        Box::pin(async move {
            self.equals_internal(other).await
        })
    }
}
