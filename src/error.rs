use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Serialization/Deserialization error: {0}")]
    Serde(#[from] miniserde::Error),
    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Invalid header: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Something went wrong: {0}")]
    WorkFail(String),
    #[error("No image found: {0}")]
    WorkFailNoImage(String),
    #[error("Clone error")]
    CloneError,
}
