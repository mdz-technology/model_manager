use crate::dynamic::application::dynamic_value::DynamicValue;
use std::collections::HashMap;
use crate::dynamic::infrastructure::dynamic_value_impl::DynamicValueImpl;

pub struct DynamicData {
    data: Box<dyn DynamicValue>,
}

impl DynamicData {
    pub fn new() -> Self {
        Self {
            data: DynamicValueImpl::from_map(HashMap::new())
        }
    }

    pub fn insert<T: Into<Box<dyn DynamicValue>>>(&mut self, id: &str, value: T) {
        self.data.set(id, value.into());
    }

    pub fn remove(&mut self, id: &str) {
        self.data.remove(id)
    }

    pub fn update<T: Into<Box<dyn DynamicValue>>>(&mut self, id: &str, value: T) {
        self.data.set(id, value.into());
    }

    pub fn get(&self, id: &str) -> Option<Box<dyn DynamicValue>> {
        self.data.get(id).map(|v| v.clone_box())
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

}