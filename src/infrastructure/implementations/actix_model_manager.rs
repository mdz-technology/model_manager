use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use actix::{Actor, Addr};
use crate::{AsyncDynamicValue, AsyncModelManager, ModelResult};
use crate::infrastructure::actors::messages::{GetAllMessage, GetMessage, InsertMessage, RemoveMessage, UpdateMessage};
use crate::infrastructure::actors::model_actor::ModelActor;

pub struct ActixModelManager<T: AsyncDynamicValue> {
    actors: HashMap<String, Addr<ModelActor<T>>>,
}

impl<T: AsyncDynamicValue + Unpin> ActixModelManager<T> {

    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
        }
    }

    fn get_or_create_actor(&mut self, model_name: &str) -> Addr<ModelActor<T>> {
        self.actors
            .entry(model_name.to_string())
            .or_insert_with(|| ModelActor::new().start())
            .clone()
    }
}

impl<T: AsyncDynamicValue + Unpin> AsyncModelManager<T> for ActixModelManager<T> {
    fn insert(
        &mut self,
        model_name: String,
        id: Option<String>,
        data: T,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>> {
        let actor = self.get_or_create_actor(&model_name);

        Box::pin(async move {
            let result = actor.send(InsertMessage { id, data }).await?;
            result
        })
    }

    fn update(
        &mut self,
        model_name: String,
        id: String,
        data: T,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>> {
        let actor = self.get_or_create_actor(&model_name);

        Box::pin(async move {
            let result = actor.send(UpdateMessage { id, data }).await?;
            result
        })
    }

    fn get(
        &mut self,
        model_name: String,
        id: String,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>> {
        let actor = self.get_or_create_actor(&model_name);

        Box::pin(async move {
            let result = actor.send(GetMessage {
                id,
                _phantom: std::marker::PhantomData,
            }).await?;
            result
        })
    }

    fn remove(
        &mut self,
        model_name: String,
        id: String,
    ) -> Pin<Box<dyn Future<Output = ModelResult<T>> + Send + '_>> {
        let actor = self.get_or_create_actor(&model_name);

        Box::pin(async move {
            let result = actor.send(RemoveMessage {
                id,
                _phantom: std::marker::PhantomData,
            }).await?;
            result
        })
    }

    fn get_all(
        &mut self,
        model_name: String,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Vec<T>>> + Send + '_>> {
        let actor = self.get_or_create_actor(&model_name);

        Box::pin(async move {
            let result = actor.send(GetAllMessage {
                _phantom: std::marker::PhantomData,
            }).await?;
            result
        })
    }
}