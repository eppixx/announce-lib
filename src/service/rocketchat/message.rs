use serde::Serialize;

use crate::message::Message as CrateMessage;

/// Allows for "tables" or "columns" to be displayed on messages.
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Fields<'a> {
    #[serde(default)]
    pub short: bool,
    pub title: &'a str,
    pub value: &'a str,
}

/// An attachment to a message
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct Attachment<'a> {
    /// color on the left side; accepts all background-css accepts
    pub color: Option<&'a str>,
    pub text: Option<&'a str>,
    /// timestamp
    pub ts: Option<chrono::DateTime<chrono::Utc>>,
    pub thumb_url: Option<&'a str>,
    pub message_link: Option<&'a str>,
    pub collapsed: bool,
    pub author_name: Option<&'a str>,
    pub author_link: Option<&'a str>,
    pub author_icon: Option<&'a str>,
    pub title: Option<&'a str>,
    pub title_link: Option<&'a str>,
    pub title_link_download: bool,
    pub image_url: Option<&'a str>,
    pub video_url: Option<&'a str>,
    pub audio_url: Option<&'a str>,
    pub fields: Vec<Fields<'a>>,
}

impl<'a> Attachment<'a> {
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

/// Main body of a message
#[derive(Serialize, Debug)]
pub struct Message<'a> {
    channel: String,
    pub text: Option<&'a str>,
    pub alias: Option<&'a str>,
    pub emoji: Option<&'a str>,
    pub avatar: Option<&'a str>,
    pub attachments: Vec<Attachment<'a>>,
}

impl<'a> Message<'a> {
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

    pub(super) fn populate(&mut self, msg: &'a CrateMessage) {
        match msg {
            CrateMessage::Text(s) => self.text = Some(s),
            CrateMessage::Link(url) => {
                let mut attachment = Attachment::default();
                attachment.link(url);
                self.attachments.push(attachment);
            }
            CrateMessage::LinkWithText(text, url) => {
                let mut attachment = Attachment::<'_> {
                    text: Some(text),
                    ..Default::default()
                };
                attachment.link(url);
                self.attachments.push(attachment);
            }
        }
    }
}
