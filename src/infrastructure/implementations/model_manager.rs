use std::collections::HashMap;
use uuid::Uuid;
use crate::{DynamicValue, ModelManager, ModelResult, ModelError};

pub struct SimpleModelManager<T: DynamicValue> {
    models: HashMap<String, HashMap<String, T>>,
}

impl<T: DynamicValue> SimpleModelManager<T> {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    fn get_or_create_model(&mut self, model_name: &str) -> &mut HashMap<String, T> {
        self.models.entry(model_name.to_string()).or_insert_with(HashMap::new)
    }

    fn values_equal(value1: &T, value2: &T) -> bool {
        value1.to_string() == value2.to_string()
    }
}

impl<T: DynamicValue> ModelManager<T> for SimpleModelManager<T> {
    fn insert(&mut self, model_name: String, id: Option<String>, data: T) -> ModelResult<T> {
        let actual_id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let model = self.get_or_create_model(&model_name);

        let data_clone = data.clone();
        model.insert(actual_id, data);

        Ok(data_clone)
    }

    fn update(&mut self, model_name: String, id: String, data: T) -> ModelResult<T> {
        let model = self.get_or_create_model(&model_name);

        if model.contains_key(&id) {
            let data_clone = data.clone();
            model.insert(id, data);
            Ok(data_clone)
        } else {
            Err(ModelError::NotFound(id))
        }
    }

    fn get(&mut self, model_name: String, id: String) -> ModelResult<T> {
        let model = self.get_or_create_model(&model_name);

        model.get(&id)
            .cloned()
            .ok_or_else(|| ModelError::NotFound(id))
    }

    fn remove(&mut self, model_name: String, id: String) -> ModelResult<T> {
        let model = self.get_or_create_model(&model_name);

        model.remove(&id)
            .ok_or_else(|| ModelError::NotFound(id))
    }

    fn get_all(&mut self, model_name: String) -> ModelResult<Vec<T>> {
        let model = self.get_or_create_model(&model_name);
        Ok(model.values().cloned().collect())
    }

    fn get_by_path(&mut self, model_name: String, id: String, path: String) -> ModelResult<Option<T>> {
        let record = self.get(model_name, id)?;
        record.get_by_path(&path)
    }

    fn find_by_path_exists(&mut self, model_name: String, path: String) -> ModelResult<Vec<T>> {
        let model = self.get_or_create_model(&model_name);
        let mut results = Vec::new();

        for record in model.values() {
            match record.has_path(&path) {
                Ok(true) => results.push(record.clone()),
                Ok(false) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }

    fn find_by_path_value(&mut self, model_name: String, path: String, expected_value: T) -> ModelResult<Vec<T>> {
        let model = self.get_or_create_model(&model_name);
        let mut results = Vec::new();

        for record in model.values() {
            match record.get_by_path(&path) {
                Ok(Some(value)) => {
                    if Self::values_equal(&value, &expected_value) {
                        results.push(record.clone());
                    }
                }
                Ok(None) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }
}