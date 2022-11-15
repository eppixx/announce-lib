//! Rocket.Chat is a open source team chat platform
//!
//! * Homepage: <https://www.rocket.chat/>
//! * Reference API: <https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage>

use thiserror::Error;

/// Message is to be used with this module
pub mod message;
mod tests;

use crate::message::Message as CrateMessage;
use crate::service::Service;
pub use message::Message;

/// Errors which can be encountered for this module
#[derive(Error, Debug)]
pub enum Error {
    /// A reqwest error
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// A parse error from a missformed url
    #[error("url parse error: {0}")]
    Parse(#[from] url::ParseError),

    /// A valid Url which misses a needed field
    #[error("missing url field: {0}")]
    Url(String),

    /// Using the wrong schema with service
    #[error("Wrong scheme for service: {0}")]
    WrongScheme(String),
}

/// A implementation of messaging to a Rocket.Chat instance
pub struct RocketChat {}

impl<'a> RocketChat {
    /// Accepts the following url formatings:
    /// * rocketchat://user:token@host{:port}
    /// * rocketchats://user:token@host{:port}
    /// # Example
    /// ```no_run
    /// use announce::service::rocketchat;
    ///
    /// let client = reqwest::Client::new();
    /// let url = "rocketchats://user:token@host.com";
    /// let url = url::Url::parse(url).unwrap();
    /// let msg = rocketchat::Message::new("some_channel");
    /// // modify msg to your liking
    ///
    /// rocketchat::RocketChat::announce(&client, &url, &msg);
    /// ```
    pub async fn announce(
        client: &reqwest::Client,
        url: &url::Url,
        msg: &Message<'_>,
    ) -> Result<reqwest::Response, Error> {
        if !Self::match_scheme(url) {
            return Err(Error::WrongScheme(format!("found \"{}\"", url.scheme())));
        }
        //extract from url
        let https = url.scheme() == "rocketchats";
        let user = url.username();
        let token = url
            .password()
            .ok_or_else(|| Error::Url(String::from("no password given")))?;
        let host = url
            .host_str()
            .ok_or_else(|| Error::Url(String::from("no Host")))?;
        let port = url.port();

        //build url
        let url = match (https, port) {
            (true, None) => format!("https://{}/api/v1/chat.postMessage", host),
            (false, None) => format!("http://{}/api/v1/chat.postMessage", host),
            (true, Some(p)) => format!("https://{}:{}/api/v1/chat.postMessage", host, p),
            (false, Some(p)) => format!("http://{}:{}/api/v1/chat.postMessage", host, p),
        };
        let url = reqwest::Url::parse(&url)?;

        //build request
        let builder = client.request(reqwest::Method::POST, url);
        let req = builder
            .header("x-auth-token", token)
            .header("x-user-id", user)
            .header("content-type", "applicatioin/json")
            .json(&msg)
            .build()?;

        Ok(client.execute(req).await?)
    }
}

impl super::Service<Error> for RocketChat {
    fn schema() -> Vec<&'static str> {
        vec!["rocketchat", "rocketchats"]
    }

    /// This is used by the announce method in [crate].
    /// Allowed url's are:
    /// * rocketchat://USER:TOKEN@HOSTNAME/CHANNEL
    /// * rocketchats://USER:TOKEN@HOSTNAME/CHANNEL
    #[doc(hidden)]
    fn build_request(
        client: &reqwest::Client,
        url: &url::Url,
        msg: &CrateMessage,
    ) -> Result<reqwest::Request, Error> {
        //extract information from target
        let https = url.scheme() == "rocketchats";
        let user = url.username();
        let token = url
            .password()
            .ok_or_else(|| Error::Url(String::from("no password given")))?;
        let host = url.host_str().unwrap();
        let port = url.port();
        let channel = url
            .path_segments()
            .ok_or_else(|| Error::Url(String::from("no channel given")))?
            .next()
            .ok_or_else(|| Error::Url(String::from("no channel given")))?;

        //build url
        let url = match (https, port) {
            (true, None) => format!("https://{}/api/v1/chat.postMessage", host),
            (false, None) => format!("http://{}/api/v1/chat.postMessage", host),
            (true, Some(p)) => format!("https://{}:{}/api/v1/chat.postMessage", host, p),
            (false, Some(p)) => format!("http://{}:{}/api/v1/chat.postMessage", host, p),
        };
        let url = reqwest::Url::parse(&url)?;

        //build body from msg
        let mut body = Message::new(channel);
        body.populate(msg);

        //build request
        let builder = client.request(reqwest::Method::POST, url);
        let req = builder
            .header("x-auth-token", token)
            .header("x-user-id", user)
            .header("content-type", "applicatioin/json")
            .json(&body)
            .build()?;

        dbg!(body);
        dbg!(&req);

        Ok(req)
    }
}
