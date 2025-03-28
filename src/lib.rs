
pub mod dynamic;
pub mod model;


pub use dynamic::application::dynamic_value::DynamicValue;
pub use model::application::model_manager::ModelManager;

pub use dynamic::infrastructure::dynamic_value_impl::DynamicValueImpl;
pub use model::infrastructure::model_manager_impl::ModelManagerImpl;

pub use dynamic::application::dynamic_value_converter::DynamicValueConverter;
pub use dynamic::infrastructure::json_to_dynamic_value_converter::JsonToDynamicValueConverter;

