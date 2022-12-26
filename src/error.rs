//! Errors that can be encounted while using announce.

use thiserror::Error;

/// Contains every Error which can be encountered in announce.
#[derive(Error, Debug)]
pub enum Error {
    /// Error while handling requests
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[cfg(feature = "dbus")]
    /// Error while handling zbus library
    #[error("Zbus error: {0}")]
    Zbus(#[from] zbus::Error),

    /// Error when no matching schema was found
    #[error("Schema does not match a supported service (create an issue for a new service)")]
    NoMatchingSchema,

    /// Using the wrong scheme for a service
    #[error("Wrong scheme form service: {0}")]
    WrongScheme(String),

    /// A parse error froma  missformed url
    #[error("Url parse error: {0}")]
    Parse(#[from] url::ParseError),

    /// A valid Url wich misses a needed field
    #[error("missing url field: {0}")]
    MissingField(String),

    /// Error while converting to json
    #[error("Cannot convert to json")]
    SerdeJson(#[from] serde_json::Error),

    /// Error with Io
    #[error("Error handling io")]
    Fs(#[from] std::io::Error),

    /// A catch all error when no other is applicable
    #[error("An Error occured: {0}")]
    Generic(String),
}
