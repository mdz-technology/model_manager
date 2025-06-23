use crate::application::traits::path_navigation::PathNavigation;
use crate::infrastructure::implementations::serde_core_value::SerdeCoreValue;
use crate::{ModelError, ModelResult};
use serde_json::Value;
use crate::application::traits::core_value::CoreValue;

impl PathNavigation for SerdeCoreValue {
    fn get_by_path(&self, path: &str) -> ModelResult<Option<Self>> {
        let parts = parse_path(path);
        navigate_to_value(self, &parts)
    }

    fn has_path(&self, path: &str) -> ModelResult<bool> {
        match self.get_by_path(path)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    fn set_by_path(&mut self, path: &str, value: Self) -> ModelResult<()> {
        validate_path_for_setting(path)?;

        let normalized_path = normalize_path(path);
        if normalized_path.is_empty() {
            return Err(ModelError::InvalidData("Path becomes empty after normalization".to_string()));
        }

        let parts = parse_path(&normalized_path);

        if !self.is_object() {
            return Err(ModelError::InvalidData("Cannot set path on non-object root value".to_string()));
        }

        set_by_path_internal(self, &parts, value)
    }
}

fn parse_path(path: &str) -> Vec<&str> {
    if path.is_empty() {
        Vec::new()
    } else {
        path.split('.').collect()
    }
}

fn navigate_to_value(value: &SerdeCoreValue, parts: &[&str]) -> ModelResult<Option<SerdeCoreValue>> {
    if parts.is_empty() {
        return Ok(Some(value.clone()));
    }

    let mut current_value = value;
    let mut owned_values = Vec::new();

    for part in parts {
        if !current_value.is_object() {
            return Ok(None);
        }

        match current_value.inner.get(part) {
            Some(inner_value) => {
                let next_value = SerdeCoreValue::from_serde_value(inner_value.clone());
                owned_values.push(next_value);
                current_value = owned_values.last().unwrap();
            }
            None => return Ok(None),
        }
    }

    Ok(Some(current_value.clone()))
}

fn normalize_path(path: &str) -> String {
    path.trim()
        .split('.')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(".")
}

fn validate_path_for_setting(path: &str) -> ModelResult<()> {
    if path.is_empty() {
        return Err(ModelError::InvalidData("Empty path not allowed".to_string()));
    }

    let parts = parse_path(path);

    for (index, part) in parts.iter().enumerate() {
        if part.is_empty() {
            return Err(ModelError::InvalidData(format!(
                "Invalid path '{}': empty segment at position {}",
                path, index
            )));
        }
    }

    Ok(())
}

fn set_by_path_internal(value: &mut SerdeCoreValue, parts: &[&str], new_value: SerdeCoreValue) -> ModelResult<()> {
    if parts.is_empty() {
        return Err(ModelError::InvalidData("Cannot set empty path".to_string()));
    }

    if parts.len() == 1 {
        return value.set(parts[0], new_value);
    }

    let mut current_map = match &mut value.inner {
        Value::Object(map) => map,
        _ => return Err(ModelError::InvalidData("Expected object".to_string())),
    };

    for part in &parts[..parts.len()-1] {
        let entry = current_map.entry(part.to_string()).or_insert_with(|| {
            Value::Object(serde_json::Map::new())
        });

        if !entry.is_object() {
            *entry = Value::Object(serde_json::Map::new());
        }

        current_map = match entry {
            Value::Object(map) => map,
            _ => return Err(ModelError::InvalidData("Expected object".to_string())),
        };
    }

    let final_key = parts[parts.len()-1];
    current_map.insert(final_key.to_string(), new_value.inner);

    Ok(())
}