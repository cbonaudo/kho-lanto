use serde::{Deserialize, Serialize};

use crate::{
    domain::SaveFile,
    secondary_adapters::{
        adventurers,
        camp_resources::CampGetters,
        chat::{ChatActions, ChatGetters},
    },
};

use super::dto::Occupation;

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

pub struct Camp;

impl Camp {
    pub async fn delete_all_messages() {
        ChatActions::delete_all_messages().await;
    }

    pub async fn display_interface() {
        let mut message_list: Vec<MessageInput> = vec![];

        let hello_message = "Bonjour, c'est Bronnie Degniart et Bienvenue pour cette première saison de Kho Lanto !".to_string();
        message_list.push(MessageInput {
            message: hello_message,
            handle: "hello".to_string(),
        });

        let white_camp_message = "Campement de la tribu blanche :".to_string();
        message_list.push(MessageInput {
            message: white_camp_message,
            handle: "white_camp".to_string(),
        });

        let wood_message = format!("Bois: {} branches", CampGetters::get_wood_amount());
        message_list.push(MessageInput {
            message: wood_message,
            handle: "wood".to_string(),
        });

        if let Some(fire_message) = get_fire_message() {
            message_list.push(fire_message);
        }

        // Exploration de l'ile -> petit % de trouver outil, immunité, etc...

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

        // TODO: remove after testing phase
        SaveFile::save_states();
    }

    pub fn assign(adventurer_name: String, occupation: Occupation) {
        dbg!(&adventurer_name);
        dbg!(&occupation);
        adventurers::Actions::assign(adventurer_name, occupation);
    }
}

pub fn get_fire_message() -> Option<MessageInput> {
    match CampGetters::get_wood_amount() > 9 {
        true => {
            let fire_message = "Feu: |========  |".to_string();
            return Some(MessageInput {
                message: fire_message,
                handle: "fire".to_string(),
            });
        }
        false => return None,
    }
}
