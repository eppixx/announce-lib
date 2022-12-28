#![warn(missing_docs)]
#![doc = include_str!("../Readme.md")]

pub mod error;
pub mod message;
pub mod service;

pub use error::Error;
pub use message::Message;

// TODO merge with ServiceResult when async in traits is allowed
/// Result that contains all results from every possible service
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
    pub dbus_con: zbus::blocking::Connection,
    //TODO replace with the following when async in traits is allowed
    // dbus_con: zbus::Connection;
}

impl Announce {
    /// Creates a Announce object for announcing
    pub fn new() -> Result<Self, Error> {
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
        let dbus_con = zbus::blocking::Connection::session()?;

        Ok(Self {
            client,
            #[cfg(feature = "dbus")]
            dbus_con,
        })
    }

    /// Sends the same messages to multiple services.
    ///
    /// If a Error is encountered while sending a message the following urls that follow
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

        ReturnType::convert(self, results).await
    }
}

impl ReturnType {
    /// Converts a [service::Result] to [Self]. Also sends the requests and changes returns
    /// its results
    async fn convert(
        announce: &crate::Announce,
        results: Vec<service::ServiceResult>,
    ) -> Result<Vec<Self>, Error> {
        let mut collection = vec![];
        for result in results {
            use service::ServiceResult;
            collection.push(match result {
                ServiceResult::Reqwest(req) => {
                    ReturnType::Reqwest(announce.client.execute(req).await?)
                }
                ServiceResult::Dbus(result) => ReturnType::Dbus(result),
            })
        }

        Ok(collection)
    }
}
