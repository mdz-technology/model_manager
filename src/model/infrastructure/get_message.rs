use std::marker::PhantomData;
use actix::{Handler, Message};
use crate::dynamic::application::dynamic_value::DynamicValue;
use crate::model::infrastructure::model_actor::ModelActor;

#[derive(Message)]
#[rtype(result = "Result<T, String>")]
pub struct GetMessage<T: DynamicValue + 'static> {
    pub id: String,
    pub _marker: PhantomData<T>,
}

impl<T: DynamicValue + Unpin + 'static> Handler<GetMessage<T>> for ModelActor<T> {
    type Result = Result<T, String>;

    fn handle(&mut self, message: GetMessage<T>, _ctx: &mut Self::Context) -> Self::Result {
        let id = message.id;
        match self.get(&id) {
            Some(value) => Ok(value),
            None => Err(format!("Elemento con id '{}' no encontrado", id)),
        }
    }

}