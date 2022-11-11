mod service;

#[derive(Clone)]
pub enum Message {
    Text(String),
    // Link(Url),
}

pub async fn announce(targets: Vec<&str>, msg: Message) -> Result<(), service::ServiceError> {
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
        .build()
        .unwrap();

    //build requests for each given target
    let mut requests = vec![];
    for target in targets {
        requests.push(service::decide_service(&client, target, &msg)?);
    }

    //send each request
    for req in requests {
        client.execute(req).await?;
    }

    Ok(())
}
