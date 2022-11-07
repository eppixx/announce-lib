use hyper::{Body, Method};
use thiserror::Error;

pub struct RocketChat {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] http::Error),
    #[error("target does not match Regex")]
    RegexMatch,
}

impl super::Service<Error> for RocketChat {
    fn schema() -> Vec<&'static str> {
        vec!["rocketchat", "rocketchats"]
    }

    /// allowed uri's are:
    /// rocketchat://USER:TOKEN@HOSTNAME/CHANNEL
    /// rocketchats://USER:TOKEN@HOSTNAME/CHANNEL
    fn request(
        target: &str,
        msg: &crate::Message,
    ) -> Result<(http::Request<hyper::Body>, bool), Error> {
        let (https, user, token, host, port, channel) = Self::extract(target)?;

        let uri = match (https, port) {
            (true, Some(port)) => format!("https://{}:{}/api/v1/chat.PostMessage", host, port),
            (true, None) => format!("https://{}/api/v1/chat.PostMessage", host),
            (false, Some(port)) => format!("http://{}:{}/api/v1/chat.PostMessage", host, port),
            (false, None) => format!("http://{}/api/v1/chat.PostMessage", host),
        };

        use crate::Message;
        let msg = match msg {
            Message::Text(msg) => {
                format!("{{ \"channel\": \"#{}\", \"text\": \"{}\" }}", channel, msg)
            }
        };

        let req = Self::create_builder()
            .method(Method::POST)
            .uri(uri)
            .header("x-auth-token", token)
            .header("x-user-id", user)
            .header("content-type", "application/json")
            .body(Body::from(msg))?;

        Ok((req, https))
    }
}

impl RocketChat {
    fn extract(target: &str) -> Result<(bool, &str, &str, &str, Option<i32>, &str), Error> {
        let re = regex::Regex::new(
            r"rocketchat(?P<https>[s]?)://(?P<user>[a-zA-Z]*):(?P<token>[a-zA-Z]*)@(?P<hostname>[a-zA-Z]*)(:(?P<port>[0-9]*))?/(?P<channel>[a-zA-Z]*)",
        )
        .unwrap();
        let cap = re.captures(target).ok_or(Error::RegexMatch)?;

        Ok((
            cap.name("https").unwrap().as_str() == "s",
            cap.name("user").unwrap().as_str(),
            cap.name("token").unwrap().as_str(),
            cap.name("hostname").unwrap().as_str(),
            cap.name("port")
                .map(|port| port.as_str().parse::<i32>().unwrap()),
            cap.name("channel").unwrap().as_str(),
        ))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract() {
        assert_eq!(
            (false, "user", "token", "host", None, "channel"),
            super::RocketChat::extract("rocketchat://user:token@host/channel").unwrap()
        );
        assert_eq!(
            (true, "user", "token", "host", None, "channel"),
            super::RocketChat::extract("rocketchats://user:token@host/channel").unwrap()
        );
        assert_eq!(
            (true, "user", "token", "host", Some(800), "channel"),
            super::RocketChat::extract("rocketchats://user:token@host:800/channel").unwrap()
        );
    }
}
