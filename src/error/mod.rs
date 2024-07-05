use std::io::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("input parameter is not valid, error: {0}")]
    InputParameterParsingError(#[from] serde_json::Error),

    #[error(transparent)]
    DocxReaderError(#[from] docx_rs::ReaderError),
    
    #[error(transparent)]
    InternalError(#[from] Error),

    #[error("system error: {0}")]
    SystemError(String),

    #[error("unknown error occurred")]
    UnknowError,
}