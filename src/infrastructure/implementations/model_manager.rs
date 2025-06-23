use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use crate::{DynamicValue, ModelManager, ModelResult, ModelError};

pub struct SimpleModelManager<T: DynamicValue> {
    models: Arc<RwLock<HashMap<String, HashMap<String, Arc<T>>>>>,
}

impl<T: DynamicValue> SimpleModelManager<T> {
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn with_model_write<F, R>(&self, model_name: &str, f: F) -> ModelResult<R>
    where
        F: FnOnce(&mut HashMap<String, Arc<T>>) -> R,
    {
        let mut models = self.models.write()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

        let model = models.entry(model_name.to_string())
            .or_insert_with(HashMap::new);

        Ok(f(model))
    }
}

pub struct ModelIterator<T: Clone> {
    items: Vec<T>,
    index: usize,
}

impl<T: Clone> ModelIterator<T> {
    fn new(items: Vec<T>) -> Self {
        Self {
            items,
            index: 0,
        }
    }
}

impl<T: Clone> Iterator for ModelIterator<T> {
    type Item = ModelResult<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(Ok(item))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.items.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<T: Clone> ExactSizeIterator for ModelIterator<T> {
    fn len(&self) -> usize {
        self.items.len() - self.index
    }
}

impl<T: DynamicValue> ModelManager<T> for SimpleModelManager<T> {
    fn insert(&self, model_name: &str, id: Option<&str>, data: T) -> ModelResult<T> {
        let actual_id = id.map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());

        self.with_model_write(model_name, |model| {
            let data_arc = Arc::new(data);
            let result = (*data_arc).clone();
            model.insert(actual_id, data_arc);
            result
        })
    }

    fn update(&self, model_name: &str, id: &str, data: T) -> ModelResult<T> {
        let models = self.models.read()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

        let model = match models.get(model_name) {
            Some(model) => model,
            None => {
                return Err(ModelError::NotFound(id.to_string()));
            }
        };

        if model.contains_key(id) {
            drop(models);

            let mut models = self.models.write()
                .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

            let model = models.get_mut(model_name).unwrap(); 
            let data_arc = Arc::new(data);
            let result = (*data_arc).clone();
            model.insert(id.to_string(), data_arc);
            Ok(result)
        } else {
            Err(ModelError::NotFound(id.to_string()))
        }
    }

    fn get(&self, model_name: &str, id: &str) -> ModelResult<T> {
        let models = self.models.read()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

        let model = match models.get(model_name) {
            Some(model) => model,
            None => {
                return Err(ModelError::NotFound(id.to_string()));
            }
        };

        model.get(id)
            .map(|arc| (**arc).clone())
            .ok_or_else(|| ModelError::NotFound(id.to_string()))
    }

    fn remove(&self, model_name: &str, id: &str) -> ModelResult<T> {
        let models = self.models.read()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

        let model = match models.get(model_name) {
            Some(model) => model,
            None => {
                return Err(ModelError::NotFound(id.to_string()));
            }
        };

        if model.contains_key(id) {
            drop(models);

            let mut models = self.models.write()
                .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

            let model = models.get_mut(model_name).unwrap(); 
            model.remove(id)
                .map(|arc| (*arc).clone())
                .ok_or_else(|| ModelError::NotFound(id.to_string()))
        } else {
            Err(ModelError::NotFound(id.to_string()))
        }
    }

    fn iter_all(&self, model_name: &str) -> ModelResult<Box<dyn ExactSizeIterator<Item = ModelResult<T>> + Send>> {
        let models = self.models.read()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

        let model = match models.get(model_name) {
            Some(model) => model,
            None => {
                let empty_vec: Vec<T> = Vec::new();
                return Ok(Box::new(ModelIterator::new(empty_vec)));
            }
        };

        let values: Vec<T> = model.values()
            .map(|arc| (**arc).clone())
            .collect();

        Ok(Box::new(ModelIterator::new(values)))
    }

    fn get_all(&self, model_name: &str) -> ModelResult<Vec<T>> {
        let models = self.models.read()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

        // Si el modelo no existe, retornar vector vacío en lugar de error
        match models.get(model_name) {
            Some(model) => {
                let values: Vec<T> = model.values()
                    .map(|arc| (**arc).clone())
                    .collect();
                Ok(values)
            }
            None => Ok(Vec::new()),
        }
    }

    fn get_by_path(&self, model_name: &str, id: &str, path: &str) -> ModelResult<Option<T>> {
        let record = self.get(model_name, id)?;
        record.get_by_path(path)
    }

    fn find_by_path_exists(&self, model_name: &str, path: &str) -> ModelResult<Vec<T>> {
        let models = self.models.read()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

        let model = match models.get(model_name) {
            Some(model) => model,
            None => return Ok(Vec::new()),
        };

        let results: Vec<T> = model.values()
            .filter_map(|arc| {
                match arc.has_path(path) {
                    Ok(true) => Some((**arc).clone()),
                    _ => None,
                }
            })
            .collect();

        Ok(results)
    }

    fn find_by_path_value(&self, model_name: &str, path: &str, expected_value: T) -> ModelResult<Vec<T>> {
        let models = self.models.read()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

        let model = match models.get(model_name) {
            Some(model) => model,
            None => return Ok(Vec::new()),
        };

        let results: Vec<T> = model.values()
            .filter_map(|arc| {
                match arc.get_by_path(path) {
                    Ok(Some(value)) => {
                        match value.equals_fast(&expected_value) {
                            Ok(true) => Some((**arc).clone()),
                            _ => None,
                        }
                    },
                    _ => None,
                }
            })
            .collect();

        Ok(results)
    }

    fn count(&self, model_name: &str) -> ModelResult<usize> {
        let models = self.models.read()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;

        match models.get(model_name) {
            Some(model) => Ok(model.len()),
            None => Ok(0),
        }
    }

    fn exists(&self, model_name: &str, id: &str) -> ModelResult<bool> {
        let models = self.models.read()
            .map_err(|_| ModelError::SystemError("Lock poisoned".to_string()))?;
        
        match models.get(model_name) {
            Some(model) => Ok(model.contains_key(id)),
            None => Ok(false),
        }
    }
}

unsafe impl<T: DynamicValue> Send for SimpleModelManager<T> {}
unsafe impl<T: DynamicValue> Sync for SimpleModelManager<T> {}