use crate::application::traits::data_converter::{JsonConverter};
use crate::application::models::converter_config::{JsonConverterConfig};
use crate::DynamicValue;

pub trait ConverterFactory<T: DynamicValue> {
    type JsonConverter: JsonConverter<T> + Send + Sync;
    
    fn create_json_converter() -> Self::JsonConverter;
    
    fn create_json_converter_with_config(config: JsonConverterConfig) -> Self::JsonConverter;
}