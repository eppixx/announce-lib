use thiserror::Error;

mod rocketchat;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Rocket.Chat error: {0}")]
    RocketChat(#[from] rocketchat::Error),
    #[error("Schema does not match a supported service (create an issue for a new service)")]
    NoMatchingSchema,
}

pub trait Service<Error> {
    fn schema() -> Vec<&'static str>;
    fn request(
        client: &reqwest::Client,
        target: &str,
        msg: &super::Message,
    ) -> Result<reqwest::Request, Error>;
    fn match_schema(target: &str) -> bool {
        Self::schema().iter().any(|s| target.starts_with(s))
    }
}

pub fn decide_service(
    client: &reqwest::Client,
    target: &str,
    msg: &super::Message,
) -> Result<reqwest::Request, ServiceError> {
    //cascade of services
    if rocketchat::RocketChat::match_schema(target) {
        return Ok(rocketchat::RocketChat::request(client, target, msg)?);
    }
    //TODO discord

    Err(ServiceError::NoMatchingSchema)
}
