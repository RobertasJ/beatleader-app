use reqwest::StatusCode;
use serde_path_to_error::Path;

use crate::AuthError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("{error}")]
    Deserialization {
        error: serde_path_to_error::Error<serde_json::Error>,
    },
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error("{code}, Text: {text}")]
    HttpError { code: StatusCode, text: String },
}

pub type Result<T> = std::result::Result<T, Error>;
