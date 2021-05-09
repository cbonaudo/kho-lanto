use serde::{Deserialize, Serialize};

use crate::secondary_adapters::{
    camp::CampGetters,
    chat::{ChatActions, ChatGetters},
};

#[derive(Clone)]
pub struct MessageInput {
    pub message: String,
    pub handle: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MessageHandle {
    pub handle: String,
    pub message_id: String,
}

pub struct Startup;

impl Startup {
    pub async fn delete_all_messages() {
        ChatActions::delete_all_messages().await;
    }

    pub async fn send_start_messages() {
        let mut message_list: Vec<MessageInput> = vec![];

        let hello_message = "Bonjour, c'est Bronnie Degniart et Bienvenue pour cette première saison de Kho Lanto !".to_string();
        message_list.push(MessageInput {
            message: hello_message,
            handle: "hello".to_string(),
        });

        let break_message = "------".to_string();
        message_list.push(MessageInput {
            message: break_message,
            handle: "break".to_string(),
        });

        let white_camp_message = "Campement de la tribu blanche :".to_string();
        message_list.push(MessageInput {
            message: white_camp_message,
            handle: "white_camp".to_string(),
        });

        let wood_message = format!("Bois: {} buches", CampGetters::get_wood_amount());
        message_list.push(MessageInput {
            message: wood_message,
            handle: "wood".to_string(),
        });

        if CampGetters::is_fire_tryable() {
            let fire_message = "Feu: |========  |".to_string();
            message_list.push(MessageInput {
                message: fire_message,
                handle: "fire".to_string(),
            });
        }

        for message_input in message_list {
            let message_input_clone = message_input.clone();
            
            match ChatGetters::get_handle(message_input.handle) {
                Some(message_handle) => {
                    ChatActions::update_message(message_handle.message_id, message_input.message)
                        .await
                }
                None => ChatActions::send_message(message_input_clone).await,
            }
        }
    }
}
