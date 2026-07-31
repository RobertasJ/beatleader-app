use oauth2::{
    HttpClientError, RequestTokenError, StandardErrorResponse, basic::BasicErrorResponseType,
    url::ParseError,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    RequestTokenError(
        #[from]
        RequestTokenError<
            HttpClientError<oauth2::reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
    #[error(transparent)]
    ParseError(#[from] ParseError),
}
