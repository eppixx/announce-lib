//! A Module for sending files to a channel in RocketChat

/// Upload holds all the possible information that can be send with a file
#[derive(Debug)]
pub struct Upload<'a> {
    /// A Message to be send with a file
    pub message: Option<&'a str>,
    /// A description what the file contains
    pub description: Option<&'a str>,
    /// The path to the file to send
    pub file_path: &'a str,
}

impl<'a> Upload<'a> {
    /// creates a new Upload struct
    pub fn new(file_path: &'a str) -> Upload {
        Self {
            description: None,
            message: None,
            file_path,
        }
    }

    /// builds the multipart form for streaming a file
    pub(super) async fn build_form(&self) -> Result<reqwest::multipart::Form, crate::Error> {
        //open file to body stream
        let file = tokio::fs::File::open(self.file_path).await?;
        let stream = tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
        let file_body = reqwest::Body::wrap_stream(stream);

        //make form part of file
        let file_path = String::from(self.file_path);
        let part = reqwest::multipart::Part::stream(file_body).file_name(file_path);
        let mime = mime_guess::from_path(self.file_path);
        let file_part = match mime.first() {
            None => part,
            Some(mime) => part.mime_str(mime.essence_str())?,
        };

        let mut form = reqwest::multipart::Form::new();
        if let Some(s) = self.message {
            form = form.text("msg", String::from(s));
        };
        if let Some(s) = self.description {
            form = form.text("description", String::from(s));
        };
        Ok(form.part("file", file_part))
    }
}
