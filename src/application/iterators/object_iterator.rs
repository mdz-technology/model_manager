pub trait ObjectIterator: Send {
    type Item: Clone + Send + Sync + 'static;
    fn next(&mut self) -> Option<(String, Self::Item)>;
    fn size_hint(&self) -> (usize, Option<usize>);
    fn find_key(&mut self, key: &str) -> Option<Self::Item>;
    fn count(&mut self) -> usize;
    fn collect(&mut self) -> Vec<(String, Self::Item)>;
    fn filter<F>(&mut self, predicate: F) -> Vec<(String, Self::Item)>
    where
        F: Fn(&str, &Self::Item) -> bool;
    fn map<F, R>(&mut self, mapper: F) -> Vec<(String, R)>
    where
        F: Fn(String, Self::Item) -> R,
        R: Clone + Send;
}