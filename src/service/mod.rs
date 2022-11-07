use thiserror::Error;

mod rocketchat;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("hyper error: {0}")]
    HyperError(#[from] hyper::Error),
    #[error("Rocket.Chat error: {0}")]
    RocketChat(#[from] rocketchat::Error),
    #[error("Faulty uri given")]
    ParseError(#[from] http::uri::InvalidUri),
    #[error("Schema does not match a supported service (create an issue for a new service)")]
    NoMatchingSchema,
}

pub trait Service<Error> {
    fn schema() -> Vec<&'static str>;
    fn request(
        target: &str,
        msg: &super::Message,
    ) -> Result<(hyper::Request<hyper::Body>, bool), Error>;
    fn match_schema(target: &str) -> bool {
        Self::schema().iter().any(|s| target.starts_with(s))
    }
    fn create_builder() -> http::request::Builder {
        hyper::Request::builder().header(
            "user-agent",
            format!("announce/{}", env!("CARGO_PKG_VERSION")),
        )
    }
}

pub fn decide_service(
    target: &str,
    msg: &super::Message,
) -> Result<(http::Request<hyper::Body>, bool), ServiceError> {
    //cascade of services
    if rocketchat::RocketChat::match_schema(target) {
        return Ok(rocketchat::RocketChat::request(target, msg)?);
    }

    Err(ServiceError::NoMatchingSchema)
}
