use crate::infrastructure::actors::messages::{FindByPathExistsMessage, FindByPathValueMessage, GetAllMessage,
                                              GetByPathMessage, GetMessage, InsertMessage, RemoveMessage, UpdateMessage};
use crate::{DynamicValue, ModelError, ModelResult};
use actix::{Actor, Context, Handler, ResponseActFuture, WrapFuture};
use std::collections::HashMap;
use uuid::Uuid;

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

impl<T: DynamicValue + Unpin> Handler<GetByPathMessage<T>> for ModelActor<T> {
    type Result = ResponseActFuture<Self, ModelResult<Option<T>>>;

    fn handle(&mut self, msg: GetByPathMessage<T>, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(record) = self.data.get(&msg.id).cloned() {
            let path = msg.path;

            Box::pin(
                async move {
                    record.get_by_path(&path).await
                }.into_actor(self)
            )
        } else {
            let id = msg.id;
            Box::pin(
                async move {
                    Err(ModelError::NotFound(id))
                }.into_actor(self)
            )
        }
    }
}

impl<T: DynamicValue + Unpin> Handler<FindByPathExistsMessage<T>> for ModelActor<T> {
    type Result = ResponseActFuture<Self, ModelResult<Vec<T>>>;

    fn handle(&mut self, msg: FindByPathExistsMessage<T>, _ctx: &mut Context<Self>) -> Self::Result {
        let records: Vec<T> = self.data.values().cloned().collect();
        let path = msg.path;

        Box::pin(
            async move {
                let mut results = Vec::new();

                for record in records {
                    match record.has_path(&path).await {
                        Ok(true) => results.push(record),
                        Ok(false) => continue,
                        Err(e) => return Err(e),
                    }
                }

                Ok(results)
            }.into_actor(self)
        )
    }
}

impl<T: DynamicValue + Unpin> Handler<FindByPathValueMessage<T>> for ModelActor<T> {
    type Result = ResponseActFuture<Self, ModelResult<Vec<T>>>;

    fn handle(&mut self, msg: FindByPathValueMessage<T>, _ctx: &mut Context<Self>) -> Self::Result {
        let records: Vec<T> = self.data.values().cloned().collect();
        let path = msg.path;
        let expected_value = msg.expected_value;

        Box::pin(
            async move {
                let mut results = Vec::new();

                for record in records {
                    match record.get_by_path(&path).await {
                        Ok(Some(value)) => {
                            // ✅ Production: Use proper value comparison
                            if Self::values_equal(&value, &expected_value).await {
                                results.push(record);
                            }
                        }
                        Ok(None) => continue,
                        Err(e) => return Err(e),
                    }
                }

                Ok(results)
            }.into_actor(self)
        )
    }
}

impl<T: DynamicValue + Unpin> ModelActor<T> {
    async fn values_equal(value1: &T, value2: &T) -> bool {
        value1.to_string() == value2.to_string()
    }
}
