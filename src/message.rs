//! A Subset of Message types that are supported by most services.

/// A Subset of Message types that are supported by all services.
/// Message should be used in conjunction with multiple services. It's a feature subset of
/// messages to the messages the supported services provide.
/// That means the struct Message from the module of a service provides more customization for that
/// service.
#[derive(Debug, Default)]
pub struct Message<'a> {
    /// Text to send
    pub text: Option<&'a str>,
    /// Some more special infos that vary by service
    pub hints: Vec<Hint<'a>>,
    /// A path to a file which can be send
    pub file_path: Option<&'a str>,
}

impl<'a> Message<'a> {
    /// Constructs a simple Message for sending to services
    pub fn new(text: &'a str) -> Self {
        Self {
            text: Some(text),
            ..Default::default()
        }
    }
}

/// They modify a Message or contain a specify information for a service
/// (which other may ignore).
#[derive(Debug)]
pub enum Hint<'a> {
    /// A Link
    Link(&'a str),
    /// A Description
    Description(&'a str),
}
