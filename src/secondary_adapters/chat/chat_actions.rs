use anyhow::{anyhow, Result as AnyResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config;

pub struct ChatActions;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelMessage {
    id: String,
}

impl ChatActions {
    pub async fn update_message(message_id: String, message: String) -> AnyResult<String> {
        let request_url = format!(
            "https://discord.com/api/channels/839488059253719080/messages/{}",
            message_id
        );

        surf::post(request_url)
            .body(json!({
                "content": message,
            }))
            .header("Authorization", config::get_header())
            .recv_json::<ChannelMessage>()
            .await
            .map(|res| res.id)
            .map_err(|err| anyhow!("Error while trying to send api message: {}", err))
    }

    pub async fn send_message(message: String) -> AnyResult<String> {
        let request_url = format!("https://discord.com/api/channels/839488059253719080/messages");

        surf::post(request_url)
            .body(json!({
                "content": message,
            }))
            .header("Authorization", config::get_header())
            .recv_json::<ChannelMessage>()
            .await
            .map(|res| res.id)
            .map_err(|err| anyhow!("Error while trying to send api message: {}", err))
    }

    pub async fn delete_all_messages() {
        let request_url = "https://discord.com/api/channels/839488059253719080/messages";

        let message_list: Vec<ChannelMessage> = surf::get(request_url)
            .header("Authorization", config::get_header())
            .recv_json()
            .await
            .unwrap();

        println!("{:?}", message_list);

        for message in message_list {
            ChatActions::delete_message(message).await;
        }
    }

    pub async fn delete_message(message: ChannelMessage) {
        let request_url = format!(
            "https://discord.com/api/channels/839488059253719080/messages/{}",
            message.id
        );

        if let Err(e) = surf::delete(request_url)
            .header("Authorization", config::get_header())
            .await
        {
            eprintln!("Couldn't send a request to the discord api: {}", e);
        }
    }
}
