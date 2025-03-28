use crate::dynamic::application::dynamic_value::DynamicValue;

pub trait ModelManager {
    type Value: DynamicValue + Send + Sync;

    fn new() -> Self;

    async fn insert(
        &mut self,
        model_name: String,
        id: Option<String>,
        data: Self::Value,
    ) -> Result<Self::Value, String>;

    async fn update(
        &mut self,
        model_name: String,
        id: String,
        data: Self::Value,
    ) -> Result<Self::Value, String>;

    async fn get(
        &mut self,
        model_name: String,
        id: String
    ) -> Result<Self::Value, String>;

    async fn remove(
        &mut self,
        model_name: String,
        id: String
    ) -> Result<Self::Value, String>;

    async fn get_all(
        &mut self,
        model_name: String
    ) -> Result<Vec<Self::Value>, String>;

}