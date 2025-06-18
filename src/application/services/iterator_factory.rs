use crate::{ArrayIterator, ObjectIterator, DynamicValue};

pub trait ObjectIteratorFactory<T: DynamicValue> {
    type ObjectIterator: ObjectIterator<Item = T>;

    fn create_object_iterator(data: T) -> Result<Self::ObjectIterator, String>;
}

pub trait ArrayIteratorFactory<T: DynamicValue> {
    type ArrayIterator: ArrayIterator<Item = T>;

    fn create_array_iterator(data: T) -> Result<Self::ArrayIterator, String>;
}

pub trait IteratorFactory<T: DynamicValue> {
    type ObjectIterator: ObjectIterator<Item = T>;
    type ArrayIterator: ArrayIterator<Item = T>;

    fn create_object_iterator(data: T) -> Result<Self::ObjectIterator, String>;
    fn create_array_iterator(data: T) -> Result<Self::ArrayIterator, String>;
}
