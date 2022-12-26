//! A Subset of Message types that are supported by all services.

/// A Subset of Message types that are supported by all services.
/// Message should be used in conjunction with multiple services. It's a feature subset of
/// messages to the messages the supported services provide.
/// That means the struct Message from the module of a service provides more customization for that
/// service.
#[derive(Debug)]
pub enum Message<'a> {
    /// simple text
    Text(&'a str),
    /// a simple link
    Link(&'a str),
    /// link with a text
    LinkWithText(&'a str, &'a str),
    // /// a file with a possible description
    // FileWithDescription {
    //     file: &'a str,
    //     description: Option<&'a str>,
    // },
}
