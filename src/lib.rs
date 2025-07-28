pub mod application;
pub mod composition;
mod infrastructure;

pub use application::traits::{
    data_converter::{DataConverter, JsonConverter},
    core_value::CoreValue,
    path_navigation::PathNavigation,
    value_introspection::ValueIntrospection,
    value_analysis::ValueAnalysis,
    dynamic_value::DynamicValue,
    model_manager::ModelManager,
};

pub use application::services::{
    iterator_factory::IteratorFactory,
    converter_factory::ConverterFactory,
    value_factory::ValueFactory,
    model_manager_factory::ModelManagerFactory,
};

pub use application::models::{
    errors::{ModelError, ModelResult},
    converter_config::{ConverterConfig, JsonConverterConfig},
};

pub use application::iterators::{
    array_iterator::ArrayIterator, 
    object_iterator::ObjectIterator,
};

pub use composition::{
    DefaultModelManagerFactory,
    DefaultValueFactory,
    DefaultConverterFactory,
    DefaultIteratorFactory,
};