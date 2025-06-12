use crate::{DynamicValue, ModelManager};

pub trait ModelManagerFactory<T: DynamicValue> {
    type Manager: ModelManager<T> + Send + Sync;
    fn create() -> Self::Manager;
}