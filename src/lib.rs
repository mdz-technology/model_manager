pub mod application;
mod infrastructure;

pub use application::traits::{
    data_converter::{DataConverter, JsonConverter},
    dynamic_value::DynamicValue,
    model_manager::ModelManager,
};

pub use application::models::converter_config::{ConverterConfig, JsonConverterConfig};
pub use application::models::errors::{ModelError, ModelResult};

pub use application::services::{
    iterator_factory::{
        ArrayIteratorFactory, IteratorFactory, ObjectIteratorFactory,
    },
    converter_factory::ConverterFactory,
    dynamic_value_factory::DynamicValueFactory,
    model_manager_factory::ModelManagerFactory,
};

pub use application::iterators::{
    array_iterator::ArrayIterator, object_iterator::ObjectIterator,
};

pub type DefaultModelManager = infrastructure::factories::default_model_manager_factory::DefaultModelManagerFactory;
pub type DefaultValue = infrastructure::factories::default_value_factory::DefaultValueFactory;
pub type DefaultConverter = infrastructure::factories::default_converter_factory::DefaultConverterFactory;
pub type DefaultIteratorFactory = infrastructure::factories::default_iterator_factory::DefaultIteratorFactory;
