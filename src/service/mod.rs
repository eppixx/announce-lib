//! Contains the services which are supportet by anounce.

use thiserror::Error;

use crate::message::Message;

pub mod rocketchat;

/// Contains every Error which can be encountered in announce.
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Error while handling requests
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Error while handling RocketChat Api
    #[error("Rocket.Chat error: {0}")]
    RocketChat(#[from] rocketchat::Error),

    /// Error when no matching schema was found
    #[error("Schema does not match a supported service (create an issue for a new service)")]
    NoMatchingSchema,
}

/// A trait implemented for all services
pub trait Service<Error> {
    /// Returns a Vec of supported schemas
    fn schema() -> Vec<&'static str>;

    // shouldn't be used
    // either use the crate::annoucne(..) method
    // or the announce method of a specific service
    #[doc(hidden)]
    fn build_request(
        client: &reqwest::Client,
        target: &url::Url,
        msg: &Message,
    ) -> Result<reqwest::Request, Error>;

    /// Returns true if a given url matches with a schema of a given service
    fn match_scheme(url: &url::Url) -> bool {
        Self::schema().iter().any(|s| &url.scheme() == s)
    }
}

/// Tests url with all services and returns a request if it does
pub fn decide_service(
    client: &reqwest::Client,
    url: &url::Url,
    msg: &Message,
) -> Result<reqwest::Request, ServiceError> {
    //cascade of services
    if rocketchat::RocketChat::match_scheme(url) {
        return Ok(rocketchat::RocketChat::build_request(client, url, msg)?);
    }
    //TODO discord

    Err(ServiceError::NoMatchingSchema)
}
