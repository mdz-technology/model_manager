use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{Map, Number, Value};
use crate::dynamic::application::dynamic_value::{DynamicValue, DynamicError, DynamicResult};

#[derive(Debug, Clone)]
pub struct  DynamicValueImpl {
    inner: Arc<Value>
}

impl DynamicValueImpl {
    fn new(inner: Value) -> Self {
        Self { inner: Arc::new(inner) }
    }

}

impl DynamicValue for DynamicValueImpl {

    fn new_object() -> Self {
        Self::new(Value::Object(Map::new()))
    }

    fn new_array() -> Self {
        Self::new(Value::Array(Vec::new()))
    }

    fn from_str(str_value: &str) -> Self {
        Self::new(Value::String(str_value.to_string()))
    }

    fn from_number(n: f64) -> DynamicResult<Self> {
        if let Some(num) = Number::from_f64(n) {
            Ok(Self::new(Value::Number(num)))
        } else {
            Err(DynamicError::InvalidNumber("Número inválido (NaN o infinito)".to_string()))
        }
    }

    fn from_bool(bool_value: bool) -> Self {
        Self::new(Value::Bool(bool_value))
    }

    fn is_object(&self) -> bool {
        self.inner.is_object()
    }

    fn is_array(&self) -> bool {
        self.inner.is_array()
    }

    fn as_str(&self) -> Option<&str> {
        self.inner.as_str()
    }

    fn as_number(&self) -> Option<f64> {
        self.inner.as_f64()
    }

    fn as_bool(&self) -> Option<bool> {
        self.inner.as_bool()
    }

    fn as_map(&self) -> Option<HashMap<String, Self>> {
        self.inner.as_object().map(|map| {
            map.iter()
                .map(|(k, v)| (k.clone(), Self::new(v.clone())))
                .collect()
        })
    }

    fn as_array(&self) -> Option<Vec<Self>> {
        self.inner.as_array().map(|arr| {
            arr.iter().map(|v| Self::new(v.clone())).collect()
        })
    }

    fn get(&self, key: &str) -> Option<Self> {
        self.inner.get(key).map(|v| Self::new(v.clone()))
    }

    fn get_all(&self) -> Vec<Self> {
        if let Value::Object(ref map) = *self.inner {
            map.values().map(|v| Self::new(v.clone())).collect()
        } else {
            vec![]
        }
    }

    fn set(&mut self, key: &str, value: Self) -> DynamicResult<()> {
        if let Value::Object(ref mut map) = Arc::make_mut(&mut self.inner) {
            map.insert(key.to_string(), (*value.inner).clone());
            Ok(())
        } else {
            Err(DynamicError::TypeMismatch("No se puede establecer una clave en un valor que no es objeto".to_string()))
        }
    }

    fn remove(&mut self, key: &str) -> DynamicResult<()> {
        if let Value::Object(ref mut map) = Arc::make_mut(&mut self.inner) {
            map.remove(key);
            Ok(())
        } else {
            Err(DynamicError::TypeMismatch("No se puede eliminar una clave en un valor que no es objeto".to_string()))
        }
    }

    fn push(&mut self, value: Self) -> DynamicResult<()> {
        if let Value::Array(ref mut arr) = Arc::make_mut(&mut self.inner) {
            arr.push((*value.inner).clone());
            Ok(())
        } else {
            Err(DynamicError::TypeMismatch("No se puede agregar un elemento a un valor que no es arreglo".to_string()))
        }
    }

    fn to_string(&self) -> String {
        self.inner.to_string()
    }

    fn is_empty(&self) -> bool {
        match &*self.inner {
            Value::Null => true,
            Value::String(s) if s.is_empty() => true,
            Value::Array(arr) if arr.is_empty() => true,
            Value::Object(obj) if obj.is_empty() => true,
            _ => false,
        }
    }
}