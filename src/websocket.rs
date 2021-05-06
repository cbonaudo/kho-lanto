use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{thread, time};

struct Token;
impl TypeMapKey for Token {
    type Value = String;
}

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.name != "bronnie-degniart" && msg.channel_id.as_u64() == &(839488059253719080 as u64) {
            println!("{}", msg.author.name);
            delete_message(&ctx, &msg);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

pub fn start_websocket(token: String) {
    let mut client = Client::new(token.clone(), Handler).expect("Err creating client");
    let five_minutes = time::Duration::from_millis(300000);

    {
        let mut data = client.data.write();
        data.insert::<Token>(token);
    }

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
