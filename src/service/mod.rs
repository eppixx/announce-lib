//! Contains the services which are supportet by anounce.

use crate::message::Message;

#[cfg(feature = "dbus")]
pub mod dbus;
#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "rocketchat")]
pub mod rocketchat;

// /// Type for catching results of services
// #[derive(Debug)]
// pub enum ServiceResult {
//     /// A Request that still needs to be executed
//     Reqwest(reqwest::Request),
//     /// A Dbus result
//     Dbus(u32),
// }

/// A trait implemented for all services
#[async_trait::async_trait]
pub trait Service {
    /// Returns a Vec of supported schemas
    fn schema() -> Vec<&'static str>;

    // shouldn't be used by the user
    // either use the crate::announce(..) method
    // or the announce method of a specific service
    #[doc(hidden)]
    async fn notify(
        announce: &crate::Announce,
        target: &reqwest::Url,
        msg: &Message,
    ) -> Result<crate::ReturnType, crate::Error>;

    /// Returns true if a given url matches with a schema of a given service
    fn match_scheme(url: &reqwest::Url) -> bool {
        Self::schema().iter().any(|s| &url.scheme() == s)
    }
}

/// Tests url with all services and returns a request if it does
pub async fn decide_service(
    announce: &crate::Announce,
    url: &reqwest::Url,
    msg: &Message<'_>,
) -> Result<crate::ReturnType, crate::Error> {
    //cascade of services
    #[cfg(feature = "rocketchat")]
    if rocketchat::RocketChat::match_scheme(url) {
        return rocketchat::RocketChat::notify(announce, url, msg).await;
    }
    #[cfg(feature = "dbus")]
    if dbus::Dbus::match_scheme(url) {
        return dbus::Dbus::notify(announce, url, msg).await;
    }
    #[cfg(feature = "discord")]
    if discord::Discord::match_scheme(url) {
        return discord::Discord::notify(announce, url, msg).await;
    }

    Err(crate::Error::NoMatchingSchema)
}
