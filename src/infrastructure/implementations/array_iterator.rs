use crate::application::iterators::array_iterator::ArrayIterator;
use crate::infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
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

    fn next<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send + 'a>> {
        Box::pin(async move {
            if let Some(value) = self.inner.next() {
                self.current_index += 1;
                Some(SerdeDynamicValue::from_value(value))
            } else {
                None
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.total_size.saturating_sub(self.current_index);
        (remaining, Some(remaining))
    }

    fn nth<'a>(
        &'a mut self,
        target_n: usize,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send + 'a>> {
        Box::pin(async move {
            if let Some(value) = self.inner.nth(target_n) {
                self.current_index += target_n + 1;
                Some(SerdeDynamicValue::from_value(value))
            } else {
                self.current_index = self.total_size;
                None
            }
        })
    }

    fn count<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = usize> + Send + 'a>> {
        Box::pin(async move {
            let remaining = self.inner.len();
            self.current_index = self.total_size;
            remaining
        })
    }

    fn collect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send + 'a>> {
        Box::pin(async move {
            let mut results = Vec::new();

            while let Some(value) = self.inner.next() {
                self.current_index += 1;
                results.push(SerdeDynamicValue::from_value(value));
            }

            results
        })
    }

    fn find<'a, F, Fut>(
        &'a mut self,
        predicate: F,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send + 'a>>
    where
        F: Fn(&Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = bool> + Send,
    {
        Box::pin(async move {
            while let Some(value) = self.inner.next() {
                self.current_index += 1;
                let item = SerdeDynamicValue::from_value(value);

                if predicate(&item).await {
                    return Some(item);
                }
            }
            None
        })
    }

    fn filter<'a, F, Fut>(
        &'a mut self,
        predicate: F,
    ) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send + 'a>>
    where
        F: Fn(&Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = bool> + Send,
    {
        Box::pin(async move {
            let mut results = Vec::new();
            while let Some(value) = self.inner.next() {
                self.current_index += 1;
                let item = SerdeDynamicValue::from_value(value);

                if predicate(&item).await {
                    results.push(item);
                }
            }

            results
        })
    }

    fn map<'a, F, Fut, R>(
        &'a mut self,
        mapper: F,
    ) -> Pin<Box<dyn Future<Output = Vec<R>> + Send + 'a>>
    where
        F: Fn(Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = R> + Send,
        R: Clone + Send + 'a,
    {
        Box::pin(async move {
            let mut results = Vec::new();
            while let Some(value) = self.inner.next() {
                self.current_index += 1;
                let item = SerdeDynamicValue::from_value(value);
                let mapped_value = mapper(item).await;
                results.push(mapped_value);
            }

            results
        })
    }

    fn for_each_batch<'a, F, Fut>(
        &'a mut self,
        batch_size: usize,
        processor: F,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'a>>
    where
        F: Fn(Vec<Self::Item>) -> Fut + Send + 'a,
        Fut: Future<Output = ()> + Send,
    {
        Box::pin(async move {
            let mut total_processed = 0;
            let mut current_batch = Vec::with_capacity(batch_size);

            while let Some(value) = self.inner.next() {
                self.current_index += 1;
                let item = SerdeDynamicValue::from_value(value);
                current_batch.push(item);

                if current_batch.len() == batch_size {
                    processor(current_batch.clone()).await;
                    total_processed += current_batch.len();
                    current_batch.clear();
                }
            }

            if !current_batch.is_empty() {
                processor(current_batch.clone()).await;
                total_processed += current_batch.len();
            }

            total_processed
        })
    }
}
