#[cfg(test)]
mod tests {
    use super::super::message::{Embed, Message};
    use super::super::Discord;
    use crate::message::Message as CrateMessage;
    use crate::service::Service;

    #[tokio::test]
    async fn test_discord_crate_message() {
        let announce = crate::Announce::new().await.unwrap();

        //set message
        let msg = CrateMessage::new("dies ist ein test");
        //or use any other type of CrateMessage

        //get url form evironment
        let url = std::env::var("DISCORD_URL").expect("environment variable URL needs to be set");
        assert_ne!(url, "", "environment variable URL is empty");
        let url = url::Url::parse(&url).expect("given URL is in the wrong format");

        let response = Discord::notify(&announce, &url, &msg).await.unwrap();
        dbg!(&response);
    }

    #[tokio::test]
    async fn test_discord_message() {
        let client = reqwest::Client::new();

        //build message
        let mut msg = Message::default();
        msg.content = Some("test");
        msg.username = Some("new_name");
        let mut embed = Embed::default();
        embed.title = Some("embed title");
        embed.description = Some("embed description");
        embed.url = Some("https://google.com/");
        // embed.color = Some(40);
        msg.embeds.push(embed);

        dbg!(&serde_json::to_string(&msg));

        //get url from environment
        let url = std::env::var("DISCORD_URL").expect("environment variable URL needs to be set");
        assert_ne!(url, "", "environment variable URL is empty");
        let url = url::Url::parse(&url).expect("given URL is in the wrong format");

        //send
        let status = Discord::announce(&client, &url, &msg).await;
        dbg!(&status);
        let _ = status.unwrap();
    }
}
