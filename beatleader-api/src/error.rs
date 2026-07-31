use reqwest::StatusCode;

use crate::AuthError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("{error:?}\ndeserialization text:\n{text}")]
    Deserialization {
        error: serde_json::Error,
        text: String,
    },
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error("{code}, Text: {text}")]
    HttpError { code: StatusCode, text: String },
}

pub type Result<T> = std::result::Result<T, Error>;
