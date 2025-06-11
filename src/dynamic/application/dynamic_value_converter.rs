use crate::DynamicValue;

pub trait DynamicValueConverter<T> {
    type Output: DynamicValue;

    fn convert(&self, input: T) -> Result<Self::Output, String>;
}