//! Dbus is a interprocess communication service often found on Linux systems.
//!
//! This module uses the notification service which is provided by
//! most Linux desktop environments.

/// Message is to be used with this module
pub mod message;

use crate::service::Service;
pub use message::Message;

//TODO remove this when following issue is solved
// https://gitlab.freedesktop.org/dbus/zbus/-/issues/87
#[zbus::dbus_proxy(assume_defaults = true, interface = "org.freedesktop.Notifications")]
/// Freedesktop Notification [Reference](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html)
trait Notifications {
    // this function is not defined by us, but the dbus specification
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: std::collections::HashMap<&str, zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

/// A implementation of messaging to a Dbus notification service
pub struct Dbus {
    app_name: Option<String>,
    app_icon: Option<String>,
    expire_timeout: Option<i32>,
}

impl Dbus {
    /// Creates a Dbus struct from a url
    fn from_url(url: &reqwest::Url) -> Result<Self, crate::Error> {
        if !Self::match_scheme(url) {
            return Err(crate::Error::WrongScheme(format!(
                "found \"{}\"",
                url.scheme()
            )));
        }

        let app_name = match url.username() {
            name if name.is_empty() => None,
            name => Some(String::from(name)),
        };

        //extract from url
        Ok(Self {
            app_name,
            app_icon: url.password().map(String::from),
            expire_timeout: url.port().map(i32::from),
        })
    }

    /// Accepts the following url formattings:
    ///* dbus://{APP_NAME@}{ICON_NAME}{:TIMEOUT}
    /// ```no_run
    /// use announce::service::dbus;
    ///
    /// let mut msg = dbus::Message::default();
    /// msg.summary = "Header of message";
    /// msg.body = "Main body of message";
    /// // modify msg to your linking
    ///
    /// dbus::Dbus::announce(&msg);
    /// ```
    pub async fn announce(msg: &Message<'_>) -> Result<u32, crate::Error> {
        let connection = zbus::Connection::session().await?;
        let proxy = NotificationsProxy::new(&connection).await?;

        let reply = proxy
            .notify(
                msg.app_name,
                msg.replaces_id,
                msg.app_icon,
                msg.summary,
                msg.body,
                msg.actions,
                msg.hints.clone(),
                msg.expire_timeout,
            )
            .await?;
        log::trace!("dbus returned: {:?}", reply);

        Ok(reply)
    }
}

#[async_trait::async_trait]
impl super::Service for Dbus {
    fn schema() -> Vec<&'static str> {
        vec!["dbus"]
    }

    /// This is used by the announce method in [crate].
    /// Allowed url's are:
    /// * dbus://{APP_NAME@}{ICON_NAME}{:TIMEOUT}
    #[doc(hidden)]
    async fn build_request(
        announce: &crate::Announce,
        url: &reqwest::Url,
        msg: &crate::Message,
    ) -> Result<super::ServiceResult, crate::Error> {
        let info = Self::from_url(url)?;
        let proxy = NotificationsProxy::new(&announce.dbus_con).await?;
        let mut message = Message::from_crate_message(msg);

        let app_name = info
            .app_name
            .unwrap_or_else(|| String::from(message.app_name));
        message.app_name = &app_name;
        let app_icon = info
            .app_icon
            .unwrap_or_else(|| String::from(message.app_icon));
        message.app_icon = &app_icon;
        message.expire_timeout = info.expire_timeout.unwrap_or(message.expire_timeout);

        let reply = proxy
            .notify(
                message.app_name,
                message.replaces_id,
                message.app_icon,
                message.summary,
                message.body,
                message.actions,
                message.hints,
                message.expire_timeout,
            )
            .await?;
        log::trace!("{:?}", reply);

        Ok(super::ServiceResult::Dbus(reply))
    }
}

#[cfg(test)]
mod tests {
    use crate::service::Service;

    #[tokio::test]
    async fn test_dbus_announce() {
        let mut msg = super::Message::default();
        msg.app_name = "Announce";
        msg.summary = "summary";
        msg.body = "body";

        let result = super::Dbus::announce(&msg).await.unwrap();
        dbg!(result);
    }

    #[tokio::test]
    async fn test_dbus_msg() {
        let announce = crate::Announce::new().unwrap();
        let msg = crate::Message::new("ein test");
        let url = "dbus://Announce@dialog_information:1";
        let url = reqwest::Url::parse(&url).expect("faulty url");

        let response = super::Dbus::build_request(&announce, &url, &msg);
        let _ = response.unwrap();
    }
}
