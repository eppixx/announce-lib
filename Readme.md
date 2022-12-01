This crate allows to send messages (text, images, audio, video) to supported services.

# Supported services
Currently supported services are:
* Rocket.Chat
* D-Bus
* Discord

# How to use

There are two ways to use this crate.

## Announcing with a specific service
You can choose to use a specific service like Rocket.Chat, create a Message from its module and use the announce method of Rocket.Chat.

An example using Rocket.Chat
```rust,no_run
use announce::service::rocketchat;

let client = reqwest::Client::new();
let url = "rocketchat://user:token@host.com";
let url = url::Url::parse(url).unwrap();
let msg = rocketchat::Message::new("some_channel");
// modify msg to your liking

rocketchat::RocketChat::announce(&client, &url, &msg);
```


## Announcing the general way
It's also possible to announce through more than one service at the same time. To do this create a Message from the crate level and use the announce method from lib.rs.

```rust,no_run
let urls = vec![
  url::Url::parse("rocketchats://user:token@secure_host.com/channel").unwrap(),
  url::Url::parse("rocketchat://user2:token2@unsecure_host.com/channel2").unwrap(),
];
let ann = announce::Announce::new().unwrap();
let msg = announce::Message::Text("A sample Message to channel and channel2");
// or use another kind of enum Message

ann.announce(urls, &msg);
```

The drawback of this way is that you are less expressive this way as Message uses a subset of features of any specific service.

# Features

By default all services are included.
Every service is contained by a seperate feature, so it is possible to get only the services you need.
To find out the the name of the features visit the feature section in Cargo.toml and use it like this.
```toml ignore
## in Cargo.toml
[dependencies]
## ...
announce = { version = "vx.x.x", default-features = false, features = ["rocketchat"] }
## ...
```
