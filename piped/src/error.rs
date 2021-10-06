use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Network(#[from] reqwest::Error),
    #[error("{0}")]
    ParseResponse(#[from] serde_json::error::Error),
    #[error("{0}")]
    Parseurl(#[from] url::ParseError),
}
