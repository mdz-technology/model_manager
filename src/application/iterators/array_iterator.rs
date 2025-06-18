use std::future::Future;
use std::pin::Pin;

pub trait ArrayIterator: Send + Unpin {
    type Item: Clone + Send + Sync + 'static;
    fn next<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send + 'a>>;
    fn size_hint(&self) -> (usize, Option<usize>);
    fn nth<'a>(
        &'a mut self,
        n: usize,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send + 'a>>;
    fn count<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = usize> + Send + 'a>>;
    fn collect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send + 'a>>;
    fn find<'a, F, Fut>(
        &'a mut self,
        predicate: F,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send + 'a>>
    where
        F: Fn(&Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = bool> + Send;
    fn filter<'a, F, Fut>(
        &'a mut self,
        predicate: F,
    ) -> Pin<Box<dyn Future<Output = Vec<Self::Item>> + Send + 'a>>
    where
        F: Fn(&Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = bool> + Send;
    fn map<'a, F, Fut, R>(
        &'a mut self,
        mapper: F,
    ) -> Pin<Box<dyn Future<Output = Vec<R>> + Send + 'a>>
    where
        F: Fn(Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = R> + Send,
        R: Clone + Send + 'a;
    fn for_each_batch<'a, F, Fut>(
        &'a mut self,
        batch_size: usize,
        processor: F,
    ) -> Pin<Box<dyn Future<Output = usize> + Send + 'a>>
    where
        F: Fn(Vec<Self::Item>) -> Fut + Send + 'a,
        Fut: Future<Output = ()> + Send;
}
