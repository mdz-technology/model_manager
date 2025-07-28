use crate::application::services::iterator_factory::IteratorFactory;
use crate::infrastructure::implementations::serde_array_iterator::SerdeArrayIterator;
use crate::infrastructure::implementations::serde_object_iterator::SerdeObjectIterator;
use crate::infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;

pub struct DefaultIteratorFactory;

impl IteratorFactory<SerdeDynamicValue> for DefaultIteratorFactory {
    type ObjectIterator = SerdeObjectIterator;
    type ArrayIterator = SerdeArrayIterator;

    fn create_object_iterator(data: SerdeDynamicValue) -> Result<Self::ObjectIterator, String> {
        data.iter_object().map_err(|e| e.to_string())
    }

    fn create_array_iterator(data: SerdeDynamicValue) -> Result<Self::ArrayIterator, String> {
        data.iter_array().map_err(|e| e.to_string())
    }
}
