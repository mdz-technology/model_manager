use std::marker::PhantomData;
use actix::{Handler, Message};
use crate::dynamic::application::dynamic_value::DynamicValue;
use crate::model::infrastructure::model_actor::ModelActor;

#[derive(Message)]
#[rtype(result = "Result<Vec<T>, String>")]
pub struct GetAllMessage<T: DynamicValue + 'static + Send + Sync> {
    pub _marker: PhantomData<T>,
}

impl<T: DynamicValue + Unpin + 'static + Send + Sync> Handler<GetAllMessage<T>> for ModelActor<T> {
    type Result = Result<Vec<T>, String>;

    fn handle(&mut self, _message: GetAllMessage<T>, _ctx: &mut Self::Context) -> Self::Result {
        Ok(self.get_all())
    }

}