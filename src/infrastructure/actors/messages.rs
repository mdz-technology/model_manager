use actix::prelude::*;
use crate::AsyncDynamicValue;
use crate::ModelResult;

#[derive(Message)]
#[rtype(result = "ModelResult<T>")]
pub struct InsertMessage<T: AsyncDynamicValue> {
    pub id: Option<String>,
    pub data: T,
}

#[derive(Message)]
#[rtype(result = "ModelResult<T>")]
pub struct UpdateMessage<T: AsyncDynamicValue> {
    pub id: String,
    pub data: T,
}

#[derive(Message)]
#[rtype(result = "ModelResult<T>")]
pub struct GetMessage<T: AsyncDynamicValue> {
    pub id: String,
    pub _phantom: std::marker::PhantomData<T>,
}

#[derive(Message)]
#[rtype(result = "ModelResult<T>")]
pub struct RemoveMessage<T: AsyncDynamicValue> {
    pub id: String,
    pub _phantom: std::marker::PhantomData<T>,
}

#[derive(Message)]
#[rtype(result = "ModelResult<Vec<T>>")]
pub struct GetAllMessage<T: AsyncDynamicValue> {
    pub _phantom: std::marker::PhantomData<T>,
}