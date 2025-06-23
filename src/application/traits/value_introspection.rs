use crate::ModelResult;

pub trait ValueIntrospection {
    fn has_property(&self, key: &str) -> ModelResult<bool>;
    fn get_property_type(&self, key: &str) -> ModelResult<Option<String>>;
    fn get_property_names(&self) -> ModelResult<Vec<String>>;
    fn count_properties(&self) -> ModelResult<usize>;
}