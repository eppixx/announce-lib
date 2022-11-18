#![warn(missing_docs)]
#![doc = include_str!("../Readme.md")]

use message::Message;

pub mod message;
pub mod service;

/// Sends the same messages to multiple services.
///
/// If a Error is encountered while sending a message the following urls that follow
/// will be canceled.
pub async fn announce(
    urls: Vec<reqwest::Url>,
    msg: &Message<'_>,
) -> Result<(), service::ServiceError> {
    //build client
    let mut agent = reqwest::header::HeaderMap::new();
    agent.insert(
        reqwest::header::USER_AGENT,
        format!("announce/{}", env!("CARGO_PKG_VERSION"))
            .parse()
            .unwrap(),
    );
    let client = reqwest::ClientBuilder::new()
        .use_rustls_tls()
        .default_headers(agent)
        .build()?;

    //build requests for each given target
    let mut requests = vec![];
    for url in urls {
        requests.push(service::decide_service(&client, &url, msg)?);
    }

    //send each request
    for req in requests {
        client.execute(req).await?;
    }

    Ok(())
}
