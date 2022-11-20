use thiserror::Error;

/// Errors which can be encountered for this module
#[derive(Error, Debug)]
pub enum Error {
    /// A reqwest error
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// A parse error from a missformed url
    #[error("Url parse error: {0}")]
    Parse(#[from] url::ParseError),

    /// A valid Url which misses a needed field
    #[error("missing url field: {0}")]
    Url(String),

    /// Using the wrong schema with service
    #[error("Wrong scheme for service: {0}")]
    WrongScheme(String),
}

pub struct Dbus {}
