use serde_json::Value;
use crate::{ModelError, ModelResult};
use crate::application::traits::data_converter::{DataConverter, JsonConverter};
use crate::application::models::converter_config::JsonConverterConfig;
use crate::infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;

pub struct SerdeJsonConverter {
    config: JsonConverterConfig,
}

impl SerdeJsonConverter {
    pub fn new() -> Self {
        Self {
            config: JsonConverterConfig::default(),
        }
    }

    pub fn new_with_config(config: JsonConverterConfig) -> Self {
        Self { config }
    }

    fn parse_json_internal(&self, json_str: &str) -> ModelResult<SerdeDynamicValue> {
        match serde_json::from_str::<Value>(json_str) {
            Ok(value) => Ok(SerdeDynamicValue::from_value(value)),
            Err(e) => Err(ModelError::InvalidData(format!("JSON parse error: {}", e))),
        }
    }

    fn serialize_json_internal(&self, value: &SerdeDynamicValue, pretty: bool) -> ModelResult<String> {
        let json_value = value.clone().into_value();

        let result = if pretty {
            serde_json::to_string_pretty(&json_value)
        } else {
            serde_json::to_string(&json_value)
        };

        match result {
            Ok(json_string) => Ok(json_string),
            Err(e) => Err(ModelError::InvalidData(format!("JSON serialize error: {}", e))),
        }
    }
}

impl DataConverter<SerdeDynamicValue> for SerdeJsonConverter {
    type Input = String;
    type Output = String;

    fn from_external(&self, input: Self::Input) -> ModelResult<SerdeDynamicValue> {
        if input.len() > self.config.base.max_direct_conversion_size {
            return Err(ModelError::InvalidData(format!(
                "JSON too large: {} bytes > {} bytes",
                input.len(),
                self.config.base.max_direct_conversion_size
            )));
        }

        self.parse_json_internal(&input)
    }

    fn to_external(&self, value: &SerdeDynamicValue) -> ModelResult<Self::Output> {
        let pretty = self.config.base.pretty_output;
        self.serialize_json_internal(value, pretty)
    }

    fn from_external_batch(&self, inputs: Vec<Self::Input>) -> ModelResult<Vec<SerdeDynamicValue>> {
        let mut results = Vec::with_capacity(inputs.len());

        for input in inputs {
            match self.parse_json_internal(&input) {
                Ok(value) => results.push(value),
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }

    fn to_external_batch(&self, values: &[SerdeDynamicValue]) -> ModelResult<Vec<Self::Output>> {
        let pretty = self.config.base.pretty_output;
        let mut results = Vec::with_capacity(values.len());

        for value in values {
            match self.serialize_json_internal(value, pretty) {
                Ok(json_string) => results.push(json_string),
                Err(e) => return Err(e),
            }
        }

        Ok(results)
    }
}

impl JsonConverter<SerdeDynamicValue> for SerdeJsonConverter {
    fn to_json_pretty(&self, value: &SerdeDynamicValue) -> ModelResult<String> {
        self.serialize_json_internal(value, true)
    }
}