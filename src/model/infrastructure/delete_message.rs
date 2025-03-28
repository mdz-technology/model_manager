use std::marker::PhantomData;
use actix::{Handler, Message};
use crate::dynamic::application::dynamic_value::DynamicValue;
use crate::model::infrastructure::model_actor::ModelActor;

#[derive(Message)]
#[rtype(result = "Result<T, String>")]
pub struct DeleteMessage<T:DynamicValue + 'static + Send + Sync> {
    pub id: String,
    pub _marker: PhantomData<T>
}

impl<T: DynamicValue + Unpin + 'static + Send + Sync> Handler<DeleteMessage<T>> for ModelActor<T> {
    type Result = Result<T, String>;

    fn handle(&mut self, message: DeleteMessage<T>, _ctx: &mut Self::Context) -> Self::Result {
        let id = message.id;
        let value_removed = self.get(&id).clone();

        match value_removed {
            Some(_) => {
                self.remove(&id).expect("No se logro eliminar el valor");
                Ok(value_removed.unwrap())
            },
            None => Err(format!("Elemento con id '{}' no encontrado", id))
        }


    }
}
