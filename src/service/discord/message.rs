//! Message is to be used with this module

use serde::Serialize;

use crate::message::Message as CrateMessage;

/// An Attachment that is embedded in a message
///
/// Embed [Reference](https://discord.com/developers/docs/resources/channel#embed-object)
#[derive(Serialize, Debug, Default)]
pub struct Embed<'a> {
    /// title of emebed
    pub title: Option<&'a str>,

    // /// type of [Embed]
    // #[serde(rename = "type")]
    // pub typ: Option<&'a str>,
    /// desciption of embed
    pub description: Option<&'a str>,

    /// url of embed
    pub url: Option<&'a str>,

    // /// timestamp of embed content
    // pub timestamp: Option<&'a str>,
    /// color code of embed
    pub color: Option<i32>,
    // /// footer information
    // footer,

    // /// image information
    // image,

    // /// thumbnail information
    // thumbnail,

    // /// video information
    // video,

    // /// provider information
    // provider,

    // /// fields information
    // fields,
}

/// Represents a message to Discord
///
/// Discord [Reference](https://discord.com/developers/docs/resources/webhook#execute-webhook)
#[derive(Serialize, Debug, Default)]
pub struct Message<'a> {
    /// the message contents (up to 2000 characters)
    pub content: Option<&'a str>,

    /// override the default username of the webhook
    pub username: Option<&'a str>,

    /// override the default avatar of the webhook
    pub avatar_url: Option<&'a str>,

    /// true if this is a TTS message
    pub tts: bool,

    /// up to 10 Embeds
    pub embeds: Vec<Embed<'a>>,

    // /// allowed mentions for the message
    // allowed_mentions: Mention,

    // /// the components to include with the message
    // components,

    // /// the contents of the file being sent
    // files: Vec<File>,

    // /// JSON encoded body of non-file params
    // payload_json: Option<&'a str>,

    // /// attachment objects with filename and description
    // attachments: Vec<Attachment>,
    /// message flags combined as a bitfield (only SUPPRESS_EMBEDS can be set)
    pub flags: Option<i32>,

    /// name of thread to create (requires the webhook channel to be a forum channel)
    pub thread_name: Option<&'a str>,
}

impl<'a> Message<'a> {
    /// creates a Message from a [crate::Message]
    pub fn from_message(msg: &'a CrateMessage) -> Message<'a> {
        let mut result = Message::default();
        match msg {
            CrateMessage::Text(s) => result.content = Some(s),
            CrateMessage::Link(url) => {
                let embed = Embed::<'_> {
                    url: Some(url),
                    ..Default::default()
                };
                result.embeds.push(embed);
            }
            CrateMessage::LinkWithText(text, url) => {
                let embed = Embed::<'_> {
                    title: Some(text),
                    url: Some(url),
                    ..Default::default()
                };
                result.embeds.push(embed);
            }
        }

        result
    }
}
