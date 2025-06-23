use crate::application::iterators::object_iterator::ObjectIterator;
use crate::infrastructure::implementations::serde_core_value::SerdeCoreValue;
use serde_json::{Map, Value};
use std::collections::btree_map::IntoIter;

#[derive(Debug)]
pub struct SerdeObjectIterator {
    inner: IntoIter<String, Value>,
    total_size: usize,
    current_index: usize,
}

impl SerdeObjectIterator {
    pub fn new(map: Map<String, Value>) -> Self {
        let total_size = map.len();
        let btree_map: std::collections::BTreeMap<String, Value> = map.into_iter().collect();

        Self {
            inner: btree_map.into_iter(),
            total_size,
            current_index: 0,
        }
    }
}

impl ObjectIterator for SerdeObjectIterator {
    type Item = SerdeCoreValue;

    fn next(&mut self) -> Option<(String, Self::Item)> {
        if let Some((key, value)) = self.inner.next() {
            self.current_index += 1;
            Some((key, SerdeCoreValue::from_serde_value(value)))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_size.saturating_sub(self.current_index);
        (remaining, Some(remaining))
    }

    fn find_key(&mut self, target_key: &str) -> Option<Self::Item> {
        while let Some((key, value)) = self.inner.next() {
            self.current_index += 1;
            if key == target_key {
                return Some(SerdeCoreValue::from_serde_value(value));
            }
        }
        None
    }

    fn count(&mut self) -> usize {
        let remaining = self.inner.len();
        self.current_index = self.total_size;
        remaining
    }

    fn collect(&mut self) -> Vec<(String, Self::Item)> {
        let mut results = Vec::new();

        while let Some((key, value)) = self.inner.next() {
            self.current_index += 1;
            results.push((key, SerdeCoreValue::from_serde_value(value)));
        }

        results
    }

    fn filter<F>(&mut self, predicate: F) -> Vec<(String, Self::Item)>
    where
        F: Fn(&str, &Self::Item) -> bool,
    {
        let mut results = Vec::new();
        while let Some((key, value)) = self.inner.next() {
            self.current_index += 1;
            let item = SerdeCoreValue::from_serde_value(value);

            if predicate(&key, &item) {
                results.push((key, item));
            }
        }

        results
    }

    fn map<F, R>(&mut self, mapper: F) -> Vec<(String, R)>
    where
        F: Fn(String, Self::Item) -> R,
        R: Clone + Send,
    {
        let mut results = Vec::new();
        while let Some((key, value)) = self.inner.next() {
            self.current_index += 1;
            let item = SerdeCoreValue::from_serde_value(value);
            let mapped_value = mapper(key.clone(), item);
            results.push((key, mapped_value));
        }

        results
    }
}