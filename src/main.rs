extern crate config;

mod env;
mod schedule;
mod websocket;

use std::thread;

fn main() {
    let token = env::get_data();

    let token_clone = token.clone();

    // let schedule_thread = thread::spawn(|| schedule::start_scheduler(token_clone));

    let websocket_thread = thread::spawn(|| websocket::start_websocket(token));

    // let _ = schedule_thread.join();
    let _ = websocket_thread.join();
}
