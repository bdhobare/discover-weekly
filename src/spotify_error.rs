use awc::error::{JsonPayloadError, SendRequestError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpotifyError {
    #[allow(dead_code)]
    #[error("User denied request.")]
    AccessDenied(String),
    #[error("Error {0} to redis")]
    RedisError(String),
    #[error("Unable to fetch spotify playlist: {0}")]
    CantFetchPlaylist(String),
    #[error("Unknown error occurred: {msg:?}.")]
    Unknown { msg: Option<String> },
}

impl From<String> for SpotifyError {
    fn from(value: String) -> Self {
        Self::Unknown { msg: value.into() }
    }
}

impl From<SendRequestError> for SpotifyError {
    fn from(value: SendRequestError) -> Self {
        Self::Unknown {
            msg: value.to_string().into(),
        }
    }
}

impl From<JsonPayloadError> for SpotifyError {
    fn from(value: JsonPayloadError) -> Self {
        Self::Unknown {
            msg: value.to_string().into(),
        }
    }
}

impl From<redis::RedisError> for SpotifyError {
    fn from(value: redis::RedisError) -> Self {
        Self::Unknown {
            msg: value.to_string().into(),
        }
    }
}

impl From<serde_json::Error> for SpotifyError {
    fn from(value: serde_json::Error) -> Self {
        Self::Unknown {
            msg: value.to_string().into(),
        }
    }
}

pub type Result<T, E = SpotifyError> = std::result::Result<T, E>;
