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
    pub hints: std::collections::HashMap<&'a str, zvariant::Value<'a>>,

    ///  The timeout time in milliseconds since the display of the notification at
    /// which the notification should automatically close.
    ///
    /// If -1, the notification's expiration time is dependent on the notification
    /// server's settings, and may vary for the type of notification.
    ///
    /// If 0, the notification never expires.
    pub expire_timeout: i32,
}

impl<'a> Message<'a> {
    /// add a standard hint; more typesafe than inserting manually to member hints
    pub fn add_standard_hint(&mut self, hint: &'a StandardHint) {
        use zvariant::Value;

        let (hint, value) = match hint {
            StandardHint::ActionIcons(status) => ("action-icons", Value::new(status)),
            StandardHint::Category(cat) => ("category", Value::new(cat)),
            StandardHint::DesktopEntry(entry) => ("desktop-entry", Value::new(entry)),
            StandardHint::ImageData(data) => ("image-data", data.clone()),
            StandardHint::ImagePath(path) => ("image-path", Value::new(path)),
            StandardHint::Resident(status) => ("resident", Value::new(status)),
            StandardHint::SoundFile(file) => ("sound-file", Value::new(file)),
            StandardHint::SoundName(name) => ("sound-name", Value::new(name)),
            StandardHint::SupressSound(status) => ("supress-sound", Value::new(status)),
            StandardHint::Transient(status) => ("transient", Value::new(status)),
            StandardHint::X(value) => ("x", Value::new(value)),
            StandardHint::Y(value) => ("y", Value::new(value)),
            StandardHint::Urgency(byte) => ("urgency", Value::new(byte)),
        };

        self.hints.insert(hint, value);
    }
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

/// Hints that are defined by the specification that may be supported by the notification
/// server
pub enum StandardHint<'a> {
    /// When set, a server that has the "action-icons" capability will attempt to
    /// interpret any action identifier as a named icon. The localized display name
    /// will be used to annotate the icon for accessibility purposes. The icon name
    /// should be compliant with the Freedesktop.org Icon Naming Specification.
    ActionIcons(bool),

    /// The type of notification this is.
    Category(String),

    /// This specifies the name of the desktop filename representing the calling
    /// program. This should be the same as the prefix used for the application's
    /// .desktop file. An example would be "rhythmbox" from "rhythmbox.desktop".
    /// This can be used by the daemon to retrieve the correct icon for the application,
    /// for logging purposes, etc.
    DesktopEntry(String),

    /// This is a raw data image format which describes the width, height, rowstride,
    /// has alpha, bits per sample, channels and image data respectively.
    ImageData(zvariant::Value<'a>),

    /// Alternative way to define the notification image. See [Icons and Images](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html#icons-and-images).
    ImagePath(String),

    /// When set the server will not automatically remove the notification when an
    /// action has been invoked. The notification will remain resident in the server
    /// until it is explicitly removed by the user or by the sender. This hint is
    /// likely only useful when the server has the "persistence" capability.
    Resident(bool),

    /// The path to a sound file to play when the notification pops up.
    SoundFile(String),

    /// A themeable named sound from the freedesktop.org sound naming specification
    /// to play when the notification pops up. Similar to icon-name, only for sounds.
    /// An example would be "message-new-instant".
    SoundName(String),

    /// Causes the server to suppress playing any sounds, if it has that ability.
    /// This is usually set when the client itself is going to play its own sound.
    SupressSound(bool),

    /// When set the server will treat the notification as transient and by-pass
    /// the server's persistence capability, if it should exist.
    Transient(bool),

    /// Specifies the X location on the screen that the notification should point to.
    /// The "y" hint must also be specified.
    X(i32),

    /// Specifies the Y location on the screen that the notification should point to.
    /// The "x" hint must also be specified.
    Y(i32),

    /// The urgency level.
    Urgency(u8),
}
