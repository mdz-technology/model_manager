pub mod application;
pub mod infrastructure;

pub use application::traits::{
    model_manager::ModelManager,
    dynamic_value::DynamicValue,
};

pub use application::models::errors::{ModelError, ModelResult};

pub use application::services::model_manager_factory::ModelManagerFactory;

pub type DefaultFactory = infrastructure::factories::default_model_manager_factory::DefaultModelManagerFactory;
pub type DefaultValue = infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;