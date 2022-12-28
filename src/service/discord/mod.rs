//! Discord is a VoIP and instant messaging social platform.
//!
//! * Homepage: <https://discord.com/>
//! * Reference API: <https://discord.com/developers/docs/intro>

pub mod message;
mod tests;

use crate::message::Message as CrateMessage;
use crate::service::Service;

pub use message::Message;

/// A implementation of messageing to a Discord channel
pub struct Discord {
    webhook_id: String,
    webhook_token: String,
}

/// A implementation of messaging to Discord
impl Discord {
    /// Creates a Discord struct from a url
    fn from_url(url: &reqwest::Url) -> Result<Self, crate::Error> {
        if !Self::match_scheme(url) {
            return Err(crate::Error::WrongScheme(format!(
                "found \"{}\"",
                url.scheme()
            )));
        }

        //extract from url
        let id = url
            .host_str()
            .ok_or_else(|| crate::Error::MissingField(String::from("webhook_id")))?;
        let token = url
            .path_segments()
            .map(|mut path| String::from(path.next().unwrap()))
            .ok_or_else(|| crate::Error::MissingField(String::from("webhook_token")))?;

        Ok(Self {
            webhook_id: String::from(id),
            webhook_token: token,
        })
    }

    /// Returns the url that the message will be send to
    fn build_url(&self) -> Result<reqwest::Url, crate::Error> {
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
    ) -> Result<reqwest::Response, crate::Error> {
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

#[async_trait::async_trait]
impl super::Service for Discord {
    fn schema() -> Vec<&'static str> {
        vec!["discord"]
    }

    /// This is used by the announce method in [crate].
    /// Allowed url's are:
    /// * discord://WEBHOOK_ID/WEBHOOK_TOKEN
    /// Note: The url copied when creating a new webhook contains the id and the token
    #[doc(hidden)]
    async fn build_request(
        announce: &crate::Announce,
        url: &reqwest::Url,
        msg: &CrateMessage,
    ) -> Result<super::ServiceResult, crate::Error> {
        let info = Self::from_url(url)?;
        let url = info.build_url()?;
        let msg = Message::from_crate_message(msg);

        //build request
        let builder = announce.client.request(reqwest::Method::POST, url);
        let req = builder
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&msg)
            .build()
            .unwrap();
        log::trace!("{:?}", req);

        Ok(super::ServiceResult::Reqwest(req))
    }
}
