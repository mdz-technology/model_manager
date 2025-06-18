use std::pin::Pin;
use std::future::Future;
use super::super::models::errors::ModelResult;
use super::dynamic_value::DynamicValue;

pub trait DataConverter<T: DynamicValue> {
    type Input;
    type Output;

    fn from_external(
        &self,
        input: Self::Input,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>>;

    fn to_external<'a>(
        &'a self,
        value: &'a T,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Self::Output>> + Send + 'a>>;


    fn from_external_batch(
        &self,
        inputs: Vec<Self::Input>,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Vec<T>>> + Send + '_>>;

    fn to_external_batch<'a>(
        &'a self,
        values: &'a [T],
    ) -> Pin<Box<dyn Future<Output = ModelResult<Vec<Self::Output>>> + Send + 'a>>;
}

pub trait JsonConverter<T: DynamicValue>: DataConverter<T, Input = String, Output = String> {
    fn from_json_string(
        &self,
        json: String,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>> {
        self.from_external(json)
    }

    fn to_json_string<'a>(
        &'a self,
        value: &'a T,
    ) -> Pin<Box<dyn Future<Output = ModelResult<String>> + Send + 'a>> {
        self.to_external(value)
    }

    fn to_json_pretty<'a>(
        &'a self,
        value: &'a T,
    ) -> Pin<Box<dyn Future<Output = ModelResult<String>> + Send + 'a>>;
}