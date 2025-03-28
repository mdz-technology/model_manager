use actix::{Handler, Message};
use crate::dynamic::application::dynamic_value::DynamicValue;
use crate::model::infrastructure::model_actor::{ModelActor};

#[derive(Message)]
#[rtype(result = "Result<T, String>")]
pub struct UpdateMessage<T: DynamicValue + 'static + Send + Sync> {
    pub id: String,
    pub data: T,
}

impl<T: DynamicValue + Unpin + 'static + Send + Sync> Handler<UpdateMessage<T>> for ModelActor<T> {
    type Result = Result<T, String>;

    fn handle(&mut self, message: UpdateMessage<T>, _ctx: &mut Self::Context) -> Self::Result {
        let id = message.id;
        let _ = self.update(&id, message.data);

        match self.get(&id) {
            Some(value) => Ok(value),
            None => Err(format!("Elemento con id '{}' no encontrado", id)),
        }
    }
}