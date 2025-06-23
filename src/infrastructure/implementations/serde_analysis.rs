use crate::application::traits::value_analysis::ValueAnalysis;
use crate::infrastructure::implementations::serde_core_value::SerdeCoreValue;
use crate::ModelResult;
use serde_json::Value;
use std::hash::{DefaultHasher, Hash, Hasher};

impl ValueAnalysis for SerdeCoreValue {
    fn deep_clone(&self) -> ModelResult<Self> {
        Ok(self.clone())
    }

    fn merge(&mut self, other: &Self) -> ModelResult<()> {
        merge_internal(self, other)
    }

    fn calculate_hash(&self) -> ModelResult<u64> {
        let mut hasher = DefaultHasher::new();
        self.inner.to_string().hash(&mut hasher);
        Ok(hasher.finish())
    }

    fn equals(&self, other: &Self) -> ModelResult<bool> {
        equals_internal(self, other)
    }
}

fn merge_internal(this: &mut SerdeCoreValue, other: &SerdeCoreValue) -> ModelResult<()> {
    match (&mut this.inner, &other.inner) {
        (Value::Object(self_map), Value::Object(other_map)) => {
            for (key, other_value) in other_map.iter() {
                let other_dynamic = SerdeCoreValue::from_serde_value(other_value.clone());

                match self_map.get_mut(key) {
                    Some(self_value) => {
                        if self_value.is_object() && other_value.is_object() {
                            let mut self_dynamic = SerdeCoreValue::from_serde_value(self_value.clone());
                            merge_internal(&mut self_dynamic, &other_dynamic)?;
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
            this.inner = other.inner.clone();
            Ok(())
        }
    }
}

fn equals_internal(this: &SerdeCoreValue, other: &SerdeCoreValue) -> ModelResult<bool> {
    if std::mem::discriminant(&this.inner) != std::mem::discriminant(&other.inner) {
        return Ok(false);
    }

    match (&this.inner, &other.inner) {
        (Value::Null, Value::Null) => Ok(true),
        (Value::Bool(a), Value::Bool(b)) => Ok(a == b),
        (Value::Number(a), Value::Number(b)) => Ok(a == b),
        (Value::String(a), Value::String(b)) => Ok(a == b),
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Ok(false);
            }

            for (a_val, b_val) in a.iter().zip(b.iter()) {
                let a_dynamic = SerdeCoreValue::from_serde_value(a_val.clone());
                let b_dynamic = SerdeCoreValue::from_serde_value(b_val.clone());

                if !equals_internal(&a_dynamic, &b_dynamic)? {
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
                        let a_dynamic = SerdeCoreValue::from_serde_value(a_val.clone());
                        let b_dynamic = SerdeCoreValue::from_serde_value(b_val.clone());

                        if !equals_internal(&a_dynamic, &b_dynamic)? {
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
}