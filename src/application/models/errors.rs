use thiserror::Error;

pub type ModelResult<T> = Result<T, ModelError>;

#[derive(Error, Debug, Clone)]
pub enum ModelError {
    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("System error: {0}")]
    SystemError(String),
    
}