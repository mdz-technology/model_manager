use serde_json::from_str;
use crate::dynamic::application::dynamic_value_converter::DynamicValueConverter;
use crate::DynamicValueImpl;

pub struct JsonToDynamicValueConverter;

impl DynamicValueConverter<&str> for JsonToDynamicValueConverter {
    type Output = DynamicValueImpl;

    fn convert(&self, input: &str) -> Result<Self::Output, String> {
        match from_str(input) {
            Ok(value) => Ok(DynamicValueImpl::from_serde_value(value)),
            Err(err) => Err(err.to_string()),
        }
    }
}