use std::future::Future;
use std::pin::Pin;

pub trait ArrayIterator: Send {
    type Item: Clone + Send + Sync + 'static;
    fn next(&mut self) -> Option<Self::Item>;
    fn size_hint(&self) -> (usize, Option<usize>);
    fn nth(&mut self, n: usize) -> Option<Self::Item>;
    fn count(&mut self) -> usize;
    fn collect(&mut self) -> Vec<Self::Item>;
    fn find<F>(&mut self, predicate: F) -> Option<Self::Item>
    where
        F: Fn(&Self::Item) -> bool;
    fn filter<F>(&mut self, predicate: F) -> Vec<Self::Item>
    where
        F: Fn(&Self::Item) -> bool;
    fn map<F, R>(&mut self, mapper: F) -> Vec<R>
    where
        F: Fn(Self::Item) -> R,
        R: Clone + Send;
    fn for_each_batch<F>(&mut self, batch_size: usize, processor: F) -> usize
    where
        F: Fn(Vec<Self::Item>);
}
