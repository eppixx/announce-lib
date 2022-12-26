#[cfg(test)]
mod tests {
    use super::super::message::{Attachment, Message};
    use super::super::{RocketChat, Upload};
    use crate::message::Message as CrateMessage;
    use crate::service::Service;

    #[tokio::test]
    async fn test_msg() {
        let announce = crate::Announce::new().unwrap();

        //set message
        let msg = CrateMessage::Text("dies ist ein test");
        //or use any other type of CrateMessage

        //get url from environment
        let url = std::env::var("ROCKET_URL").expect("environment variable URL needs to be set");
        assert_ne!(url, "", "environment variable URL is empty");
        let url = url::Url::parse(&url).expect("given URL is in the wrong format");

        let req = RocketChat::build_request(&announce, &url, &msg);
        let req = match req.unwrap() {
            crate::service::ServiceResult::Reqwest(req) => req,
            _ => panic!("expected a reqwest"),
        };
        let response = announce.client.execute(req).await.unwrap();
        dbg!(&response);
    }

    #[tokio::test]
    async fn test_message() {
        let client = reqwest::Client::new();

        //build message
        let mut msg = Message::new("testi");
        msg.text = Some("testitesttest");
        msg.avatar = Some("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fcdn1.vectorstock.com%2Fi%2F1000x1000%2F31%2F95%2Fuser-sign-icon-person-symbol-human-avatar-vector-12693195.jpg&f=1&nofb=1&ipt=36bf2ef7570e41608a9f86b6a2f9e0456859e467c7fcbfe2535004eb341c517c&ipo=images");
        let mut attachment = Attachment::default();
        attachment.color = Some("#ff0000");
        attachment.text = Some("dsfdsf");
        attachment.ts = Some(chrono::offset::Utc::now() + chrono::Duration::seconds(99999999999));
        attachment.thumb_url = Some("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fcdn1.vectorstock.com%2Fi%2F1000x1000%2F31%2F95%2Fuser-sign-icon-person-symbol-human-avatar-vector-12693195.jpg&f=1&nofb=1&ipt=36bf2ef7570e41608a9f86b6a2f9e0456859e467c7fcbfe2535004eb341c517c&ipo=images");
        attachment.message_link = Some("https://google.com");
        attachment.title = Some("Title");
        attachment.title_link = Some("https://youtube.com/");
        attachment.title_link_download = false;
        attachment.collapsed = true;
        msg.attachments.push(attachment);

        dbg!(&serde_json::to_string(&msg));

        //get url from environment
        let url = std::env::var("ROCKET_URL").expect("environment variable URL needs to be set");
        assert_ne!(url, "", "environment variable URL is empty");
        let url = url::Url::parse(&url).expect("given URL is in the wrong format");

        //send
        let status = RocketChat::announce(&client, &url, &msg).await;
        dbg!(&status);
        let _ = status.unwrap();
    }

    #[tokio::test]
    async fn send_file() {
        let client = reqwest::Client::new();

        //build upload
        let upload = Upload {
            description: "a sample description",
            message: "a sample message",
            file_path: "./sample_uploads/License.md",
            // file_path: "./sample_uploads/rustacean-flat-happy.svg",
            // file_path: "./sample_uploads/rustacean-flat-happy.png",
        };

        let url = std::env::var("ROCKET_URL").expect("environment variable URL needs to be set");
        let url = url::Url::parse(&url).expect("given URL is missformed or empty");
        let response = RocketChat::upload(&client, &url, &upload).await;
        println!("response: {:?}", response.unwrap().text().await);
    }
}
