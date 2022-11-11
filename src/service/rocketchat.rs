use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    channel: String,
    text: String,
}

impl Message {
    fn new(channel: &str) -> Self {
        Self {
            channel: format!("#{}", channel),
            text: String::new(),
        }
    }

    fn populate(&mut self, msg: &crate::Message) {
        use crate::Message;
        match msg {
            Message::Text(s) => self.text = s.clone(),
        }
    }
}

//reference: https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage
pub struct RocketChat {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("uri parse error: {0}")]
    Parse(#[from] url::ParseError),
    #[error("url error: {0}")]
    Url(String),
}

impl super::Service<Error> for RocketChat {
    fn schema() -> Vec<&'static str> {
        vec!["rocketchat", "rocketchats"]
    }

    /// allowed uri's are:
    /// rocketchat://USER:TOKEN@HOSTNAME/CHANNEL
    /// rocketchats://USER:TOKEN@HOSTNAME/CHANNEL
    fn request(
        client: &reqwest::Client,
        target: &str,
        msg: &crate::Message,
    ) -> Result<reqwest::Request, Error> {
        //extract information from target
        let url = url::Url::parse(target).unwrap();
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

        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    }
}
