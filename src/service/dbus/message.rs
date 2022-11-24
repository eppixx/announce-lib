/// Struct that defines a message which can be send via Dbus.
///
/// Notificaiton [Reference](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html)
pub struct Message<'a> {
    /// This is the optional name of the application sending the notification.
    /// This should be the application's formal name, rather than some sort of ID.
    /// An example would be "FredApp E-Mail Client," rather than "fredapp-email-client."
    pub app_name: &'a str,

    /// An optional ID of an existing notification that this notification is intended to replace.
    pub replaces_id: u32,

    /// The notification icon.
    pub app_icon: &'a str,

    /// This is a single line overview of the notification.
    /// For instance, "You have mail" or "A friend has come online".
    /// It should generally not be longer than 40 characters, though this is not a
    /// requirement, and server implementations should word wrap if necessary.
    /// The summary must be encoded using UTF-8.
    pub summary: &'a str,

    /// This is a multi-line body of text.
    /// Each line is a paragraph, server implementations are free to word wrap
    /// them as they see fit.
    ///
    /// The body may contain simple markup as specified in [Markup](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html#markup).
    /// It must be encoded using UTF-8.
    ///
    /// If the body is omitted, just the summary is displayed
    pub body: &'a str,

    /// The actions send a request message back to the notification client when invoked.
    /// This functionality may not be implemented by the notification server,
    /// conforming clients should check if it is available before using it (see
    /// the GetCapabilities message in [Protocol](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html#protocol)).
    /// An implementation is free to ignore
    /// any requested by the client. As an example one possible rendering of actions
    /// would be as buttons in the notification popup.
    ///
    /// Actions are sent over as a list of pairs.
    /// Each even element in the list (starting at index 0) represents the identifier
    /// for the action. Each odd element in the list is the localized string that
    /// will be displayed to the user.
    ///
    /// The default action (usually invoked my clicking the notification) should
    /// have a key named "default".
    /// The name can be anything, though implementations are free not to display it.
    pub actions: &'a [&'a str],

    /// Hints are a way to provide extra data to a notification server that the
    /// server may be able to make use of.
    ///
    /// See Hints for a list of available hints.
    pub hints: std::collections::HashMap<&'a str, &'a zvariant::Value<'a>>,

    ///  The timeout time in milliseconds since the display of the notification at
    /// which the notification should automatically close.
    ///
    /// If -1, the notification's expiration time is dependent on the notification
    /// server's settings, and may vary for the type of notification.
    ///
    /// If 0, the notification never expires.
    pub expire_timeout: i32,
}

impl<'a> Default for Message<'a> {
    fn default() -> Self {
        Self {
            app_name: "Announce",
            replaces_id: 0,
            app_icon: "dialog-information",
            summary: "Announce",
            body: "",
            actions: &[],
            hints: std::collections::HashMap::new(),
            expire_timeout: 0,
        }
    }
}
