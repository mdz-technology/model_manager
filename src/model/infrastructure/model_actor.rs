use actix::prelude::*;
use crate::dynamic::application::dynamic_value::{DynamicResult, DynamicValue};

pub struct ModelActor<T: DynamicValue + Send + Sync +'static> {
    state: T,
}

impl<T: DynamicValue + Send + Sync + 'static> ModelActor<T> {
    pub fn new(data: T) -> Self {
        Self { state: data }
    }

    pub fn new_object() -> Self {
        Self::new(T::new_object())
    }

    pub fn insert(&mut self, id: &str, value: T) -> DynamicResult<()> {
        self.state.set(id, value)
    }

    pub fn remove(&mut self, id: &str) -> DynamicResult<()> {
        self.state.remove(id)
    }

    pub fn update(&mut self, id: &str, value: T) -> DynamicResult<()> {
        self.state.set(id, value)
    }

    pub fn get(&self, id: &str) -> Option<T> {
        self.state.get(id)
    }

    pub fn get_all(&self) -> Vec<T> {
        self.state.get_all()
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

}

impl<T: DynamicValue + Unpin + 'static + Send + Sync> Actor for ModelActor<T> {
    type Context = Context<Self>;
}



