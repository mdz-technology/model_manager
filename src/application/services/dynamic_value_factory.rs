use crate::AsyncDynamicValue;

pub trait DynamicValueFactory {
    type Value: AsyncDynamicValue + Send + Sync;
    fn create() -> Self::Value;
}