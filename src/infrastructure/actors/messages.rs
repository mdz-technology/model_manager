use actix::prelude::*;
use crate::DynamicValue;
use crate::ModelResult;

#[derive(Message)]
#[rtype(result = "ModelResult<T>")]
pub struct InsertMessage<T: DynamicValue> {
    pub id: Option<String>,
    pub data: T,
}

#[derive(Message)]
#[rtype(result = "ModelResult<T>")]
pub struct UpdateMessage<T: DynamicValue> {
    pub id: String,
    pub data: T,
}

#[derive(Message)]
#[rtype(result = "ModelResult<T>")]
pub struct GetMessage<T: DynamicValue> {
    pub id: String,
    pub _phantom: std::marker::PhantomData<T>,
}

#[derive(Message)]
#[rtype(result = "ModelResult<T>")]
pub struct RemoveMessage<T: DynamicValue> {
    pub id: String,
    pub _phantom: std::marker::PhantomData<T>,
}

#[derive(Message)]
#[rtype(result = "ModelResult<Vec<T>>")]
pub struct GetAllMessage<T: DynamicValue> {
    pub _phantom: std::marker::PhantomData<T>,
}