pub mod application;
pub mod infrastructure;

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

pub type DefaultFactory =
    infrastructure::factories::default_model_manager_factory::DefaultModelManagerFactory;
pub type DefaultValue = infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;
pub type DefaultConverterFactory =
    infrastructure::factories::default_converter_factory::DefaultConverterFactory;
pub type DefaultJsonConverter = infrastructure::implementations::json_converter::SerdeJsonConverter;
pub type DefaultObjectIterator =
    infrastructure::implementations::object_iterator::SerdeObjectIterator;
pub type DefaultArrayIterator =
    infrastructure::implementations::array_iterator::SerdeArrayIterator;
pub type DefaultObjectIteratorFactory =
    infrastructure::factories::default_iterator_factory::DefaultObjectIteratorFactory;
pub type DefaultArrayIteratorFactory =
    infrastructure::factories::default_iterator_factory::DefaultAsyncArrayIteratorFactory;
pub type DefaultIteratorFactory =
    infrastructure::factories::default_iterator_factory::DefaultIteratorFactory;
