//! Message is to be used with this module

use serde::Serialize;

use crate::message::Message as CrateMessage;

/// Represents a message to Discord
///
/// Discord [Reference](https://discord.com/developers/docs/resources/webhook#execute-webhook)
#[derive(Serialize, Debug, Default)]
pub struct Message<'a> {
    /// the message contents (up to 2000 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<&'a str>,

    /// override the default username of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<&'a str>,

    /// override the default avatar of the webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<&'a str>,

    /// true if this is a TTS message
    pub tts: bool,

    /// up to 10 Embeds
    pub embeds: Vec<Embed<'a>>,

    // /// allowed mentions for the message
    // TODO implement
    // allowed_mentions: Mention,

    // /// the components to include with the message
    // TODO implement
    // components,

    // /// the contents of the file being sent
    // TODO implement
    // files: Vec<File>,

    // /// JSON encoded body of non-file params
    // TODO implement
    // payload_json: Option<&'a str>,

    // /// attachment objects with filename and description
    // TODO implement
    // attachments: Vec<Attachment>,
    /// message flags combined as a bitfield (only SUPPRESS_EMBEDS can be set)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,

    /// name of thread to create (requires the webhook channel to be a forum channel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_name: Option<&'a str>,
}

impl<'a> Message<'a> {
    /// creates a Message from a [crate::Message]
    pub fn from_crate_message(msg: &'a CrateMessage) -> Message<'a> {
        let mut result = Message {
            content: msg.text,
            ..Default::default()
        };
        for hint in &msg.hints {
            match hint {
                crate::message::Hint::Link(link) => {
                    let embed = Embed::<'_> {
                        url: Some(link),
                        ..Default::default()
                    };
                    result.embeds.push(embed);
                }
            }
        }

        result
    }
}

/// An Attachment that is embedded in a message
///
/// Embed [Reference](https://discord.com/developers/docs/resources/channel#embed-object)
#[derive(Serialize, Debug, Default)]
pub struct Embed<'a> {
    /// title of emebed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,

    /// type of [Embed]
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub typ: Option<Typ>,

    /// desciption of embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// url of embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,

    //TODO add a some sort of ISO8601 compliant type
    // /// timestamp of embed content
    // pub timestamp: Option<ISO8601>,
    /// color code of embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,

    /// footer information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<Footer<'a>>,

    /// image information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image<'a>>,

    /// thumbnail information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail<'a>>,

    /// video information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video<'a>>,

    /// provider information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Provider<'a>>,

    /// author information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author<'a>>,

    /// fields information
    pub fields: Vec<Field<'a>>,
}

/// Embedded Type
///
/// Embed types are "loosely defined" and, for the most part, are not used by our clients for rendering. Embed attributes power what is rendered. Embed types should be considered deprecated and might be removed in a future API version.
///
///Type [Reference](https://discord.com/developers/docs/resources/channel#embed-object-embed-types)
#[derive(Serialize, Debug)]
pub enum Typ {
    /// generic embed rendered from embed attributes
    Rich,

    /// image embed
    Image,

    /// video embed
    Video,

    /// animated gif image embed rendered as a video embed
    Gifv,

    /// artile embed
    Article,

    /// link embed
    Link,
}

/// Embedded Footer
///
/// Footer [Reference](https://discord.com/developers/docs/resources/channel#embed-object-embed-footer-structure)
#[derive(Serialize, Debug, Default)]
pub struct Footer<'a> {
    /// footer text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,

    /// url of footer icon (only supports http(s) and attachments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<&'a str>,

    /// a proxied url of footer icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<&'a str>,
}

/// Embedded Thumbnail
///
/// Thumbnail [Reference](https://discord.com/developers/docs/resources/channel#embed-object-embed-thumbnail-structure)
#[derive(Serialize, Debug, Default)]
pub struct Thumbnail<'a> {
    /// source url of the thumbnail (only supports http(s) and attachments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,

    /// a proxied url of the thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<&'a str>,

    /// height of thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,

    /// width of thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

/// Embedded Video
///
/// Video [Reference](https://discord.com/developers/docs/resources/channel#embed-object-embed-video-structure)
#[derive(Serialize, Debug, Default)]
pub struct Video<'a> {
    /// source url of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,

    /// a proxied url of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<&'a str>,

    /// height of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,

    /// width of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

/// Embedded Image
///
/// Image [Reference](https://discord.com/developers/docs/resources/channel#embed-object-embed-image-structure)
#[derive(Serialize, Debug, Default)]
pub struct Image<'a> {
    /// source url of image (only supports http(s) and attachments)
    pub url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]

    /// a proxied url of the image
    pub proxy: Option<&'a str>,

    /// height of the image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,

    /// width of the image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

/// Embedded Provider
///
/// Provider [Reference](https://discord.com/developers/docs/resources/channel#embed-object-embed-provider-structure)
#[derive(Serialize, Debug, Default)]
pub struct Provider<'a> {
    /// name of provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// url of provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
}

/// Embedded Author
///
/// Author [Reference](https://discord.com/developers/docs/resources/channel#embed-object-embed-author-structure)
#[derive(Serialize, Debug, Default)]
pub struct Author<'a> {
    /// name of the author
    pub name: &'a str,

    /// url of author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,

    /// url of author icon (only supports http(s) and attachments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<&'a str>,

    /// a proxied url of author icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<&'a str>,
}

/// Embedded Field
///
/// Field [Reference](https://discord.com/developers/docs/resources/channel#embed-object-embed-field-structure)
#[derive(Serialize, Debug, Default)]
pub struct Field<'a> {
    /// name of the field
    pub name: &'a str,

    /// value of the field
    pub value: &'a str,

    /// whether or not this field should display inline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}
