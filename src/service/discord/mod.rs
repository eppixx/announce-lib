//! Discord is a VoIP and instant messaging social platform.
//!
//! * Homepage: <https://discord.com/>
//! * Reference API: <https://discord.com/developers/docs/intro>

use thiserror::Error;

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
    #[error("Url parse error: {0}")]
    Parse(#[from] url::ParseError),

    /// A valid Url which misses a needed field
    #[error("missing url field: {0}")]
    Url(String),

    /// Using the wrong schema with service
    #[error("Wrong scheme for service: {0}")]
    WrongScheme(String),
}

/// A implementation of messageing to a Discord channel
pub struct Discord {
    webhook_id: String,
    webhook_token: String,
}

/// A implementation of messaging to Discord
impl Discord {
    /// Creates a Discord struct from a url
    fn from_url(url: &reqwest::Url) -> Result<Self, Error> {
        if !Self::match_scheme(url) {
            return Err(Error::WrongScheme(format!("found \"{}\"", url.scheme())));
        }

        //extract from url
        let id = url
            .host_str()
            .ok_or_else(|| Error::Url(String::from("webhook_id")))?;
        let token = url
            .path_segments()
            .map(|mut path| String::from(path.next().unwrap()))
            .ok_or_else(|| Error::Url(String::from("webhook_token")))?;

        Ok(Self {
            webhook_id: String::from(id),
            webhook_token: token,
        })
    }

    /// Returns the url that the message will be send to
    fn build_url(&self) -> Result<reqwest::Url, Error> {
        let url = format!(
            "https://discord.com/api/webhooks/{}/{}",
            self.webhook_id, self.webhook_token
        );
        Ok(reqwest::Url::parse(&url)?)
    }

    /// Accepts the following url formattings;
    /// * discord://WEBHOOK_ID/WEBHOOK_TOKEN
    /// # Example
    /// ```no_run
    /// use announce::service::discord;
    ///
    /// let client = reqwest::Client::new();
    /// let url = "discord://discord_id/discord_token";
    /// let url = reqwest::Url::parse(url).unwrap();
    /// let mut msg = discord::Message::default();
    /// msg.content = Some("example text");
    /// // modify msg to your linking
    ///
    /// discord::Discord::announce(&client, &url, &msg);
    /// ```
    pub async fn announce(
        client: &reqwest::Client,
        url: &reqwest::Url,
        msg: &Message<'_>,
    ) -> Result<reqwest::Response, Error> {
        let info = Self::from_url(url)?;
        let url = info.build_url()?;

        //build request
        let builder = client.request(reqwest::Method::POST, url);
        let req = builder
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&msg)
            .build()
            .unwrap();
        log::trace!("{:?}", req);

        Ok(client.execute(req).await?)
    }
}

impl super::Service<Error> for Discord {
    fn schema() -> Vec<&'static str> {
        vec!["discord"]
    }

    /// This is used by the announce method in [crate].
    /// Allowed url's are:
    /// * discord://WEBHOOK_ID/WEBHOOK_TOKEN
    /// Note: The url copied when creating a new webhook contains the id and the token
    #[doc(hidden)]
    fn build_request(
        client: &reqwest::Client,
        url: &reqwest::Url,
        msg: &CrateMessage,
    ) -> Result<reqwest::Request, Error> {
        let info = Self::from_url(url)?;
        let url = info.build_url()?;
        let msg = Message::from_message(msg);

        //build request
        let builder = client.request(reqwest::Method::POST, url);
        let req = builder
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&msg)
            .build()
            .unwrap();
        log::trace!("{:?}", req);

        Ok(req)
    }
}