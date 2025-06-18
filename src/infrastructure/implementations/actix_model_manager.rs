use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use actix::{Actor, Addr};
use crate::{DynamicValue, ModelManager, ModelResult};
use crate::infrastructure::actors::messages::{FindByPathExistsMessage, FindByPathValueMessage, GetAllMessage, GetByPathMessage, GetMessage, InsertMessage, RemoveMessage, UpdateMessage};
use crate::infrastructure::actors::model_actor::ModelActor;

pub struct ActixModelManager<T: DynamicValue> {
    actors: HashMap<String, Addr<ModelActor<T>>>,
}

impl<T: DynamicValue + Unpin> ActixModelManager<T> {

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

impl<T: DynamicValue + Unpin> ModelManager<T> for ActixModelManager<T> {
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

    fn get_by_path(
        &mut self,
        model_name: String,
        id: String,
        path: String,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Option<T>>> + Send + '_>> {
        let actor = self.get_or_create_actor(&model_name);
        Box::pin(async move {
            let result = actor.send(GetByPathMessage {
                id,
                path,
                _phantom: std::marker::PhantomData,
            }).await?;
            result
        })
    }

    fn find_by_path_exists(
        &mut self,
        model_name: String,
        path: String,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Vec<T>>> + Send + '_>> {
        let actor = self.get_or_create_actor(&model_name);
        Box::pin(async move {
            let result = actor.send(FindByPathExistsMessage {
                path,
                _phantom: std::marker::PhantomData,
            }).await?;
            result
        })
    }

    fn find_by_path_value(
        &mut self,
        model_name: String,
        path: String,
        expected_value: T,
    ) -> Pin<Box<dyn Future<Output = ModelResult<Vec<T>>> + Send + '_>> {
        let actor = self.get_or_create_actor(&model_name);
        Box::pin(async move {
            let result = actor.send(FindByPathValueMessage {
                path,
                expected_value,
                _phantom: std::marker::PhantomData,
            }).await?;
            result
        })
    }
}