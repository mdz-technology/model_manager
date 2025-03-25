use crate::dynamic::application::dynamic_value::DynamicValue;

pub trait ModelManager<T : DynamicValue> {

    fn new() -> Self;

    async fn insert(
        &mut self,
        model_name: String,
        id: Option<String>,
        data: T,
    ) -> Result<T, String>;

    async fn update(
        &mut self,
        model_name: String,
        id: String,
        data: T,
    ) -> Result<T, String>;

    async fn get(
        &mut self,
        model_name: String,
        id: String
    ) -> Result<T, String>;

    async fn remove(
        &mut self,
        model_name: String,
        id: String
    ) -> Result<T, String>;

    async fn get_all(
        &mut self,
        model_name: String
    ) -> Result<Vec<T>, String>;

}