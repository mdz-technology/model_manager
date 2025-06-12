use std::collections::HashMap;
use actix::{Actor, Context, Handler};
use uuid::Uuid;
use crate::{DynamicValue, ModelError, ModelResult};
use crate::infrastructure::actors::messages::{GetAllMessage, GetMessage, InsertMessage, RemoveMessage, UpdateMessage};

pub struct ModelActor<T: DynamicValue> {
    data: HashMap<String, T>,
}

impl<T: DynamicValue> ModelActor<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<T: DynamicValue + Unpin> Actor for ModelActor<T> {
    type Context = Context<Self>;
}

impl<T: DynamicValue + Unpin> Handler<InsertMessage<T>> for ModelActor<T> {
    type Result = ModelResult<T>;

    fn handle(&mut self, msg: InsertMessage<T>, _ctx: &mut Context<Self>) -> Self::Result {
        let id = msg.id.unwrap_or_else(|| Uuid::new_v4().to_string());

        let data_clone = msg.data.clone();
        self.data.insert(id, msg.data);

        Ok(data_clone)
    }
}

impl<T: DynamicValue + Unpin> Handler<UpdateMessage<T>> for ModelActor<T> {
    type Result = ModelResult<T>;

    fn handle(&mut self, msg: UpdateMessage<T>, _ctx: &mut Context<Self>) -> Self::Result {
        if self.data.contains_key(&msg.id) {
            let data_clone = msg.data.clone();
            self.data.insert(msg.id, msg.data);
            Ok(data_clone)
        } else {
            Err(ModelError::NotFound(msg.id))
        }
    }
}

impl<T: DynamicValue + Unpin> Handler<GetMessage<T>> for ModelActor<T> {
    type Result = ModelResult<T>;

    fn handle(&mut self, msg: GetMessage<T>, _ctx: &mut Context<Self>) -> Self::Result {
        self.data.get(&msg.id)
            .cloned()
            .ok_or_else(|| ModelError::NotFound(msg.id))
    }
}

impl<T: DynamicValue + Unpin> Handler<RemoveMessage<T>> for ModelActor<T> {
    type Result = ModelResult<T>;

    fn handle(&mut self, msg: RemoveMessage<T>, _ctx: &mut Context<Self>) -> Self::Result {
        self.data.remove(&msg.id)
            .ok_or_else(|| ModelError::NotFound(msg.id))
    }
}

impl<T: DynamicValue + Unpin> Handler<GetAllMessage<T>> for ModelActor<T> {
    type Result = ModelResult<Vec<T>>;

    fn handle(&mut self, _msg: GetAllMessage<T>, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(self.data.values().cloned().collect())
    }
}