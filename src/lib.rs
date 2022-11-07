mod service;

#[derive(Clone)]
pub enum Message {
    Text(String),
}

pub async fn announce(targets: Vec<&str>, msg: Message) -> Result<(), service::ServiceError> {
    //build requests for each given target
    let mut requests = vec![];
    for r in targets {
        requests.push(service::decide_service(r, &msg)?);
    }

    //build client
    let connector = hyper_tls::HttpsConnector::new();
    let clients = hyper::client::Client::builder().build::<_, hyper::Body>(connector);
    let client = hyper::client::Client::new();

    //send each request
    for (req, https) in requests {
        match https {
            true => clients.request(req).await?,
            false => client.request(req).await?,
        };
    }

    Ok(())
}
