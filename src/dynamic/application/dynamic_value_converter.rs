use crate::DynamicValue;

pub trait DynamicValueConverter<Input> {
    type Output: DynamicValue;

    fn convert(&self, input: Input) -> Result<Self::Output, String>;
}