//! Rocket.Chat is an open source team chat platform.
//!
//! * Homepage: <https://www.rocket.chat/>
//! * Reference API: <https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage>

/// Message is to be used with this module
pub mod message;
mod tests;

use crate::message::Message as CrateMessage;
use crate::service::Service;
pub use message::Message;

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
    fn from_url(url: &reqwest::Url) -> Result<Self, crate::Error> {
        if !Self::match_scheme(url) {
            return Err(crate::Error::WrongScheme(format!(
                "found \"{}\"",
                url.scheme()
            )));
        }

        //extract from url
        let https = url.scheme() == "rocketchats";
        let user = url.username();
        let token = url
            .password()
            .ok_or_else(|| crate::Error::MissingField(String::from("password")))?;
        let host = url
            .host_str()
            .ok_or_else(|| crate::Error::MissingField(String::from("host")))?;
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
    fn build_url(&self) -> Result<reqwest::Url, crate::Error> {
        let url = match (self.https, self.port) {
            (true, None) => format!("https://{}/api/v1/chat.postMessage", self.host),
            (false, None) => format!("http://{}/api/v1/chat.postMessage", self.host),
            (true, Some(p)) => format!("https://{}:{}/api/v1/chat.postMessage", self.host, p),
            (false, Some(p)) => format!("http://{}:{}/api/v1/chat.postMessage", self.host, p),
        };
        Ok(reqwest::Url::parse(&url)?)
    }

    /// Accepts the following url formattings:
    /// * rocketchat://USER:TOKEN@HOST{:PORT}
    /// * rocketchats://USER:TOKEN@HOST{:PORT}
    /// # Example
    /// ```no_run
    /// use announce::service::rocketchat;
    ///
    /// let client = reqwest::Client::new();
    /// let url = "rocketchats://user:token@host.com";
    /// let url = reqwest::Url::parse(url).unwrap();
    /// let msg = rocketchat::Message::new("some_channel");
    /// // modify msg to your liking
    ///
    /// rocketchat::RocketChat::announce(&client, &url, &msg);
    /// ```
    pub async fn announce(
        client: &reqwest::Client,
        url: &reqwest::Url,
        msg: &Message<'_>,
    ) -> Result<reqwest::Response, crate::Error> {
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
        log::trace!("request: {:?}", req);

        Ok(client.execute(req).await?)
    }
}

impl super::Service for RocketChat {
    fn schema() -> Vec<&'static str> {
        vec!["rocketchat", "rocketchats"]
    }

    /// This is used by the announce method in [crate].
    /// Allowed url's are:
    /// * rocketchat://USER:TOKEN@HOSTNAME/CHANNEL
    /// * rocketchats://USER:TOKEN@HOSTNAME/CHANNEL
    #[doc(hidden)]
    fn build_request(
        announce: &crate::Announce,
        url: &reqwest::Url,
        msg: &CrateMessage,
    ) -> Result<super::ServiceResult, crate::Error> {
        let info = Self::from_url(url)?;
        let url = info.build_url()?;

        //build body from msg
        let mut body = Message::new(
            &info
                .channel
                .ok_or_else(|| crate::Error::MissingField(String::from("channel")))?,
        );
        body.populate(msg);

        //build request
        let builder = announce.client.request(reqwest::Method::POST, url);
        let req = builder
            .header("x-auth-token", info.token)
            .header("x-user-id", info.user)
            .header("content-type", "applicatioin/json")
            .json(&body)
            .build()?;
        log::trace!("{:?}", req);

        Ok(super::ServiceResult::Reqwest(req))
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
