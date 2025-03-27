use actix::{Actor, Addr};
use std::collections::HashMap;
use std::marker::PhantomData;
use crate::dynamic::application::dynamic_value::DynamicValue;
use crate::model::application::model_manager::ModelManager;
use crate::model::infrastructure::model_actor::{ModelActor};
use crate::model::infrastructure::delete_message::DeleteMessage;
use crate::model::infrastructure::get_all_message::GetAllMessage;
use crate::model::infrastructure::get_message::GetMessage;
use crate::model::infrastructure::insert_message::InsertMessage;
use crate::model::infrastructure::update_message::UpdateMessage;

pub struct ModelManagerImpl<T: DynamicValue + Unpin +'static + Send + Sync> {
    models: HashMap<String, Addr<ModelActor<T>>>,
}

impl<T: DynamicValue + Unpin + 'static + Send + Sync> ModelManagerImpl<T> {
    fn get_addr_model_actor(&mut self, model_name: String) -> Addr<ModelActor<T>> {
        self.models
            .entry(model_name)
            .or_insert_with(|| ModelActor::new_object().start())
            .clone()
    }
}

impl<T: DynamicValue + Unpin + 'static + Send + Sync> ModelManager for ModelManagerImpl<T> {
    type Value = T;
    fn new() -> Self {
        ModelManagerImpl {
            models: HashMap::new(),
        }
    }

    async fn insert(&mut self, model_name: String, id: Option<String>, data: Self::Value) -> Result<Self::Value, String> {
        let addr = self.get_addr_model_actor(model_name);

        let result = addr
            .send(InsertMessage {
                id,
                data,
            })
            .await; // -> Result<Result<Value,String>, MailboxError>

        match result {
            Ok(inner_result) => inner_result,
            Err(mailbox_err) => Err(format!("Error en mailbox: {}", mailbox_err)),
        }
    }

    async fn update(&mut self, model_name: String, id: String, data: Self::Value) -> Result<Self::Value, String> {
        let addr = self.get_addr_model_actor(model_name);

        let result = addr
            .send(UpdateMessage {
                id,
                data,
            })
            .await; // -> Result<Result<Value,String>, MailboxError>

        match result {
            Ok(inner_result) => inner_result,
            Err(mailbox_err) => Err(format!("Error en mailbox: {}", mailbox_err)),
        }
    }

    async fn get(&mut self, model_name: String, id: String) -> Result<Self::Value, String> {

        let addr = self.get_addr_model_actor(model_name);

        let result = addr
            .send(GetMessage {
                id,
                _marker: PhantomData,
            })
            .await; // -> Result<Result<Value,String>, MailboxError>

        match result {
            Ok(inner_result) => inner_result,
            Err(mailbox_err) => Err(format!("Error en mailbox: {}", mailbox_err)),
        }
    }

    async fn remove(&mut self, model_name: String, id: String) -> Result<Self::Value, String> {
        let addr = self.get_addr_model_actor(model_name);

        let result = addr.send(
            DeleteMessage {
                id,
                _marker: PhantomData,
            })
            .await;

        match result {
            Ok(inner_result) => inner_result,
            Err(mailbox_err) => Err(format!("Error en mailbox: {}", mailbox_err)),
        }
    }

    async fn get_all(&mut self, model_name: String) -> Result<Vec<Self::Value>, String> {
        let addr = self.get_addr_model_actor(model_name);

        let result = addr.send(
            GetAllMessage {
                _marker: PhantomData,
            })
            .await;

        match result {
            Ok(inner_result) => inner_result,
            Err(mailbox_err) => Err(format!("Error en mailbox: {}", mailbox_err)),
        }
    }
}

