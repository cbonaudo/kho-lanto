use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{thread, time};

use crate::{
    config,
    domain::{dto::Occupation, Camp},
};

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name != "bronnie-degniart"
            && msg.channel_id.as_u64() == &(839488059253719080 as u64)
        {
            println!("{}: {}", msg.author.name, msg.content);

            delete_message(&ctx, &msg);

            if msg.content == "bois" {
                Camp::assign(msg.author.name, Occupation::WOOD);
            }
        }
    }
}

pub fn start_websocket() {
    let mut client =
        Client::new(config::CONFIG.token.to_string(), Handler).expect("Err creating client");
    let five_minutes = time::Duration::from_millis(300000);

    loop {
        if let Err(why) = client.start() {
            println!("Client error: {:?}", why);
        }
        thread::sleep(five_minutes);
    }
}

fn delete_message(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.delete_message(&ctx.http, msg.id) {
        println!("Error deleting message: {:?}", why);
    }
}
