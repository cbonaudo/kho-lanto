use crate::models::camp::camp_getters::CampGetters;


pub struct Startup;

impl Startup {
    pub fn get_startup_messages() -> Vec<String> {
        let mut message_list: Vec<String> = vec![];

        let hello_message = "Bonjour, c'est Bronnie Degniart et Bienvenue pour cette première saison de Kho Lanto !".to_string();  
        message_list.push(hello_message);

        let break_message = "------".to_string();
        message_list.push(break_message);

        let white_camp_message = "Campement de la tribu blanche :".to_string();
        message_list.push(white_camp_message);

        let wood_message = format!("Bois: {} buches", CampGetters::get_wood_amount());
        message_list.push(wood_message);

        if CampGetters::is_fire_tryable() {
            let fire_message = "Feu: |========  |".to_string();
            message_list.push(fire_message);
        }

        message_list
    }
}

