use crate::application::iterators::array_iterator::ArrayIterator;
use crate::infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;
use serde_json::Value;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct SerdeArrayIterator {
    inner: IntoIter<Value>,
    total_size: usize,
    current_index: usize,
}

impl SerdeArrayIterator {
    pub fn new(array: Vec<Value>) -> Self {
        let total_size = array.len();

        Self {
            inner: array.into_iter(),
            total_size,
            current_index: 0,
        }
    }
}

impl ArrayIterator for SerdeArrayIterator {
    type Item = SerdeDynamicValue;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.inner.next() {
            self.current_index += 1;
            Some(SerdeDynamicValue::from_value(value))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_size.saturating_sub(self.current_index);
        (remaining, Some(remaining))
    }

    fn nth(&mut self, target_n: usize) -> Option<Self::Item> {
        if let Some(value) = self.inner.nth(target_n) {
            self.current_index += target_n + 1;
            Some(SerdeDynamicValue::from_value(value))
        } else {
            self.current_index = self.total_size;
            None
        }
    }

    fn count(&mut self) -> usize {
        let remaining = self.inner.len();
        self.current_index = self.total_size;
        remaining
    }

    fn collect(&mut self) -> Vec<Self::Item> {
        let mut results = Vec::new();

        while let Some(value) = self.inner.next() {
            self.current_index += 1;
            results.push(SerdeDynamicValue::from_value(value));
        }

        results
    }

    fn find<F>(&mut self, predicate: F) -> Option<Self::Item>
    where
        F: Fn(&Self::Item) -> bool,
    {
        while let Some(value) = self.inner.next() {
            self.current_index += 1;
            let item = SerdeDynamicValue::from_value(value);

            if predicate(&item) {
                return Some(item);
            }
        }
        None
    }

    fn filter<F>(&mut self, predicate: F) -> Vec<Self::Item>
    where
        F: Fn(&Self::Item) -> bool,
    {
        let mut results = Vec::new();
        while let Some(value) = self.inner.next() {
            self.current_index += 1;
            let item = SerdeDynamicValue::from_value(value);

            if predicate(&item) {
                results.push(item);
            }
        }

        results
    }

    fn map<F, R>(&mut self, mapper: F) -> Vec<R>
    where
        F: Fn(Self::Item) -> R,
        R: Clone + Send,
    {
        let mut results = Vec::new();
        while let Some(value) = self.inner.next() {
            self.current_index += 1;
            let item = SerdeDynamicValue::from_value(value);
            let mapped_value = mapper(item);
            results.push(mapped_value);
        }

        results
    }

    fn for_each_batch<F>(&mut self, batch_size: usize, processor: F) -> usize
    where
        F: Fn(Vec<Self::Item>),
    {
        let mut total_processed = 0;
        let mut current_batch = Vec::with_capacity(batch_size);

        while let Some(value) = self.inner.next() {
            self.current_index += 1;
            let item = SerdeDynamicValue::from_value(value);
            current_batch.push(item);

            if current_batch.len() == batch_size {
                processor(current_batch.clone());
                total_processed += current_batch.len();
                current_batch.clear();
            }
        }

        if !current_batch.is_empty() {
            processor(current_batch.clone());
            total_processed += current_batch.len();
        }

        total_processed
    }
}