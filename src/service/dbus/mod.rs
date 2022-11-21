use thiserror::Error;

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

    /// A zbus error
    #[error("zbus error: {0}")]
    Zbus(#[from] zbus::Error),

    /// Using the wrong schema with service
    #[error("Wrong scheme for service: {0}")]
    WrongScheme(String),
}

/// Freedesktop Notification [Reference](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html)
#[zbus::dbus_proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: std::collections::HashMap<&str, &zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

pub struct Dbus {}

impl Dbus {
    pub async fn announce(/* msg: &Message<'_> */) -> Result<(), Error> {
        let connection = zbus::Connection::session().await?;
        let proxy = NotificationsProxy::new(&connection).await?;

        let reply = proxy
            .notify(
                "Announce",
                0,
                "insert-image",
                "test",
                "nice text",
                &[],
                std::collections::HashMap::new(),
                0,
            )
            .await?;
        println!("{reply}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_dbus_announce() {
        let result = super::Dbus::announce().await.unwrap();
        // dbg!(result);
    }
}
