#![warn(missing_docs)]
#![doc = include_str!("../Readme.md")]

pub mod error;
pub mod message;
pub mod service;

pub use error::Error;
pub use message::Hint;
pub use message::Message;

// TODO merge with ServiceResult when async in traits is allowed
/// Result that contains all results from every possible service
#[derive(Debug)]
pub enum ReturnType {
    /// A Reqwest result
    Reqwest(reqwest::Response),
    /// A Dbus result
    Dbus(u32),
}

/// A struct to contain information to call services
pub struct Announce {
    /// client for internet Api's
    pub client: reqwest::Client,

    #[cfg(feature = "dbus")]
    /// connection for dbus
    dbus_con: zbus::Connection,
}

impl Announce {
    /// Creates a Announce object for announcing
    pub async fn new() -> Result<Self, Error> {
        //build client
        let mut agent = reqwest::header::HeaderMap::new();
        agent.insert(
            reqwest::header::USER_AGENT,
            format!("announce/{}", env!("CARGO_PKG_VERSION"))
                .parse()
                .unwrap(),
        );
        let client = reqwest::ClientBuilder::new()
            .use_rustls_tls()
            .default_headers(agent)
            .build()?;

        #[cfg(feature = "dbus")]
        let dbus_con = zbus::Connection::session().await?;

        Ok(Self {
            client,
            #[cfg(feature = "dbus")]
            dbus_con,
        })
    }

    /// Sends the same messages to multiple services.
    ///
    /// If an error is encountered while sending a message the following urls that follow
    /// will be canceled.
    pub async fn announce(
        &self,
        urls: Vec<reqwest::Url>,
        msg: &Message<'_>,
    ) -> Result<Vec<ReturnType>, Error> {
        //build requests for each given target
        let mut results = vec![];
        for url in urls {
            results.push(service::decide_service(self, &url, msg).await?);
        }

        Ok(results)
    }

    /// Sends the same message to multiple services but ignores errors.
    ///
    /// If a services produces an error it will be logged and ignored.
    pub async fn announce_ignore_errors(&self, urls: Vec<reqwest::Url>, msg: &Message<'_>) {
        for url in urls {
            match service::decide_service(self, &url, msg).await {
                Ok(_) => {}
                Err(e) => log::warn!("encountered an error in {}: {}", url, e),
            }
        }
    }
}
