use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config, controllers, models::camp::camp_getters::CampGetters};

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
    let request_url = "https://discord.com/api/channels/839488059253719080/messages".to_string();

    let message_list = controllers::Startup::get_startup_messages();

    for message in message_list {
        send_message(request_url.clone(), message).await;
    }
}

async fn send_message(_request_url: String, message: String) {
    println!("{}", message);
    
    // if let Err(e) = surf::post(request_url)
    //     .body(json!({
    //         "content": message,
    //     }))
    //     .header("Authorization", config::get_header())
    //     .await
    // {
    //     eprintln!("Couldn't send a request to the discord api: {}", e);
    // }
}
