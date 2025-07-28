use crate::application::services::converter_factory::ConverterFactory;
use crate::application::models::converter_config::JsonConverterConfig;
use crate::infrastructure::implementations::{
    serde_dynamic_value::SerdeDynamicValue,
    json_converter::SerdeJsonConverter,
};

pub struct DefaultConverterFactory;

impl ConverterFactory<SerdeDynamicValue> for DefaultConverterFactory {
    type JsonConverter = SerdeJsonConverter;

    fn create_json_converter() -> Self::JsonConverter {
        SerdeJsonConverter::new()
    }

    fn create_json_converter_with_config(config: JsonConverterConfig) -> Self::JsonConverter {
        SerdeJsonConverter::new_with_config(config)
    }
}