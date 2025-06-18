use crate::application::iterators::object_iterator::ObjectIterator;
use crate::infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;
use serde_json::{Map, Value};
use std::collections::btree_map::IntoIter;
use std::future::Future;
use std::pin::Pin;

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
    type Item = SerdeDynamicValue;

    fn next<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Option<(String, Self::Item)>> + Send + 'a>> {
        Box::pin(async move {
            if let Some((key, value)) = self.inner.next() {
                self.current_index += 1;
                Some((key, SerdeDynamicValue::from_value(value)))
            } else {
                None
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_size.saturating_sub(self.current_index);
        (remaining, Some(remaining))
    }

    fn find_key<'a>(
        &'a mut self,
        target_key: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send + 'a>> {
        Box::pin(async move {
            while let Some((key, value)) = self.inner.next() {
                self.current_index += 1;
                if key == target_key {
                    return Some(SerdeDynamicValue::from_value(value));
                }
            }
            None
        })
    }

    fn count<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = usize> + Send + 'a>> {
        Box::pin(async move {
            let remaining = self.inner.len();
            self.current_index = self.total_size;
            remaining
        })
    }

    fn collect<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Vec<(String, Self::Item)>> + Send + 'a>> {
        Box::pin(async move {
            let mut results = Vec::new();

            while let Some((key, value)) = self.inner.next() {
                self.current_index += 1;
                results.push((key, SerdeDynamicValue::from_value(value)));
            }

            results
        })
    }

    fn filter<'a, F, Fut>(
        &'a mut self,
        predicate: F,
    ) -> Pin<Box<dyn Future<Output = Vec<(String, Self::Item)>> + Send + 'a>>
    where
        F: Fn(&str, &Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = bool> + Send,
    {
        Box::pin(async move {
            let mut results = Vec::new();
            while let Some((key, value)) = self.inner.next() {
                self.current_index += 1;
                let item = SerdeDynamicValue::from_value(value);

                if predicate(&key, &item).await {
                    results.push((key, item));
                }
            }

            results
        })
    }

    fn map<'a, F, Fut, R>(
        &'a mut self,
        mapper: F,
    ) -> Pin<Box<dyn Future<Output = Vec<(String, R)>> + Send + 'a>>
    where
        F: Fn(String, Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = R> + Send,
        R: Clone + Send + 'a,
    {
        Box::pin(async move {
            let mut results = Vec::new();
            while let Some((key, value)) = self.inner.next() {
                self.current_index += 1;
                let item = SerdeDynamicValue::from_value(value);
                let mapped_value = mapper(key.clone(), item).await;
                results.push((key, mapped_value));
            }

            results
        })
    }
}
