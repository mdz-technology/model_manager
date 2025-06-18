use std::future::Future;
use std::pin::Pin;

pub trait ObjectIterator: Send + Unpin {
    type Item: Clone + Send + Sync + 'static;
    fn next<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Option<(String, Self::Item)>> + Send + 'a>>;
    fn size_hint(&self) -> (usize, Option<usize>);
    fn find_key<'a>(
        &'a mut self,
        key: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<Self::Item>> + Send + 'a>>;
    fn count<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = usize> + Send + 'a>>;
    fn collect<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Vec<(String, Self::Item)>> + Send + 'a>>;
    fn filter<'a, F, Fut>(
        &'a mut self,
        predicate: F,
    ) -> Pin<Box<dyn Future<Output = Vec<(String, Self::Item)>> + Send + 'a>>
    where
        F: Fn(&str, &Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = bool> + Send;
    fn map<'a, F, Fut, R>(
        &'a mut self,
        mapper: F,
    ) -> Pin<Box<dyn Future<Output = Vec<(String, R)>> + Send + 'a>>
    where
        F: Fn(String, Self::Item) -> Fut + Send + 'a,
        Fut: Future<Output = R> + Send,
        R: Clone + Send + 'a;
}
