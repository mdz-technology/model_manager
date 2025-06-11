use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde_json::{Map, Number, Value};
use crate::dynamic::application::dynamic_value::{DynamicValue, DynamicError, DynamicResult};

#[derive(Debug, Clone)]
pub struct DynamicValueImpl {
    inner: Arc<RwLock<Value>>,
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
            Err(DynamicError::InvalidNumber(
                "Número inválido (NaN o infinito)".to_string(),
            ))
        }
    }

    fn from_bool(bool_value: bool) -> Self {
        Self::new(Value::Bool(bool_value))
    }

    fn is_object(&self) -> bool {
        self.inner.read().unwrap().is_object()
    }

    fn is_array(&self) -> bool {
        self.inner.read().unwrap().is_array()
    }

    fn as_str(&self) -> Option<String> {
        self.inner.read().unwrap().as_str().map(|s| s.to_string())
    }

    fn as_number(&self) -> Option<f64> {
        self.inner.read().unwrap().as_f64()
    }

    fn as_bool(&self) -> Option<bool> {
        self.inner.read().unwrap().as_bool()
    }

    fn as_map(&self) -> Option<HashMap<String, Self>> {
        self.inner.read().unwrap().as_object().map(|map| {
            map.iter()
                .map(|(k, v)| (k.clone(), Self::new(v.clone())))
                .collect()
        })
    }

    fn as_array(&self) -> Option<Vec<Self>> {
        self.inner.read().unwrap().as_array().map(|arr| {
            arr.iter().map(|v| Self::new(v.clone())).collect()
        })
    }

    fn get(&self, key: &str) -> Option<Self> {
        self.inner.read().unwrap().get(key).map(|v| Self::new(v.clone()))
    }

    fn get_all(&self) -> Vec<Self> {
        if let Value::Object(ref map) = *self.inner.read().unwrap() {
            map.values().map(|v| Self::new(v.clone())).collect()
        } else {
            vec![]
        }
    }

    fn set(&mut self, key: &str, value: Self) -> DynamicResult<()> {
        let mut lock = self.inner.write().unwrap();
        if let Value::Object(ref mut map) = *lock {
            map.insert(key.to_string(), (*value.inner.read().unwrap()).clone());
            Ok(())
        } else {
            Err(DynamicError::TypeMismatch(
                "No se puede establecer una clave en un valor que no es objeto".to_string(),
            ))
        }
    }

    fn remove(&mut self, key: &str) -> DynamicResult<()> {
        let mut lock = self.inner.write().unwrap();
        if let Value::Object(ref mut map) = *lock {
            map.remove(key);
            Ok(())
        } else {
            Err(DynamicError::TypeMismatch(
                "No se puede eliminar una clave en un valor que no es objeto".to_string(),
            ))
        }
    }

    fn push(&mut self, value: Self) -> DynamicResult<()> {
        let mut lock = self.inner.write().unwrap();
        if let Value::Array(ref mut arr) = *lock {
            arr.push((*value.inner.read().unwrap()).clone());
            Ok(())
        } else {
            Err(DynamicError::TypeMismatch(
                "No se puede agregar un elemento a un valor que no es arreglo".to_string(),
            ))
        }
    }

    fn to_string(&self) -> String {
        self.inner.read().unwrap().to_string()
    }

    fn is_empty(&self) -> bool {
        let lock = self.inner.read().unwrap();
        match &*lock {
            Value::Null => true,
            Value::String(s) if s.is_empty() => true,
            Value::Array(arr) if arr.is_empty() => true,
            Value::Object(obj) if obj.is_empty() => true,
            _ => false,
        }
    }

    fn get_type(&self) -> String {
        let lock = self.inner.read().unwrap();
        match &*lock {
            Value::Null => "Null".to_string(),
            Value::Bool(_) => "Bool".to_string(),
            Value::Number(_) => "Number".to_string(),
            Value::String(_) => "String".to_string(),
            Value::Array(_) => "Array".to_string(),
            Value::Object(_) => "Object".to_string(),
        }
    }

    fn iter_object(&self) -> Option<Box<dyn Iterator<Item=(String, Self)> + '_>> {
        let lock = self.inner.read().unwrap();
        if let Value::Object(ref map) = *lock {
            let items: Vec<(String, Self)> = map.iter()
                .map(|(k, v)| (k.clone(), Self::new(v.clone())))
                .collect();

            drop(lock);
            Some(Box::new(items.into_iter()))
        } else {
            None
        }
    }

    fn iter_array(&self) -> Option<Box<dyn Iterator<Item=Self> + '_>> {
        let lock = self.inner.read().unwrap();
        if let Value::Array(ref arr) = *lock {
            let items: Vec<Self> = arr.iter()
                .map(|v| Self::new(v.clone()))
                .collect();
            drop(lock);
            Some(Box::new(items.into_iter()))
        } else {
            None
        }
    }

    fn get_by_path(&self, path: &str) -> DynamicResult<Option<Self>> {
        if !Self::is_valid_path(path) {
            return Err(DynamicError::InvalidPath(
                format!("Path inválido: '{}'", path)
            ));
        }

        let parts = Self::split_path(path);
        let mut current = self.clone();

        for part in parts {
            if let Some(next) = current.get(&part) {
                current = next;
            } else {
                return Ok(None); // Path no encontrado, pero válido
            }
        }

        Ok(Some(current))
    }

    fn set_by_path(&mut self, path: &str, value: Self) -> DynamicResult<()> {
        if !Self::is_valid_path(path) {
            return Err(DynamicError::InvalidPath(
                format!("Path inválido: '{}'", path)
            ));
        }

        let parts = Self::split_path(path);
        if parts.is_empty() {
            return Err(DynamicError::InvalidPath("Path vacío".to_string()));
        }

        if parts.len() == 1 {
            return self.set(&parts[0], value);
        }

        self.set_by_path_simplified(&parts, value)
    }

    fn deep_clone(&self) -> Self {
        let lock = self.inner.read().unwrap();
        let cloned_value = lock.clone();
        drop(lock);
        Self::new(cloned_value)
    }

    fn merge(&mut self, other: &Self) -> DynamicResult<()> {
        if !self.is_object() {
            return Err(DynamicError::TypeMismatch(
                "Solo se pueden mergear objetos".to_string()
            ));
        }

        if !other.is_object() {
            return Err(DynamicError::TypeMismatch(
                "El valor a mergear debe ser un objeto".to_string()
            ));
        }

        if let Some(other_iter) = other.iter_object() {
            for (key, value) in other_iter {
                if let Some(existing) = self.get(&key) {
                    if existing.is_object() && value.is_object() {
                        let mut existing_clone = existing.clone();
                        existing_clone.merge(&value)?;
                        self.set(&key, existing_clone)?;
                    } else {
                        self.set(&key, value)?;
                    }
                } else {
                    self.set(&key, value)?;
                }
            }
        }

        Ok(())
    }

    fn keys(&self) -> Vec<String> {
        let lock = self.inner.read().unwrap();
        if let Value::Object(ref map) = *lock {
            map.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    //TODO: basic implementation, improve later
    fn matches_schema(&self, schema: &Self) -> bool {
        let self_type = self.get_type();
        let schema_lock = schema.inner.read().unwrap();

        match &*schema_lock {
            Value::String(expected_type) => {
                // Schema simple: {"field": "string", "other": "number"}
                match expected_type.as_str() {
                    "string" => self_type == "String",
                    "number" => self_type == "Number",
                    "boolean" => self_type == "Bool",
                    "object" => self_type == "Object",
                    "array" => self_type == "Array",
                    "null" => self_type == "Null",
                    _ => false,
                }
            },
            Value::Object(schema_map) => {
                if !self.is_object() {
                    return false;
                }

                for (key, expected_type_value) in schema_map {
                    if let Some(actual_value) = self.get(key) {
                        let expected_schema = Self::new(expected_type_value.clone());
                        if !actual_value.matches_schema(&expected_schema) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            },
            _ => false,
        }
    }
}

impl DynamicValueImpl {
    fn new(inner: Value) -> Self {
        Self {
            inner: Arc::new(RwLock::new(inner)),
        }
    }
    pub fn from_serde_value(value: Value) -> Self {
        Self::new(value)
    }

    fn set_by_path_simplified(&mut self, parts: &[String], value: Self) -> DynamicResult<()> {
        if parts.is_empty() {
            return Err(DynamicError::InvalidPath("Path vacío".to_string()));
        }

        if parts.len() == 1 {
            return self.set(&parts[0], value);
        }

        if !self.is_object() {
            *self = Self::new_object();
        }

        let current_key = &parts[0];
        let remaining_parts = &parts[1..];

        let mut nested_value = self.get(current_key).unwrap_or_else(|| Self::new_object());

        nested_value.set_by_path_simplified(remaining_parts, value)?;

        self.set(current_key, nested_value)?;

        Ok(())
    }

}