use actix::{Handler, Message};
use uuid::Uuid;
use crate::dynamic::application::dynamic_value::DynamicValue;
use crate::model::infrastructure::model_actor::{ModelActor};

#[derive(Message)]
#[rtype(result = "Result<T, String>")]
pub struct InsertMessage<T: DynamicValue + 'static> {
    pub id: Option<String>,
    pub data: T,
}

impl<T: DynamicValue + Unpin + 'static> Handler<InsertMessage<T>> for ModelActor<T> {
    type Result = Result<T, String>;

    fn handle(&mut self, message: InsertMessage<T>, _ctx: &mut Self::Context) -> Self::Result {
        let id = match message.id {
            Some(id) => {
                id
            },
            None => {
                Uuid::new_v4().to_string()
            }
        };

        let _ = self.insert(&id, message.data);
        match self.get(&id) {
            Some(value) => Ok(value),
            None => Err(format!("Elemento con id '{}' no encontrado", id)),
        }
    }
}