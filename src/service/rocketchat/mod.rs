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
pub struct RocketChat {
    https: bool,
    user: String,
    token: String,
    host: String,
    port: Option<u16>,
    channel: Option<String>,
}

impl RocketChat {
    /// Creates a RocketChat struct from a url
    fn from_url(url: &url::Url) -> Result<Self, Error> {
        if !Self::match_scheme(url) {
            return Err(Error::WrongScheme(format!("found \"{}\"", url.scheme())));
        }

        //extract from url
        let https = url.scheme() == "rocketchats";
        let user = url.username();
        let token = url
            .password()
            .ok_or_else(|| Error::Url(String::from("password")))?;
        let host = url
            .host_str()
            .ok_or_else(|| Error::Url(String::from("host")))?;
        let port = url.port();
        let channel = url.path_segments().map(|mut path| path.next().unwrap());

        Ok(Self {
            https,
            user: String::from(user),
            token: String::from(token),
            host: String::from(host),
            port,
            channel: channel.map(String::from),
        })
    }

    /// Returns the url that the message will be send to
    fn build_url(&self) -> Result<url::Url, Error> {
        let url = match (self.https, self.port) {
            (true, None) => format!("https://{}/api/v1/chat.postMessage", self.host),
            (false, None) => format!("http://{}/api/v1/chat.postMessage", self.host),
            (true, Some(p)) => format!("https://{}:{}/api/v1/chat.postMessage", self.host, p),
            (false, Some(p)) => format!("http://{}:{}/api/v1/chat.postMessage", self.host, p),
        };
        Ok(reqwest::Url::parse(&url)?)
    }

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
        let info = Self::from_url(url)?;
        let url = info.build_url()?;

        //build request
        let builder = client.request(reqwest::Method::POST, url);
        let req = builder
            .header("x-auth-token", info.token)
            .header("x-user-id", info.user)
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
        let info = Self::from_url(url)?;
        let url = info.build_url()?;

        //build body from msg
        let mut body = Message::new(
            &info
                .channel
                .ok_or_else(|| Error::Url(String::from("channel")))?,
        );
        body.populate(msg);

        //build request
        let builder = client.request(reqwest::Method::POST, url);
        let req = builder
            .header("x-auth-token", info.token)
            .header("x-user-id", info.user)
            .header("content-type", "applicatioin/json")
            .json(&body)
            .build()?;

        log::trace!("{:?}", body);
        log::trace!("{:?}", req);

        Ok(req)
    }
}

mod testss {
    #[test]
    fn test_from_url() {
        let url = "rocketchat://user:token@host:3000";
        let url = url::Url::parse(url).unwrap();
        let rocket = super::RocketChat::from_url(&url).unwrap();
        assert_eq!(false, rocket.https);
        assert_eq!(String::from("user"), rocket.user);
        assert_eq!(String::from("token"), rocket.token);
        assert_eq!(String::from("host"), rocket.host);
        assert_eq!(Some(3000u16), rocket.port);
        assert_eq!(None, rocket.channel);

        let url = "rocketchat://user:token@host/";
        let url = url::Url::parse(url).unwrap();
        let rocket = super::RocketChat::from_url(&url).unwrap();
        assert_eq!(None, rocket.port);
        assert_eq!(Some(String::from("")), rocket.channel);

        let url = "rocketchats://user:token@host:3000/channel";
        let url = url::Url::parse(url).unwrap();
        let rocket = super::RocketChat::from_url(&url).unwrap();
        assert_eq!(true, rocket.https);
        assert_eq!(Some(String::from("channel")), rocket.channel);
    }
}
