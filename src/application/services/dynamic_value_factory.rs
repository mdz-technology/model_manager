use crate::DynamicValue;

pub trait DynamicValueFactory {
    type Value: DynamicValue + Send + Sync;
    fn create() -> Self::Value;
}