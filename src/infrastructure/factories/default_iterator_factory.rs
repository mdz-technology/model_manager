use crate::application::services::iterator_factory::{
    ArrayIteratorFactory, IteratorFactory, ObjectIteratorFactory,
};
use crate::infrastructure::implementations::array_iterator::SerdeArrayIterator;
use crate::infrastructure::implementations::object_iterator::SerdeObjectIterator;
use crate::infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;
pub struct DefaultObjectIteratorFactory;

impl ObjectIteratorFactory<SerdeDynamicValue> for DefaultObjectIteratorFactory {
    type ObjectIterator = SerdeObjectIterator;

    fn create_object_iterator(data: SerdeDynamicValue) -> Result<Self::ObjectIterator, String> {
        data.iter_object().map_err(|e| e.to_string())
    }
}

pub struct DefaultAsyncArrayIteratorFactory;

impl ArrayIteratorFactory<SerdeDynamicValue> for DefaultAsyncArrayIteratorFactory {
    type ArrayIterator = SerdeArrayIterator;

    fn create_array_iterator(data: SerdeDynamicValue) -> Result<Self::ArrayIterator, String> {
        data.iter_array().map_err(|e| e.to_string())
    }
}

pub struct DefaultIteratorFactory;

impl IteratorFactory<SerdeDynamicValue> for DefaultIteratorFactory {
    type ObjectIterator = SerdeObjectIterator;
    type ArrayIterator = SerdeArrayIterator;

    fn create_object_iterator(data: SerdeDynamicValue) -> Result<Self::ObjectIterator, String> {
        DefaultObjectIteratorFactory::create_object_iterator(data)
    }

    fn create_array_iterator(data: SerdeDynamicValue) -> Result<Self::ArrayIterator, String> {
        DefaultAsyncArrayIteratorFactory::create_array_iterator(data)
    }
}
