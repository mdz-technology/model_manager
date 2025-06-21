use super::super::models::errors::ModelResult;
use super::dynamic_value::DynamicValue;

pub trait DataConverter<T: DynamicValue> {
    type Input;
    type Output;

    fn from_external(&self, input: Self::Input) -> ModelResult<T>;
    fn to_external(&self, value: &T) -> ModelResult<Self::Output>;
    fn from_external_batch(&self, inputs: Vec<Self::Input>) -> ModelResult<Vec<T>>;
    fn to_external_batch(&self, values: &[T]) -> ModelResult<Vec<Self::Output>>;
}

pub trait JsonConverter<T: DynamicValue>: DataConverter<T, Input = String, Output = String> {
    fn from_json_string(&self, json: String) -> ModelResult<T> {
        self.from_external(json)
    }

    fn to_json_string(&self, value: &T) -> ModelResult<String> {
        self.to_external(value)
    }

    fn to_json_pretty(&self, value: &T) -> ModelResult<String>;
}