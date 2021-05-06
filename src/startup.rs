use async_std::task;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::env;

#[derive(Debug, Deserialize, Serialize)]
struct ChannelMessage {
    id: String,
}

pub fn startup_actions() {
    let task = task::spawn(async {
        delete_all_messages().await;
        send_start_messages().await;
    });

    task::block_on(task);
}

async fn delete_all_messages() {
    let request_url = "https://discord.com/api/channels/839488059253719080/messages";

    let message_list: Vec<ChannelMessage> = surf::get(request_url)
        .header("Authorization", env::get_header())
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
        .header("Authorization", env::get_header())
        .await
    {
        eprintln!("Couldn't send a request to the discord api: {}", e);
    }
}

async fn send_start_messages() {
    let request_url = "https://discord.com/api/channels/839488059253719080/messages".to_string();

    let hello_message = "Bonjour, c'est Bronnie Degniart et Bienvenue pour cette première saison de Kho Lanto !".to_string();
    send_message(request_url.clone(), hello_message).await;

    let break_message = "------".to_string();
    send_message(request_url.clone(), break_message).await;

    let white_camp_message = "Campement de la tribu blanche :".to_string();
    send_message(request_url.clone(), white_camp_message).await;

    let fire_message = "Bois: 0 buches".to_string();
    send_message(request_url.clone(), fire_message).await;

    // let fire_message = "Feu: |========  |".to_string();
    // send_message(request_url, fire_message).await;

}

async fn send_message(request_url: String, message: String) {
    if let Err(e) = surf::post(request_url)
        .body(json!({
            "content": message,
        }))
        .header("Authorization", env::get_header())
        .await
    {
        eprintln!("Couldn't send a request to the discord api: {}", e);
    }
}
