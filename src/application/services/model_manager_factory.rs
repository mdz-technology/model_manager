use crate::{AsyncDynamicValue, AsyncModelManager};

pub trait ModelManagerFactory<T: AsyncDynamicValue> {
    type Manager: AsyncModelManager<T> + Send + Sync;
    fn create() -> Self::Manager;
}