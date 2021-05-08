use serde::{Deserialize, Serialize};
use serde_json::json;
use anyhow::{Result as AnyResult, anyhow};

use crate::{
    config,
    controllers::{self, MessageHandle},
};

#[derive(Debug, Deserialize, Serialize)]
struct ChannelMessage {
    id: String,
}

pub async fn startup_actions() {
    delete_all_messages().await;
    send_start_messages().await;
}

async fn delete_all_messages() {
    let request_url = "https://discord.com/api/channels/839488059253719080/messages";

    let message_list: Vec<ChannelMessage> = surf::get(request_url)
        .header("Authorization", config::get_header())
        .recv_json()
        .await
        .unwrap();

    println!("{:?}", message_list);

    for message in message_list {
        delete_message(message).await;
    }
}

async fn delete_message(message: ChannelMessage) {
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

async fn send_start_messages() {

    let message_input_list = controllers::Startup::get_startup_messages();

    for message_input in message_input_list {
        let message_id = send_message(message_input.message).await.unwrap();

        controllers::Startup::save_message_handle(MessageHandle {
            handle: message_input.handle,
            message_id,
        });
    }
}

async fn update_message(message_id: String, message: String) -> AnyResult<String> {
    let request_url = format!("https://discord.com/api/channels/839488059253719080/messages/{}", message_id);
    
    surf::post(request_url)
        .body(json!({
            "content": message,
        }))
        .header("Authorization", config::get_header())
        .recv_json::<ChannelMessage>()
        .await
        .map(|res|res.id)
        .map_err(|err| anyhow!("Error while trying to send api message: {}", err))
}

async fn send_message(message: String) -> AnyResult<String> {
    let request_url = format!("https://discord.com/api/channels/839488059253719080/messages");
    
    surf::post(request_url)
        .body(json!({
            "content": message,
        }))
        .header("Authorization", config::get_header())
        .recv_json::<ChannelMessage>()
        .await
        .map(|res|res.id)
        .map_err(|err| anyhow!("Error while trying to send api message: {}", err))
}
