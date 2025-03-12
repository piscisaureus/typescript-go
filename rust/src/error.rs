use thiserror::Error;

#[derive(Error, Debug)]
pub enum TsError {
    #[error("syntax error: {0}")]
    SyntaxError(String),
    
    #[error("type error: {0}")]
    TypeError(String),
    
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TsError>;