use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum Error {
    /// IO error
    IO(std::io::Error),
    /// Json error
    Json(serde_json::Error),
    /// HTTP error
    Http(reqwest::Error),
    /// Error response
    Rpc(RpcError),
    /// Response to a request did not have the expected nonce
    NonceMismatch,
    /// Response to a request had a jsonrpc field other than "2.0"
    VersionMismatch,
    /// URL error
    Url(String),
    /// Generic error. TODO: remove in favor of more specific errors.
    Err(String),
}

/// A JSONRPC error object
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcError {
    /// The integer identifier of the error
    pub code: i32,
    /// A string describing the error
    pub message: String,
    /// Additional data specific to the error
    pub data: Option<Box<serde_json::value::RawValue>>,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Http(error)
    }
}
