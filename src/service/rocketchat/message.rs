use serde::Serialize;

use crate::message::Message as CrateMessage;

/// Allows for "tables" or "columns" to be displayed on messages.
///
/// Rocket.Chat [Reference](https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage#attachment-field-objects)
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Fields<'a> {
    /// Whether this field should be a short field
    #[serde(default)]
    pub short: bool,

    /// The title of the field
    pub title: &'a str,

    /// The value of this field, displayed underneath the title value
    pub value: &'a str,
}

/// An attachment to a [Message]
///
/// Rocket.Chat [Reference](https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage#attachments-detail)
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct Attachment<'a> {
    /// The color you want the order on the left side to be.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<&'a str>,

    /// The Text to display for this attachment, differs from the message's text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,

    /// Displays the time next to the text portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<chrono::DateTime<chrono::Utc>>,

    /// An image that displays to the left of the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<&'a str>,

    /// Only applicable if the ts is provided, as it makes the time clickable to this link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_link: Option<&'a str>,

    /// Causes the image, audio, and video sections to be hiding when collapsed is true.
    pub collapsed: bool,

    /// Name of the author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<&'a str>,

    /// Providing this makes the author name clickable and points to this link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_link: Option<&'a str>,

    /// Displays a tiny icon to the left of the Author's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_icon: Option<&'a str>,

    /// Title to display for this attachment, displays under the author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,

    /// Providing this makes the title clickable, pointing to this link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_link: Option<&'a str>,

    /// When this is true, a download icon appears and clicking this saves the link to file.
    pub title_link_download: bool,

    /// The image to display, will be "big" and easy to see.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<&'a str>,

    /// Video file to play, only supports what [html video](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video) does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<&'a str>,

    /// Audio file to play, only supports what [html audio](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio) does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_url: Option<&'a str>,

    /// An array of [Attachment Field Objects](Fields).
    pub fields: Vec<Fields<'a>>,
}

impl<'a> Attachment<'a> {
    /// decides the type of url by the file ending
    pub fn link(&mut self, url: &'a str) {
        let image_endings = vec!["png", "tiff", "jpg", "jpeg", "webp", "gif", "bmp"];
        let video_endings = vec![
            "mp4", "mkv", "webm", "ogv", "avi", "wmv", "mpg", "mpeg", "flv",
        ];
        let audio_endings = vec![
            "mp3", "opus", "oga", "ogg", "wav", "aac", "wma", "flac", "ape", "webm",
        ];

        if image_endings.iter().any(|end| url.ends_with(end)) {
            self.image_url = Some(url);
        } else if video_endings.iter().any(|end| url.ends_with(end)) {
            self.video_url = Some(url);
        } else if audio_endings.iter().any(|end| url.ends_with(end)) {
            self.audio_url = Some(url);
        } else {
            self.message_link = Some(url);
        }
    }
}

/// Main body of a message to be used by this module
///
/// Rocket.Chat [Reference](https://developer.rocket.chat/reference/api/rest-api/endpoints/core-endpoints/chat-endpoints/postmessage#payload)
#[derive(Serialize, Debug, Clone)]
pub struct Message<'a> {
    /// The channel name of where the message is to be sent.
    channel: String,

    /// The text of the message to send, is optional because of attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,

    /// This will cause the message's name to appear as the given alias, but your username will still display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<&'a str>,

    /// If provided, this will make the avatar on this message be an [emoji](https://emoji.codes/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<&'a str>,

    /// If provided, this will make the avatar use the provided image url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<&'a str>,

    /// See [Attachment]
    pub attachments: Vec<Attachment<'a>>,
}

impl<'a> Message<'a> {
    /// creates a message for a channel
    pub fn new(channel: &str) -> Self {
        let channel = if channel.starts_with('#') {
            format!("#{}", channel)
        } else {
            String::from(channel)
        };
        Self {
            channel,
            text: None,
            alias: None,
            emoji: None,
            avatar: None,
            attachments: vec![],
        }
    }

    /// creates a copy of the message, but for another channel
    pub fn other_channel(other: &Self, channel: &str) -> Self {
        let channel = if channel.starts_with('#') {
            format!("#{}", channel)
        } else {
            String::from(channel)
        };
        let mut clone = other.clone();
        clone.channel = channel;
        clone
    }

    /// converts a crate::Message to Message of this module
    pub(super) fn populate_from_crate_message(&mut self, msg: &'a CrateMessage) {
        self.text = msg.text;
        for hint in &msg.hints {
            match hint {
                crate::message::Hint::Link(link) => {
                    let mut attachment = Attachment::default();
                    attachment.link(link);
                    self.attachments.push(attachment);
                }
            }
        }
    }
}
